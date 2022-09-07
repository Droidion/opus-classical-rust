use crate::helpers::format_years_range_string;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Search result for a composer.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Composer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub year_born: i16,
    pub year_died: Option<i16>,
    pub countries: Vec<String>,
    pub slug: String,
    pub wikipedia_link: Option<String>,
    pub imslp_link: Option<String>,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct ComposerTemplate {
    pub base: Composer,
    pub years_lived: String,
}

impl From<Composer> for ComposerTemplate {
    fn from(item: Composer) -> Self {
        ComposerTemplate {
            years_lived: format_years_range_string(item.year_born, item.year_died),
            base: item,
        }
    }
}
