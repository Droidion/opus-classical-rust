use crate::handlers::helpers::{handle_search_error, ok_json_response, CustomError};
use crate::repositories::database::Database;
use axum::extract::Query;
use axum::response::Response;
use axum::Extension;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Query parameters of Search API endpoint.
#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Handler for Search API endpoint.
pub async fn search_handler(
    Extension(database): Extension<Arc<Database>>,
    query: Query<SearchQuery>,
) -> Result<Response, CustomError> {
    let q = &query.q;
    let search_result = database
        .search_composers(q.to_string(), 5)
        .await
        .map_err(handle_search_error)?;
    Ok(ok_json_response(search_result))
}
