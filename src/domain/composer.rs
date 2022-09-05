use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Search result for a composer.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Composer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub year_born: i32,
    pub year_died: Option<i32>,
    pub countries: Vec<String>,
    pub slug: String,
    pub wikipedia_link: Option<String>,
    pub imslp_link: Option<String>,
    pub enabled: bool,
}
