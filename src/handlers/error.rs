use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{CustomError, handle_common_error, ok_html_response, render_html};
use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

/// Data for html template of About page.
#[derive(Serialize)]
struct ErrorData {
    shared: SharedHandlerData,
    title: String,
}

/// Handler for Error page.
#[get("/error")]
pub async fn error_handler(
    tmpl: web::Data<tera::Tera>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, CustomError> {
    let data = ErrorData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Error"),
        title: String::from("Error"),
    };
    let html = render_html(&tmpl, "pages/error.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
