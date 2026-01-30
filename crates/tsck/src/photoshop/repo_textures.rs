#![allow(unused)]
use crate::{
    AppConfigHandler, dp, log_debug, log_error, query, query_page,
    store::{DbStore, PageChunk, Texture, TextureScanner, config},
    transaction,
};
use anyhow::{Ok, Result};
use rusqlite::ToSql;
pub struct TextureRepo {
    db: DbStore,
}
impl TextureRepo {
    pub fn new(db: DbStore) -> Self {
        TextureRepo::check_texture_db(&db);
        Self { db }
    }
    fn check_texture_db(db: &DbStore) -> anyhow::Result<()> {
        if let Err(_) = db.read(|db| query!(db, Texture, "SELECT * from textures")) {
            TextureRepo::create_table()?;
        }

        Ok(())
    }
    fn create_table() -> Result<()> {
        let root = AppConfigHandler::get().get(|c| c.store_root.clone());
        let texture_path = std::path::Path::new(&root).join("texture");
        let mut db = DbStore::new()?;
        db.write(|db| -> anyhow::Result<()> {
            db.create_table::<Texture>()?;
            let entries = TextureScanner::scan(&texture_path)?;
            transaction!(db, "textures", &entries)?;
            Ok(())
        });
        Ok(())
    }

    pub fn get_all_categories(&self) -> Result<Vec<String>> {
        let result = self.db.read(|db| {
            db.query(
                r#"SELECT DISTINCT category
        FROM textures
        WHERE category IS NOT NULL
        AND category != ''
        ORDER BY category"#,
                &[],
                |row| Ok(row.get::<_, String>("category")?),
            )
        });
        log_error!(dp!(result));
        Ok(Vec::new())
    }
    pub fn get_textures_chunk(&self, page: usize, limit: usize) -> Result<PageChunk<Texture>> {
        self.db
            .read(|db| db.query_page_models::<Texture>("SELECT * FROM textures", &[], page, limit))
    }
    pub fn get_favorite_chunk(&self, page: usize, limit: usize) -> Result<PageChunk<Texture>> {
        self.db.read(|db| {
            db.query_page_models::<Texture>(
                "SELECT * FROM textures WHERE favorite = 1",
                &[],
                page,
                limit,
            )
        })
    }
    pub fn get_textures_chunk_by_category(
        &self,
        category: String,
        page: usize,
        limit: usize,
    ) -> Result<PageChunk<Texture>> {
        self.db.read(|db| {
            query_page!(
                db,
                Texture,
                "SELECT * FROM textures WHERE category = ?",
                page,
                limit,
                &category
            )
            // db.query_page_models::<Texture>(
            //     "SELECT * FROM textures WHERE category = ?",
            //     &[&category],
            //     page,
            //     limit,
            // )
        })
    }

    pub fn get_all(&self) -> Result<Vec<Texture>> {
        self.db
            .read(|db| query!(db, Texture, "SELECT * FROM textures"))
    }
    pub fn get_by_category(&self, category: &str) -> Result<Vec<Texture>> {
        self.db.read(|db| {
            query!(
                db,
                Texture,
                "SELECT * FROM textures WHERE category = ?",
                category
            )
        })
    }

    pub fn get_favorites(&self) -> Result<Vec<Texture>> {
        self.db
            .read(|db| query!(db, Texture, "SELECT * FROM textures WHERE favorite = 1"))
    }

    pub fn set_favorite(&self, id: i32, favorite: bool) -> Result<()> {
        let fav = if favorite { 1 } else { 0 };
        self.db.write(|db| {
            db.execute(
                "UPDATE textures SET favorite = ? WHERE id = ?",
                &[&fav as &dyn ToSql, &id as &dyn ToSql],
            );
        });
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.db.write(|db| {
            if let Err(err) = db.execute("DELETE FROM textures", []) {
                log_error!("CLEAR ALL", dp!(err));
            }
        });
        Ok(())
    }

    pub fn batch_insert(&self, textures: &[Texture]) -> Result<()> {
        self.db.write(|db| transaction!(db, "textures", textures))
    }
}

#[cfg(test)]
mod repo_textures {
    use crate::{
        AppConfigHandler, dp, log_debug, log_error,
        photoshop::repo_textures::TextureRepo,
        store::{DbStore, Texture, TextureScanner},
        transaction,
    };

    #[test]
    fn read_db() -> anyhow::Result<()> {
        let repo = TextureRepo::new(DbStore::new()?);
        let cat = repo.get_all_categories()?;
        let chunck = repo.get_textures_chunk(0, 30)?;
        log_debug!("Textures", dp!(chunck.total_pages));
        let chunck = repo.get_favorite_chunk(0, 30)?;
        log_debug!("Favorite", dp!(chunck.items.len()));
        let chunck = repo.get_textures_chunk_by_category("Water".to_string(), 0, 30)?;
        log_debug!("By Category", dp!(chunck.total_pages));
        Ok(())
    }
}
