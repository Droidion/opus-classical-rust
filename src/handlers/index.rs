use crate::domain::period::PeriodTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{CustomError, handle_common_error, ok_html_response, render_html};
use crate::repositories::database::Database;
use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

/// Data for html template of Index page.
#[derive(Serialize)]
struct IndexData {
    shared: SharedHandlerData,
    periods: Vec<PeriodTemplate>,
}

/// Handler for Index page.
#[get("/")]
pub async fn index_handler(
    database: web::Data<Database>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, CustomError> {
    let periods = database
        .get_periods()
        .await
        .map_err(handle_common_error)?
        .into_iter()
        .map(PeriodTemplate::from)
        .collect();
    let data = IndexData {
        shared: SharedHandlerData::new(&app_data.umami_id, "Composers"),
        periods,
    };
    let html = render_html(&tmpl, "pages/periods.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
