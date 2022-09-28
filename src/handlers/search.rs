use crate::handlers::helpers::handle_error;
use crate::repositories::database::Database;
use actix_web::{get, web, Error, Responder};
use serde::{Deserialize, Serialize};

/// Query parameters of Search API endpoint.
#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

/// Handler for Search API endpoint.
#[get("/api/search")]
pub async fn search_handler(
    database: web::Data<Database>,
    query: web::Query<SearchQuery>,
) -> Result<impl Responder, Error> {
    let q = query.into_inner().q;
    let search_result = database
        .search_composers(q, 5)
        .await
        .map_err(handle_error)?;
    Ok(web::Json(search_result))
}
