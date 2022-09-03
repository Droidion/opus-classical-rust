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
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

impl Database {
    /// Retrieves a collection of items from database.
    async fn get_many<T: Unpin + Send>(&self, sql: &str, func: fn(PgRow) -> T) -> Vec<T> {
        let results: Vec<T> = sqlx::query(sql)
            .map(func)
            .fetch_all(&self.pg_pool)
            .await
            .unwrap();
        results
    }

    /// Returns all labels.
    pub async fn get_labels(&self) -> Vec<Label> {
        self.get_many("SELECT id, name FROM labels ORDER BY name", |row: PgRow| {
            Label {
                id: row.get("id"),
                name: row.get("name"),
            }
        })
        .await
    }
}
