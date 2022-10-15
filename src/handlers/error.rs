use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_common_error, ok_html_response, render_html, CustomError};
use crate::startup::AppData;
use axum::response::Response;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

/// Data for html template of Error page.
#[derive(Serialize)]
struct ErrorData {
    shared: SharedHandlerData,
    title: String,
}

/// Handler for Error page.
pub async fn error_handler(
    Extension(tmpl): Extension<Arc<tera::Tera>>,
    Extension(app_data): Extension<Arc<AppData>>,
) -> Result<Response, CustomError> {
    let data = ErrorData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Error"),
        title: String::from("Error"),
    };
    let html = render_html(tmpl, "pages/error.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
