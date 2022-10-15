use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_common_error, ok_html_response, render_html, CustomError};
use crate::startup::AppData;
use axum::response::Response;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

/// Data for html template of 404 page.
#[derive(Serialize)]
struct ComposerData {
    shared: SharedHandlerData,
    title: String,
}

/// Handler for 404 page.
pub async fn not_found_handler(
    Extension(tmpl): Extension<Arc<tera::Tera>>,
    Extension(app_data): Extension<Arc<AppData>>,
) -> Result<Response, CustomError> {
    let data = ComposerData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Not found"),
        title: String::from("Not found"),
    };
    let html = render_html(tmpl, "pages/404.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
