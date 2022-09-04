use crate::domain::work::Work;

pub struct Genre {
    pub name: String,
    pub icon: String, // e.g. 🐕
    pub works: Vec<Work>,
}