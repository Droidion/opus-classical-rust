use crate::configuration::DatabaseSettings;
use crate::domain::label::Label;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use sqlx::PgPool;
use sqlx::Row;

pub struct Database {
    pub pg_pool: PgPool,
}

/// Creates new connection pool.
pub fn get_connection_pool(db_config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_config.get_connection_options())
}

impl Database {
    /// Retrieves a collection of items from database.
    async fn get_many<T: Unpin + Send>(&self, sql: &str, func: fn(PgRow) -> T) -> Vec<T> {
        sqlx::query(sql)
            .map(func)
            .fetch_all(&self.pg_pool)
            .await
            .unwrap()
    }

    /// Returns all labels.
    pub async fn get_labels(&self) -> Vec<Label> {
        let sql = "SELECT id, name FROM labels ORDER BY name";
        let mapper = |row: PgRow| Label {
            id: row.get("id"),
            name: row.get("name"),
        };
        self.get_many(sql, mapper).await
    }
}
