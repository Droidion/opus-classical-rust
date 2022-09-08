use crate::domain::period::PeriodTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::repositories::database::Database;
use crate::startup::AppData;
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct IndexData {
    shared: SharedHandlerData,
    periods: Vec<PeriodTemplate>,
}

#[get("/")]
pub async fn index_handler(
    database: web::Data<Database>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let periods = database
        .get_periods()
        .await
        .map_err(handle_error)?
        .into_iter()
        .map(PeriodTemplate::from)
        .collect();
    let data = IndexData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Composers"),
        periods,
    };
    let html = render_html(&tmpl, "periods.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
