use sqlx::FromRow;
use serde::{Serialize};

#[derive(Debug, FromRow, Serialize)]
pub struct Label {
    pub id: i32,
    pub name: String,
}
