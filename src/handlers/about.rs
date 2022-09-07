use crate::handlers::helpers::{handle_error, ok_response, render_html};
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ComposerData {
    title: String,
}

#[get("/about")]
pub async fn about_handler(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let data = ComposerData {
        title: String::from("About"),
    };
    let html = render_html(&tmpl, "about.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
