use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{
    response::{Html, IntoResponse, Response},
    Json,
};
use log::error;
use serde::Serialize;
use std::sync::Arc;
use tera::Context;
use tokio::io;

#[derive(Debug)]
pub enum CustomError {
    InternalError,
    SearchError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        match self {
            CustomError::InternalError => Redirect::permanent("/error").into_response(),
            CustomError::SearchError => {
                (StatusCode::BAD_REQUEST, "Bad request".to_string()).into_response()
            }
        }
    }
}

/// Returns HTTP response from html string, setting content type and status 200.
pub fn ok_html_response(html: String) -> Response {
    Html(html).into_response()
}

/// Returns HTTP response from as JSON body, setting content type and status 200.
pub fn ok_json_response<T: Serialize>(json: T) -> Response {
    Json(json).into_response()
}

/// Processes error on controller/handler level.
pub fn handle_common_error(err: anyhow::Error) -> CustomError {
    error!("{}", err);
    CustomError::InternalError
}

/// Processes specific error for search endpoint.
pub fn handle_search_error(err: anyhow::Error) -> CustomError {
    error!("{}", err);
    CustomError::SearchError
}

pub async fn handle_static_asset_error(_err: io::Error) -> impl IntoResponse {
    error!("{}", _err);
    CustomError::SearchError
}

/// Renders tera html template to string.
pub fn render_html<T: Serialize>(
    tmpl: Arc<tera::Tera>,
    name: &str,
    data: &T,
) -> anyhow::Result<String> {
    let context = Context::from_serialize(data)?;
    let html = tmpl.render(name, &context)?;
    Ok(html)
}
