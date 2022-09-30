use crate::domain::composer::Composer;
use crate::domain::recording::RecordingTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::domain::work::WorkTemplate;
use crate::handlers::helpers::{CustomError, handle_common_error, ok_html_response, render_html};
use crate::repositories::database::Database;
use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;
use crate::helpers::parse_string;

/// Data for html template of Work page.
#[derive(Serialize)]
struct WorkData {
    shared: SharedHandlerData,
    composer: Composer,
    work: WorkTemplate,
    child_works: Vec<WorkTemplate>,
    recordings: Vec<RecordingTemplate>,
    static_assets_url: String,
}

/// Handler for Work page.
#[get("/composer/{slug}/work/{id}")]
pub async fn work_handler(
    params: web::Path<(String, String)>,
    database: web::Data<Database>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, CustomError> {
    let (slug, id) = params.into_inner();
    let id_as_int = parse_string(id).map_err(handle_common_error)?;
    let work: WorkTemplate = database.get_work(id_as_int).await.map_err(handle_common_error)?.into();
    let template_data = WorkData {
        shared: SharedHandlerData::new(&app_data.umami_id, &work.full_name),
        composer: database
            .get_composer(slug.as_str())
            .await
            .map_err(handle_common_error)?,
        work,
        child_works: database
            .get_child_works(id_as_int)
            .await
            .map_err(handle_common_error)?
            .into_iter()
            .map(WorkTemplate::from)
            .collect(),
        recordings: database
            .get_recordings(id_as_int)
            .await
            .map_err(handle_common_error)?
            .into_iter()
            .map(RecordingTemplate::from)
            .collect(),
        static_assets_url: app_data.static_assets_url.to_string(),
    };
    let html = render_html(&tmpl, "pages/work.html", &template_data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
