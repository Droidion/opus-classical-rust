use actix_web::http::header::ContentType;
use actix_web::{error, web, Error, HttpResponse};
use log::error;
use serde::Serialize;
use tera::Context;

/// Returns HTTP response from html string, setting content type and status 200.
pub fn ok_response(html: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .append_header(("cache-control", "private, max-age=0"))
        .append_header(("referrer-policy", "no-referrer"))
        .append_header(("strict-transport-security", "max-age=31536000; includeSubDomains; preload"))
        .append_header(("permissions-policy", "geolocation=(), microphone=()"))
        .append_header(("content-security-policy", "default-src 'none'; manifest-src 'self'; connect-src 'self' https://logs.opusclassical.net; script-src 'self' https://logs.opusclassical.net; style-src 'self'; img-src 'self' https://static.zunh.dev"))
        .body(html)
}

/// Processes error on controller/handler level.
pub fn handle_error(err: anyhow::Error) -> Error {
    error!("{}", err);
    error::ErrorInternalServerError("Server error happened")
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
