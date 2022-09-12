use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SharedHandlerData {
    pub umami_id: String,
    pub title: String,
}

impl SharedHandlerData {
    /// Adds more data for html rendering.
    pub fn new(umami_id: &String, title: &str) -> Self {
        Self {
            umami_id: umami_id.to_string(),
            title: format!("{} | Opus Classical", title),
        }
    }
}
