use axum::http::StatusCode;
use axum::{
    response::{Html, IntoResponse, Response},
    Extension, Json,
};
use log::error;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tera::Context;

#[derive(Debug)]
pub enum CustomError {
    InternalError,
    SearchError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong"),
        )
            .into_response()
    }
}
/*
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::FOUND,
            CustomError::SearchError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalError => HttpResponse::Found()
                .append_header(("Location", "/error"))
                .finish(),
            CustomError::SearchError => HttpResponse::BadRequest().body("Bad request"),
        }
    }
}
*/
/// Adds common security headers to HTTP response.
/*
fn add_security_headers(builder: &mut HttpResponseBuilder) -> &mut HttpResponseBuilder {
    builder
        .append_header((header::REFERRER_POLICY, "no-referrer"))
        .append_header((header::STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains; preload"))
        .append_header(("permissions-policy", "geolocation=(), microphone=()"))
        .append_header((header::CONTENT_SECURITY_POLICY, "default-src 'none'; manifest-src 'self'; connect-src 'self' https://logs.opusclassical.net; script-src 'self' https://logs.opusclassical.net; style-src 'self'; img-src 'self' https://static.zunh.dev"))
        .append_header((header::X_XSS_PROTECTION, "1; mode=block"))
        .append_header((header::X_FRAME_OPTIONS, "sameorigin"))
        .append_header((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        .append_header(("X-Permitted-Cross-Domain-Policies", "none"))
}*/

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
