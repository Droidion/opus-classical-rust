use std::fmt::{Display, Formatter};
use actix_web::http::{header, StatusCode};
use actix_web::http::header::{ContentType};
use actix_web::{web, HttpResponse, ResponseError, HttpResponseBuilder};
use log::error;
use serde::Serialize;
use tera::Context;
use actix_web::web::Json;

#[derive(Debug)]
pub enum CustomError {
    InternalError,
    SearchError
}

impl Display for CustomError {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::FOUND,
            CustomError::SearchError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::InternalError => HttpResponse::Found().append_header(("Location", "/error")).finish(),
            CustomError::SearchError => HttpResponse::BadRequest().body("Bad request"),
        }

    }
}

/// Adds common security headers to HTTP response.
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
}

/// Returns HTTP response from html string, setting content type and status 200.
pub fn ok_html_response(html: String) -> HttpResponse {
    let mut response_builder = HttpResponse::Ok();
    add_security_headers(&mut response_builder);
    response_builder
        .content_type(ContentType::html())
        .body(html)
}

/// Returns HTTP response from as JSON body, setting content type and status 200.
pub fn ok_json_response<T: Serialize>(json: Json<T>) -> HttpResponse {
    let mut response_builder = HttpResponse::Ok();
    add_security_headers(&mut response_builder);
    response_builder
        .content_type(ContentType::json())
        .json(json)
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
    tmpl: &web::Data<tera::Tera>,
    name: &str,
    data: &T,
) -> anyhow::Result<String> {
    let context = Context::from_serialize(data)?;
    let html = tmpl.render(name, &context)?;
    Ok(html)
}
