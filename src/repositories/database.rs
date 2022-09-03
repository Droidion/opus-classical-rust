use crate::configuration::DatabaseSettings;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::{PgRow};
use sqlx::{Row};
use crate::domain::label::Label;

pub struct Database {
    pub pg_pool: PgPool
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

impl Database {
    pub async fn get_labels(&self) -> Vec<Label> {
        let labels: Vec<Label> = sqlx::query("SELECT id, name FROM labels ORDER BY name")
            .map(|row: PgRow| Label {
                id: row.get("id"),
                name: row.get("name")
            })
            .fetch_all(&self.pg_pool)
            .await
            .unwrap();
        labels
    }
}