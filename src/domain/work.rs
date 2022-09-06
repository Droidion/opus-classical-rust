use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::helpers::{format_years_range_loose};

/// Musical work, like Symphony No. 9 by Beethoven
#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Work {
    pub id: i32,
    pub title: String,
    pub year_start: Option<i16>, // Year when composer started the work, if known. Can be used without YearFinish if the work was finished in a single year.
    pub year_finish: Option<i16>, // Year when composer finished the work, if known. Can be used without YearStart if the work was finished in a single year.
    pub average_minutes: Option<i16>, // Approximate length of the work in minutes.
    pub catalogue_name: Option<String>, // Name of the catalogue of composer's works, like "BWV" for Bach or "Op." for Beethoven.
    pub catalogue_number: Option<i32>,  // Catalogue number of the work, like 123 for Op. 123
    pub catalogue_postfix: Option<String>, // Postfix for the number of the work in the catalogue, like b in Op. 123b
    pub key: Option<String>,               // e.g. C# minor
    pub no: Option<i32>,                   // Work number in some sequence, like 9 in Symphony No. 9
    pub nickname: Option<String>,          // e.g. Great in Beethoven's Symphony No. 9 Great
}

impl Work {
    /// Formats catalogue name and number, e.g. "BWV 1034" for Bach's Flute Sonata No. 1
    pub fn format_catalogue_name(&self) -> String {
        let postfix = self.catalogue_postfix.clone().unwrap_or_else(|| "".to_string());
        match (self.catalogue_name.clone(), self.catalogue_number) {
            (Some(name), Some(number)) => format!("{} {}{}", name, number, postfix),
            (_, _) => "".to_string(),
        }
    }

    pub fn format_work_name(&self) -> String {
        match (self.no, self.nickname.clone()) {
            (Some(no), Some(nickname)) => format!("{} No. {} {}", self.title, no, nickname),
            (Some(no), None) => format!("{} No. {}", self.title, no),
            (None, Some(nickname)) => format!("{} {}", self.title, nickname),
            (None, None) => self.title.clone()
        }
    }
}

#[derive(Serialize)]
pub struct WorkTemplate {
    pub base: Work,
    pub compose_period: String,
    pub catalogue_notation: String,
    pub full_name: String,
}

impl From<Work> for WorkTemplate {
    fn from(item: Work) -> Self {
        WorkTemplate {
            compose_period: format_years_range_loose(item.year_start, item.year_finish),
            catalogue_notation: item.format_catalogue_name(),
            full_name: item.format_work_name(),
            base: item,
        }
    }
}