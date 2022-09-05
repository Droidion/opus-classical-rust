use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::configuration::DatabaseSettings;
use crate::domain::label::Label;
use crate::domain::period::Period;
use sqlx::postgres::{PgArguments, PgPoolOptions, types};
use sqlx::postgres::PgRow;
use sqlx::{Encode, PgPool, postgres, Postgres};
use sqlx::Row;
use sqlx::{types::Uuid, types::Json};
use sqlx::database::HasArguments;
use sqlx::query::Query;
use crate::domain::composer::Composer;
use crate::domain::genre::Genre;

static GET_LABELS_SQL: &str = "SELECT id, name FROM labels ORDER BY name";
static GET_PERIODS_SQL: &str = "select json from periods_composers";
static GET_COMPOSER: &str = "select composer_by_slug($1) as json";
static GET_GENRES: &str = "select genres_and_works_by_composer($1) as json";

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
    /// Extracts a collection of items from database and maps them to a vector of structs.
    async fn extract_values<T: Unpin + Send>(&self, sql: &str, func: fn(PgRow) -> T) -> anyhow::Result<Vec<T>> {
        let values: Vec<T> = sqlx::query(sql)
            .map(func)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(values)
    }

    /// Extracts JSON from database.
    async fn extract_json<T: DeserializeOwned>(&self, query: Query<'_, Postgres, PgArguments>) -> anyhow::Result<T> {
        let postgres_row = query.fetch_one(&self.pg_pool).await?;
        let json_value: serde_json::Value = postgres_row.get("json");
        let parsed_value: T = serde_json::from_value(json_value)?;
        Ok(parsed_value)
    }

    /// Returns all labels.
    pub async fn get_labels(&self) -> anyhow::Result<Vec<Label>> {
        let mapper = |row: PgRow| Label {
            id: row.get("id"),
            name: row.get("name"),
        };
        let result = self.extract_values(GET_LABELS_SQL, mapper).await?;
        Ok(result)
    }

    /// Returns periods with contained composers.
    pub async fn get_periods(&self) -> anyhow::Result<Vec<Period>> {
        let query = sqlx::query(GET_PERIODS_SQL);
        let periods: Vec<Period> = self.extract_json(query).await?;
        Ok(periods)
    }

    /// Returns composer data.
    pub async fn get_composer(&self, id: &str) -> anyhow::Result<Composer> {
        let query = sqlx::query(GET_COMPOSER).bind(id);
        let composer: Composer = self.extract_json(query).await?;
        Ok(composer)
    }

    /// Returns genres for composer
    pub async fn get_genres(&self, id: i32) -> anyhow::Result<Vec<Genre>> {
        let query = sqlx::query(GET_GENRES).bind(id);
        let genres: Vec<Genre> = self.extract_json(query).await?;
        Ok(genres)
    }

}
