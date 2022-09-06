use crate::domain::composer::Composer;
use crate::domain::recording::Recording;
use crate::domain::work::Work;
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::repositories::database::Database;
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;
use crate::helpers::format_work_length;

#[derive(Serialize)]
struct WorkData {
    composer: Composer,
    work: Work,
    child_works: Vec<Work>,
    recordings: Vec<RecordingTemplate>,
    static_assets_url: String,
}

#[derive(Serialize)]
struct RecordingTemplate {
    base: Recording,
    length_formatted: String,
}

impl From<Recording> for RecordingTemplate {
    fn from(item: Recording) -> Self {
        RecordingTemplate {
            length_formatted: format_work_length(Some(item.length)),
            base: item,
        }
    }
}

#[get("/composer/{slug}/work/{id}")]
pub async fn work_handler(
    params: web::Path<(String, i32)>,
    database: web::Data<Database>,
    static_assets_url: web::Data<String>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let (slug, id) = params.into_inner();
    let recordings: Vec<RecordingTemplate> = database.get_recordings(id).await.map_err(handle_error)?.into_iter().map(|rec| RecordingTemplate::from(rec)).collect();
    let data = WorkData {
        composer: database
            .get_composer(slug.as_str())
            .await
            .map_err(handle_error)?,
        work: database.get_work(id).await.map_err(handle_error)?,
        child_works: database.get_child_works(id).await.map_err(handle_error)?,
        recordings: recordings,
        static_assets_url: static_assets_url.to_string(),
    };
    let html = render_html(&tmpl, "work.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
