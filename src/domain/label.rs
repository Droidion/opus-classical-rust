use sqlx::{FromRow};

#[derive(Debug, FromRow)]
pub struct Label {
    pub id: i32,
    pub name: String,
}