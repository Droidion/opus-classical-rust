use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::startup::AppData;
use actix_web::{web, Error, HttpResponse};
use serde::Serialize;

/// Data for html template of 404 page.
#[derive(Serialize)]
struct ComposerData {
    shared: SharedHandlerData,
    title: String,
}

/// Handler for 404 page.
pub async fn not_found_handler(
    tmpl: web::Data<tera::Tera>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    let data = ComposerData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Not found"),
        title: String::from("Not found"),
    };
    let html = render_html(&tmpl, "pages/404.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
