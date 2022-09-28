use crate::configuration::DatabaseSettings;
use crate::domain::composer::Composer;
use crate::domain::composer_search_result::ComposerSearchResult;
use crate::domain::genre::Genre;
use crate::domain::period::Period;
use crate::domain::recording::Recording;
use crate::domain::work::Work;
use serde::de::DeserializeOwned;
use sqlx::postgres::PgRow;
use sqlx::postgres::{PgArguments, PgPoolOptions};
use sqlx::query::Query;
use sqlx::Row;
use sqlx::{PgPool, Postgres};

static GET_PERIODS_SQL: &str = "select json from periods_composers";
static GET_COMPOSER: &str = "select composer_by_slug($1) as json";
static GET_GENRES: &str = "select genres_and_works_by_composer($1) as json";
static GET_WORK_BY_ID: &str = r#"select w.id,
           w.title,
           w.year_start,
           w.year_finish,
           w.average_minutes,
           c.name catalogue_name,
           w.catalogue_number ,
           w.catalogue_postfix,
           k.name as key,
           w.no,
           w.nickname
    from works w
             left join catalogues c on w.catalogue_id = c.id
             left join keys k on w.key_id = k.id
    where w.id = $1"#;
static GET_CHILD_WORKS_BY_PARENT_WORK_ID: &str = r#"select w.id,
           w.title,
           w.year_start,
           w.year_finish,
           w.average_minutes,
           c.name as catalogue_name,
           w.catalogue_number,
           w.catalogue_postfix,
           k.name as key,
           w.no,
           w.nickname
    from works w
             left join catalogues c on w.catalogue_id = c.id
             left join keys k on w.key_id = k.id
    where w.parent_work_id = $1
    order by sort, year_finish, no, catalogue_number, catalogue_postfix, nickname"#;
static GET_RECORDINGS: &str = "SELECT recordings_by_work($1) AS json";
static SEARCH_COMPOSERS_BY_LAST_NAME: &str = "select id, first_name, last_name, slug, last_name_score from search_composers_by_last_name($1, $2)";

pub struct Database {
    pub pg_pool: PgPool,
}

/// Creates new connection pool.
pub fn get_connection_pool(db_config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_config.get_connection_options())
}

/// Maps musical work data from Postgres row to Rust struct.
fn work_mapper(row: PgRow) -> Work {
    Work {
        id: row.get("id"),
        title: row.get("title"),
        year_start: row.get("year_start"),
        year_finish: row.get("year_finish"),
        average_minutes: row.get("average_minutes"),
        catalogue_name: row.get("catalogue_name"),
        catalogue_number: row.get("catalogue_number"),
        catalogue_postfix: row.get("catalogue_postfix"),
        key: row.get("key"),
        no: row.get("no"),
        nickname: row.get("nickname"),
    }
}

impl Database {
    /// Extracts JSON from database.
    async fn extract_json<T: DeserializeOwned>(
        &self,
        query: Query<'_, Postgres, PgArguments>,
    ) -> anyhow::Result<T> {
        let postgres_row = query.fetch_one(&self.pg_pool).await?;
        let json_value: serde_json::Value = postgres_row.get("json");
        let parsed_value: T = serde_json::from_value(json_value)?;
        Ok(parsed_value)
    }

    /// Returns periods with contained composers.
    pub async fn get_periods(&self) -> anyhow::Result<Vec<Period>> {
        let query = sqlx::query(GET_PERIODS_SQL);
        let periods: Vec<Period> = self.extract_json(query).await?;
        Ok(periods)
    }

    /// Returns composer data.
    pub async fn get_composer(&self, slug: &str) -> anyhow::Result<Composer> {
        let query = sqlx::query(GET_COMPOSER).bind(slug);
        let composer: Composer = self.extract_json(query).await?;
        Ok(composer)
    }

    /// Returns genres for composer
    pub async fn get_genres(&self, id: i32) -> anyhow::Result<Vec<Genre>> {
        let query = sqlx::query(GET_GENRES).bind(id);
        let genres: Vec<Genre> = self.extract_json(query).await?;
        Ok(genres)
    }

    /// Returns work by id.
    pub async fn get_work(&self, id: i32) -> anyhow::Result<Work> {
        let work = sqlx::query(GET_WORK_BY_ID)
            .bind(id)
            .map(work_mapper)
            .fetch_one(&self.pg_pool)
            .await?;
        Ok(work)
    }

    /// Returns child works by its parent id.
    pub async fn get_child_works(&self, id: i32) -> anyhow::Result<Vec<Work>> {
        let works = sqlx::query(GET_CHILD_WORKS_BY_PARENT_WORK_ID)
            .bind(id)
            .map(work_mapper)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(works)
    }

    /// Returns recordings of a given work.
    pub async fn get_recordings(&self, id: i32) -> anyhow::Result<Vec<Recording>> {
        let query = sqlx::query(GET_RECORDINGS).bind(id);
        let recordings: Vec<Recording> = self.extract_json(query).await?;
        Ok(recordings)
    }

    /// Returns search results of composers by last name.
    pub async fn search_composers(
        &self,
        search_query: String,
        limit: i32,
    ) -> anyhow::Result<Vec<ComposerSearchResult>> {
        let mapper = |row: PgRow| ComposerSearchResult {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            slug: row.get("slug"),
            rating: row.get("last_name_score"),
        };
        let search_results = sqlx::query(SEARCH_COMPOSERS_BY_LAST_NAME)
            .bind(search_query)
            .bind(limit)
            .map(mapper)
            .fetch_all(&self.pg_pool)
            .await?;
        Ok(search_results)
    }
}
