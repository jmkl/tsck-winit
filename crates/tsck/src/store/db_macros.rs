/// Query macros for database operations
///
/// ## query! - Execute query and return typed models
/// ```ignore
/// // Single parameter
/// let user = query!(db, User, "SELECT * FROM users WHERE id = ?", 1)?;
///
/// // Multiple parameters
/// let users = query!(db, User, "SELECT * FROM users WHERE name = ? AND age > ?", "John", 18)?;
///
/// // No parameters
/// let all_users = query!(db, User, "SELECT * FROM users")?;
/// ```
#[macro_export]
macro_rules! query {
    ($db:expr, $model:ty, $sql:expr $(, $param:expr)*) => {{
        $db.query_models::<$model>($sql, &[$(&$param as &dyn ToSql),*])
    }};
}

/// query_chunk! - Execute paginated query with offset/limit
/// ```ignore
/// // Get page 2 with 10 items per page
/// let users = query_chunk!(db, User, "SELECT * FROM users", 2, 10)?;
///
/// // Paginated query with filters
/// let users = query_chunk!(
///     db, User,
///     "SELECT * FROM users WHERE status = ?",
///     1, 20,
///     "active"
/// )?;
///
/// // Page 0 is the first page
/// let first_page = query_chunk!(db, User, "SELECT * FROM users", 0, 25)?;
/// ```
///

#[macro_export]
macro_rules! query_page {
    ($db:expr, $model:ty, $sql:expr, $page:expr, $per_page:expr $(, $param:expr)*) => {{
        $db.query_page_models::<$model>($sql, &[$(&$param as &dyn ToSql),*], $page, $per_page)
    }};
}

/// query_raw! - Execute query with custom mapper function
/// ```ignore
/// // Simple row mapping
/// let data = query_raw!(
///     db,
///     "SELECT id, name FROM users WHERE age > ?",
///     |row| { (row.get::<_, i32>(0)?, row.get::<_, String>(1)?) },
///     21
/// )?;
///
/// // Complex mapping with multiple columns
/// let results = query_raw!(
///     db,
///     "SELECT id, name, email, created_at FROM users WHERE active = ?",
///     |row| {
///         Ok((
///             row.get::<_, i32>(0)?,
///             row.get::<_, String>(1)?,
///             row.get::<_, String>(2)?,
///             row.get::<_, String>(3)?
///         ))
///     },
///     true
/// )?;
///
/// // No parameters
/// let count = query_raw!(
///     db,
///     "SELECT COUNT(*) FROM users",
///     |row| row.get::<_, i32>(0)
/// )?;
/// ```
#[macro_export]
macro_rules! query_raw {
    ($db:expr, $sql:expr, $mapper:expr $(, $param:expr)*) => {{
        $db.query($sql, &[$(&$param as &dyn rusqlite::ToSql),*], $mapper)
    }};
}

/// insert! - Insert a single record
/// ```ignore
/// let user = User { id: 1, name: "John".to_string(), age: 30 };
/// insert!(db, "users", user)?;
///
/// // Or with reference
/// insert!(db, "messages", &message)?;
///
/// // Struct gets serialized to JSON automatically
/// let product = Product { id: 1, title: "Laptop", price: 999.99 };
/// insert!(db, "products", product)?;
/// ```
#[macro_export]
macro_rules! insert {
    ($db:expr, $table:expr, $data:expr) => {{ $db.insert($table, &$data) }};
}

/// transaction! - Batch insert multiple records in a transaction
/// ```ignore
/// let users = vec![
///     User { id: 1, name: "Alice".to_string(), age: 25 },
///     User { id: 2, name: "Bob".to_string(), age: 30 },
///     User { id: 3, name: "Charlie".to_string(), age: 35 },
/// ];
/// transaction!(db, "users", &users)?;
///
/// // Insert multiple messages atomically
/// transaction!(db, "messages", &messages)?;
///
/// // Large batch inserts are more efficient than individual inserts
/// let records = generate_many_records();
/// transaction!(db, "logs", &records)?;
/// ```
#[macro_export]
macro_rules! transaction {
    ($db:expr, $table:expr, $data:expr) => {{ $db.insert_batch($table, $data) }};
}

/// update! - Update a record by ID
/// ```ignore
/// // Update with modified struct
/// let mut user = User { id: 1, name: "Alice".to_string(), age: 26 };
/// update!(db, "users", 1, user)?;
///
/// // Update with reference
/// update!(db, "products", product.id, &product)?;
///
/// // Struct gets serialized to JSON and all fields updated
/// update!(db, "messages", message_id, &updated_message)?;
/// ```
#[macro_export]
macro_rules! update {
    ($db:expr, $table:expr, $id:expr, $data:expr) => {{ $db.update($table, $id, &$data) }};
}

/// delete! - Delete a record by ID
/// ```ignore
/// // Delete by integer ID
/// delete!(db, "users", 1)?;
///
/// // Delete by other types
/// delete!(db, "messages", message_id)?;
/// delete!(db, "sessions", session_uuid)?;
///
/// // Delete multiple by calling in a loop or use batch operations
/// for id in user_ids {
///     delete!(db, "users", id)?;
/// }
/// ```
#[macro_export]
macro_rules! delete {
    ($db:expr, $table:expr, $id:expr) => {{ $db.delete($table, $id) }};
}

/// upsert! - Insert or update records on conflict
/// Requires a UNIQUE INDEX or PRIMARY KEY on the specified columns.
///
/// ```ignore
/// // Single unique key
/// let users = vec![
///     User { id: 1, name: "Alice", age: 25 },
///     User { id: 2, name: "Bob", age: 30 },
/// ];
/// upsert!(db, "users", &users, "id")?;
///
/// // Multiple unique keys (composite key)
/// let messages = vec![
///     Message { id: 1, message_id: "msg_1", content: "Hello" },
///     Message { id: 2, message_id: "msg_2", content: "World" },
/// ];
/// upsert!(db, "messages", &messages, "id", "message_id")?;
///
/// // If records exist, all fields are updated to new values
/// // If records don't exist, they are inserted
/// upsert!(db, "products", &products, "sku")?;
/// ```
#[macro_export]
macro_rules! upsert {
    ($db:expr, $table:expr, $data:expr, $($key:expr),+) => {{
        $db.batch_upsert($table, $data, &[$($key),+])
    }};
}
