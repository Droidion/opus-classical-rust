use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Search result for a composer.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ComposerSearchResult {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub slug: String,
    pub rating: f32,
}
