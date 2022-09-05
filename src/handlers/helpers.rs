use actix_web::http::header::ContentType;
use actix_web::{error, web, Error, HttpResponse};
use log::error;
use serde::Serialize;
use tera::Context;

pub fn ok_response(html: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

pub fn handle_error(err: anyhow::Error) -> Error {
    error!("{}", err);
    error::ErrorInternalServerError("Server error happened")
}

pub fn render_html<T: Serialize>(
    tmpl: &web::Data<tera::Tera>,
    name: &str,
    data: &T,
) -> anyhow::Result<String> {
    let context = Context::from_serialize(data)?;
    let html = tmpl.render(name, &context)?;
    Ok(html)
}
