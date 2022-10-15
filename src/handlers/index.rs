use crate::domain::period::PeriodTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_common_error, ok_html_response, render_html, CustomError};
use crate::repositories::database::Database;
use crate::startup::AppData;
use axum::response::Response;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

/// Data for html template of Index page.
#[derive(Serialize)]
struct IndexData {
    shared: SharedHandlerData,
    periods: Vec<PeriodTemplate>,
}

/// Handler for Index page.
pub async fn index_handler(
    Extension(database): Extension<Arc<Database>>,
    Extension(tmpl): Extension<Arc<tera::Tera>>,
    Extension(app_data): Extension<Arc<AppData>>,
) -> Result<Response, CustomError> {
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
    let html = render_html(tmpl, "pages/periods.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
