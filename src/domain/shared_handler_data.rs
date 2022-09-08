use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SharedHandlerData {
    pub umami_id: String,
    pub title: String,
}

impl SharedHandlerData {
    pub fn new(umami_id: &String, title: &str) -> Self {
        Self {
            umami_id: umami_id.to_string(),
            title: format!("{} | Opus Classical", title),
        }
    }
}
