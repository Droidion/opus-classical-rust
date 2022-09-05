use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Streaming service, e.g. Spotify, Tidal, or Apple Music.
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Streamer {
    pub name: String,
    pub icon: String, // Logo filename, e.g. foo.webp
    pub link: String,
    pub prefix: String,
}
