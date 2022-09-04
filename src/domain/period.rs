use crate::domain::composer::Composer;

pub struct Period {
    id: i32,
    name: String,
    year_start: i32,
    year_end: Option<i32>,
    slug: String, // Unique period readable text id, to be used in URLs.
    composers: Vec<Composer>,
}
