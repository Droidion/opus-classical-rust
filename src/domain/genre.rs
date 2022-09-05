use crate::domain::work::Work;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

/// Genre of the work, like Symphony, or String Quartet, or Choral music.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Genre {
    pub name: String,
    pub icon: String, // e.g. 🐕
    pub works: Vec<Work>,
}