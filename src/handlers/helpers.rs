use actix_web::http::header::ContentType;
use actix_web::{error, web, Error, HttpResponse};
use log::error;
use serde::Serialize;
use tera::Context;

/// Returns HTTP response from html string, setting content type and status 200.
pub fn ok_response(html: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
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
