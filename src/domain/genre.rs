use crate::domain::work::Work;

/// Genre of the work, like Symphony, or String Quartet, or Choral music.
pub struct Genre {
    pub name: String,
    pub icon: String, // e.g. 🐕
    pub works: Vec<Work>,
}