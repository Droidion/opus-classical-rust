use crate::domain::composer::{Composer, ComposerTemplate};
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

/// Period when composer lived and worked with additional data for html rendering.
#[derive(Serialize)]
pub struct PeriodTemplate {
    pub base: Period,
    pub composers: Vec<ComposerTemplate>,
}

impl From<Period> for PeriodTemplate {
    /// Adds more data for html rendering.
    fn from(item: Period) -> Self {
        PeriodTemplate {
            composers: item
                .composers
                .clone()
                .into_iter()
                .map(ComposerTemplate::from)
                .collect(),
            base: item,
        }
    }
}
