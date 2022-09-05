use crate::domain::composer::Composer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Period when composer lived and worked, e.g. Late Baroque or Romanticism.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Period {
    pub id: i32,
    pub name: String,
    pub year_start: i16,
    pub year_end: Option<i16>,
    pub slug: String, // Unique period readable text id, to be used in URLs.
    pub composers: Vec<Composer>,
}
