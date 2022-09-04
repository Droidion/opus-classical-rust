use crate::domain::performer::Performer;
use crate::domain::streamer::Streamer;

pub struct Recording {
    pub id: i32,
    pub cover_name: String,
    pub year_start: Option<i32>,
    pub year_finish: Option<i32>,
    pub performers: Vec<Performer>,
    pub label: Option<String>,
    pub length: i32,
    pub streamers: Vec<Streamer>,
}