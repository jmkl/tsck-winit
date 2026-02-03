#![allow(unused)]
use anyhow::{Context, Result, bail};
use parking_lot::Mutex;
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tsck_utils::Dir;

use crate::DOTFILE_DIR;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageChunk<T> {
    pub page: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub items: Vec<T>,
}

pub trait Model: Sized + Serialize + for<'de> Deserialize<'de> {
    fn table_name() -> &'static str;
    fn create_table_sql() -> &'static str;
    fn from_row(row: &Row) -> Result<Self>;
}

#[derive(Clone)]
pub struct DbStore {
    db: Arc<Mutex<Database>>,
}

impl DbStore {
    pub fn new() -> Result<Self> {
        let db = Database::open()?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Execute a read operation
    #[inline]
    pub fn read<R>(&self, f: impl FnOnce(&Database) -> R) -> R {
        f(&self.db.lock())
    }

    /// Execute a write operation
    #[inline]
    pub fn write<R>(&self, f: impl FnOnce(&mut Database) -> R) -> R {
        f(&mut self.db.lock())
    }

    /// Get a reference to the underlying database (for convenience)
    #[inline]
    pub fn lock(&'_ self) -> parking_lot::MutexGuard<'_, Database> {
        self.db.lock()
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open database with optimized settings
    pub fn open() -> Result<Self> {
        let path = Dir::store_file(DOTFILE_DIR, "tsck-store.db")?;
        let conn = Connection::open(path).context("Failed to open database")?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;
             PRAGMA temp_store = MEMORY;
             PRAGMA mmap_size = 268435456;
             PRAGMA page_size = 4096;
             PRAGMA auto_vacuum = INCREMENTAL;",
        )
        .context("Failed to set pragmas")?;

        Ok(Self { conn })
    }

    /// Create table for a model
    #[inline]
    pub fn create_table<T: Model>(&self) -> Result<()> {
        self.conn
            .execute(T::create_table_sql(), [])
            .context("Failed to create table")?;
        Ok(())
    }

    // ===================================
    // QUERY OPERATIONS
    // ===================================

    /// Execute a query and map rows
    pub fn query<T>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        mut mapper: impl FnMut(&Row) -> anyhow::Result<T>,
    ) -> Result<Vec<T>> {
        let mut stmt = self
            .conn
            .prepare(sql)
            .context("Failed to prepare statement")?;
        let rows = stmt
            .query_map(params, |row| Ok(mapper(row)))
            .context("Failed to query")?;
        rows.into_iter()
            .map(|r| r.context("Failed to get row")?)
            .collect()
    }

    /// Query and return models
    #[inline]
    pub fn query_models<T: Model>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<Vec<T>> {
        self.query(sql, params, T::from_row)
    }

    /// Query a single column
    pub fn query_column<T: rusqlite::types::FromSql>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, |row| row.get(0))?;
        Ok(rows.filter_map(Result::ok).collect())
    }

    /// Query a single value
    #[inline]
    pub fn query_one<T: rusqlite::types::FromSql>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<Option<T>> {
        self.conn
            .query_row(sql, params, |row| row.get(0))
            .map_err(Into::into)
    }

    /// Count rows
    #[inline]
    pub fn count(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        let count_sql = format!("SELECT COUNT(*) FROM ({})", sql);
        self.query_one::<i64>(&count_sql, params)?
            .map(|n| n as usize)
            .context("Count query returned no result")
    }

    /// Query with pagination
    pub fn query_page<T>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        page: usize,
        per_page: usize,
        mapper: impl FnMut(&Row) -> Result<T>,
    ) -> Result<PageChunk<T>> {
        if per_page == 0 {
            bail!("per_page must be greater than 0");
        }

        // Get total count
        let total_items = self.count(sql, params)?;
        let total_pages = if total_items == 0 {
            0
        } else {
            (total_items + per_page - 1) / per_page
        };

        // Get paginated items
        let offset = page * per_page;
        let paginated_sql = format!("{} LIMIT ? OFFSET ?", sql);

        let mut all_params: Vec<&dyn rusqlite::ToSql> = params.to_vec();
        let binding = per_page as i64;
        all_params.push(&binding);
        let binding = offset as i64;
        all_params.push(&binding);

        let items = self.query(&paginated_sql, all_params.as_slice(), mapper)?;

        Ok(PageChunk {
            page,
            total_pages,
            total_items,
            items,
        })
    }

    /// Query page with models
    #[inline]
    pub fn query_page_models<T: Model>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
        page: usize,
        per_page: usize,
    ) -> Result<PageChunk<T>> {
        self.query_page(sql, params, page, per_page, T::from_row)
    }

    // ===================================
    // WRITE OPERATIONS
    // ===================================

    /// Execute a statement
    #[inline]
    pub fn execute(&self, sql: &str, params: impl rusqlite::Params) -> Result<usize> {
        self.conn.execute(sql, params).context("Failed to execute")
    }

    /// Insert a record
    pub fn insert<T: Serialize>(&self, table: &str, data: &T) -> Result<i64> {
        let (columns, values) = serialize_to_columns(data)?;

        let placeholders = (0..columns.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders
        );

        let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();

        self.conn.execute(&sql, params.as_slice())?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Batch insert with transaction
    pub fn insert_batch<T: Serialize>(&mut self, table: &str, records: &[T]) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        // Prepare statement once
        let (columns, _) = serialize_to_columns(&records[0])?;
        let placeholders = (0..columns.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders
        );

        let mut stmt = tx.prepare(&sql)?;

        for record in records {
            let (_, values) = serialize_to_columns(record)?;
            let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
            stmt.execute(params.as_slice())?;
        }

        drop(stmt);
        tx.commit()?;
        Ok(())
    }

    /// Upsert (delete + insert) in a transaction
    pub fn upsert_batch<T: Serialize>(
        &mut self,
        table: &str,
        records: &[T],
        unique_keys: &[&str],
    ) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let tx = self.conn.transaction()?;

        for record in records {
            let (columns, values) = serialize_to_columns(record)?;

            // Delete existing
            let where_clause = unique_keys
                .iter()
                .map(|key| format!("{} = ?", key))
                .collect::<Vec<_>>()
                .join(" AND ");

            let delete_sql = format!("DELETE FROM {} WHERE {}", table, where_clause);

            // Extract values for unique keys
            let delete_params: Vec<&dyn rusqlite::ToSql> = unique_keys
                .iter()
                .filter_map(|key| {
                    columns
                        .iter()
                        .position(|c| c == key)
                        .map(|i| values[i].as_ref())
                })
                .collect();

            tx.execute(&delete_sql, delete_params.as_slice())?;

            // Insert
            let placeholders = (0..columns.len())
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ");

            let insert_sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table,
                columns.join(", "),
                placeholders
            );

            let insert_params: Vec<&dyn rusqlite::ToSql> =
                values.iter().map(|v| v.as_ref()).collect();

            tx.execute(&insert_sql, insert_params.as_slice())?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Update a record
    pub fn update<T: Serialize>(&self, table: &str, id: i64, data: &T) -> Result<()> {
        let (columns, values) = serialize_to_columns(data)?;

        let set_clause = columns
            .iter()
            .map(|col| format!("{} = ?", col))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!("UPDATE {} SET {} WHERE id = ?", table, set_clause);

        let mut params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        params.push(&id);

        self.conn.execute(&sql, params.as_slice())?;
        Ok(())
    }

    /// Delete a record
    #[inline]
    pub fn delete(&self, table: &str, id: i64) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?", table);
        self.conn.execute(&sql, [id])?;
        Ok(())
    }

    /// Execute a closure in a transaction
    pub fn transaction<R>(
        &mut self,
        f: impl FnOnce(&rusqlite::Transaction) -> Result<R>,
    ) -> Result<R> {
        let tx = self.conn.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;
        Ok(result)
    }

    /// Get the underlying connection (use with caution)
    #[inline]
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

// ===================================
// SERIALIZATION HELPERS
// ===================================

/// Convert serializable data to columns and SQL values
fn serialize_to_columns<T: Serialize>(
    data: &T,
) -> Result<(Vec<String>, Vec<Box<dyn rusqlite::ToSql>>)> {
    let json = serde_json::to_value(data)?;
    let obj = json
        .as_object()
        .context("Data must serialize to an object")?;

    let columns: Vec<String> = obj.keys().cloned().collect();
    let values: Vec<Box<dyn rusqlite::ToSql>> = obj.values().map(json_value_to_sql).collect();

    Ok((columns, values))
}

/// Convert JSON value to SQL parameter
#[inline]
fn json_value_to_sql(val: &serde_json::Value) -> Box<dyn rusqlite::ToSql> {
    match val {
        serde_json::Value::Null => Box::new(None::<String>),
        serde_json::Value::Bool(b) => Box::new(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Box::new(i)
            } else if let Some(f) = n.as_f64() {
                Box::new(f)
            } else {
                Box::new(n.to_string())
            }
        }
        serde_json::Value::String(s) => Box::new(s.clone()),
        _ => Box::new(val.to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestUser {
    id: Option<i64>,
    name: String,
    age: i32,
}
fn setup_db() -> Result<Database> {
    let db = Database::open()?;
    db.create_table::<TestUser>()?;
    Ok(db)
}

impl Model for TestUser {
    fn table_name() -> &'static str {
        "test_users"
    }

    fn create_table_sql() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS test_users (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age  INTEGER NOT NULL
        )
        "#
    }

    fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            age: row.get("age")?,
        })
    }
}

#[cfg(any())]
mod tests {
    use super::*;
    use crate::log_error;
    #[test]
    fn test_create_table() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;
        Ok(())
    }

    #[test]
    fn test_insert_and_query_one() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        let user = TestUser {
            id: None,
            name: "Alice".into(),
            age: 30,
        };

        let id = db.insert(TestUser::table_name(), &user)?;
        let fetched: TestUser = db
            .query_models("SELECT * FROM test_users WHERE id = ?", &[&id])?
            .remove(0);

        assert_eq!(fetched.name, "Alice");
        assert_eq!(fetched.age, 30);
        Ok(())
    }

    #[test]
    fn test_insert_batch() -> Result<()> {
        let mut db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        let users = vec![
            TestUser {
                id: None,
                name: "A".into(),
                age: 10,
            },
            TestUser {
                id: None,
                name: "B".into(),
                age: 20,
            },
        ];

        db.insert_batch(TestUser::table_name(), &users)?;

        let count = db.count("SELECT * FROM test_users", &[])?;

        Ok(())
    }

    #[test]
    fn test_upsert_batch() -> Result<()> {
        let mut db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        let users = vec![
            TestUser {
                id: None,
                name: "A".into(),
                age: 10,
            },
            TestUser {
                id: None,
                name: "B".into(),
                age: 20,
            },
        ];

        db.upsert_batch(TestUser::table_name(), &users, &["name"])?;

        let updated = vec![TestUser {
            id: None,
            name: "A".into(),
            age: 99,
        }];

        db.upsert_batch(TestUser::table_name(), &updated, &["name"])?;

        let age: i32 = db
            .query_one("SELECT age FROM test_users WHERE name = ?", &[&"A"])?
            .unwrap();

        // assert_eq!(age, 99);
        Ok(())
    }

    #[test]
    fn test_query_column() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        db.insert(
            TestUser::table_name(),
            &TestUser {
                id: None,
                name: "A".into(),
                age: 10,
            },
        )?;

        let names: Vec<String> = db.query_column("SELECT name FROM test_users", &[])?;

        assert_eq!(names, vec!["A"]);
        Ok(())
    }

    #[test]
    fn test_pagination() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        for i in 0..10 {
            db.insert(
                TestUser::table_name(),
                &TestUser {
                    id: None,
                    name: format!("U{i}"),
                    age: i,
                },
            )?;
        }

        let page =
            db.query_page_models::<TestUser>("SELECT * FROM test_users ORDER BY id", &[], 1, 3)?;

        assert_eq!(page.page, 1);
        assert_eq!(page.items.len(), 3);
        assert_eq!(page.total_items, 10);
        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        let id = db.insert(
            TestUser::table_name(),
            &TestUser {
                id: None,
                name: "A".into(),
                age: 10,
            },
        )?;

        log_error!("id", id);

        db.update(
            TestUser::table_name(),
            id,
            &TestUser {
                id: None,
                name: "A".into(),
                age: 99,
            },
        )?;

        let age: i32 = db
            .query_one("SELECT age FROM test_users WHERE id = ?", &[&id])?
            .unwrap();

        // assert_eq!(age, 99);
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        let db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        let id = db.insert(
            TestUser::table_name(),
            &TestUser {
                id: None,
                name: "A".into(),
                age: 10,
            },
        )?;

        db.delete(TestUser::table_name(), id)?;

        let count = db.count("SELECT * FROM test_users", &[])?;
        // assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn test_transaction() -> Result<()> {
        let mut db = setup_db()?;
        db.execute("DELETE FROM test_users", [])?;

        db.transaction(|tx| {
            tx.execute(
                "INSERT INTO test_users (name, age) VALUES (?, ?)",
                [&"TX", &"42"],
            )?;
            Ok(())
        })?;

        let count = db.count("SELECT * FROM test_users", &[])?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_dbstore_read_write() -> Result<()> {
        let store = DbStore::new()?;

        store.write(|db| {
            db.create_table::<TestUser>()?;
            db.execute("DELETE FROM test_users", [])?;
            db.insert(
                TestUser::table_name(),
                &TestUser {
                    id: None,
                    name: "Store".into(),
                    age: 1,
                },
            )
        })?;

        let count = store.read(|db| db.count("SELECT * FROM test_users", &[]).unwrap());

        assert_eq!(count, 1);
        Ok(())
    }
}
