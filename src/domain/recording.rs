use crate::domain::performer::Performer;
use crate::domain::streamer::Streamer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::helpers::{format_work_length, format_years_range_loose};

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
    pub length: i16,
    pub streamers: Vec<Streamer>,
}

#[derive(Serialize)]
pub struct RecordingTemplate {
    pub base: Recording,
    pub length_formatted: String,
    pub recording_period: String,
}

impl From<Recording> for RecordingTemplate {
    fn from(item: Recording) -> Self {
        RecordingTemplate {
            length_formatted: format_work_length(Some(item.length)),
            recording_period: format_years_range_loose(item.year_start, item.year_finish),
            base: item,
        }
    }
}