use serde::Serialize;
use sqlx::FromRow;

/// Music Label, like Sony or EMI
#[derive(Debug, FromRow, Serialize)]
pub struct Label {
    pub id: i32,
    pub name: String,
}
