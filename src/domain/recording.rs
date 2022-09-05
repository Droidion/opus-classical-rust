use crate::domain::performer::Performer;
use crate::domain::streamer::Streamer;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

/// Recording of a musical work.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Recording {
    pub id: i32,
    pub cover_name: String,
    pub year_start: Option<i16>,
    pub year_finish: Option<i16>,
    pub performers: Vec<Performer>,
    pub label: Option<String>,
    pub length: i32,
    pub streamers: Vec<Streamer>,
}