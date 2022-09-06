use crate::domain::period::{Period, PeriodTemplate};
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::repositories::database::Database;
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct IndexData {
    periods: Vec<PeriodTemplate>,
}

#[get("/")]
pub async fn index_handler(
    database: web::Data<Database>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let periods = database.get_periods().await.map_err(handle_error)?.into_iter().map(PeriodTemplate::from).collect();
    let data = IndexData {
        periods,
    };
    let html = render_html(&tmpl, "periods.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
