use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::startup::AppData;
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

/// Data for html template of About page.
#[derive(Serialize)]
struct AboutData {
    shared: SharedHandlerData,
    title: String,
}

/// Handler for About page.
#[get("/about")]
pub async fn about_handler(
    tmpl: web::Data<tera::Tera>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let data = AboutData {
        shared: SharedHandlerData::new(&app_data.umami_id, "About"),
        title: String::from("About"),
    };
    let html = render_html(&tmpl, "pages/about.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
