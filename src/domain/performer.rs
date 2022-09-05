use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Performer of some musical work.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Performer {
    pub first_name: Option<String>,
    pub last_name: String,
    pub priority: Option<i32>,
    pub instrument: Option<String>,
}
