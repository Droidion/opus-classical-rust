use crate::domain::work::{Work, WorkTemplate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Genre of the work, like Symphony, or String Quartet, or Choral music.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Genre {
    pub name: String,
    pub icon: String, // e.g. 🐕
    pub works: Vec<Work>,
}

/// Genre of the work with additional data for html rendering.
#[derive(Serialize)]
pub struct GenreTemplate {
    pub base: Genre,
    pub works: Vec<WorkTemplate>,
}

impl From<Genre> for GenreTemplate {
    /// Adds more data for html rendering.
    fn from(item: Genre) -> Self {
        GenreTemplate {
            works: item
                .works
                .clone()
                .into_iter()
                .map(WorkTemplate::from)
                .collect(),
            base: item,
        }
    }
}
