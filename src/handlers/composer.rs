use crate::domain::composer::Composer;
use crate::domain::genre::GenreTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_common_error, ok_html_response, render_html, CustomError};
use crate::repositories::database::Database;
use crate::startup::AppData;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

/// Data for html template of Composer page.
#[derive(Serialize)]
struct ComposerData {
    shared: SharedHandlerData,
    composer: Composer,
    genres: Vec<GenreTemplate>,
}

/// Handler for Composer page.
#[get("/composer/{slug}")]
pub async fn composer_handler(
    slug: web::Path<String>,
    database: web::Data<Database>,
    app_data: web::Data<AppData>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, CustomError> {
    let composer: Composer = database
        .get_composer(slug.into_inner().as_str())
        .await
        .map_err(handle_common_error)?;
    let genres: Vec<GenreTemplate> = database
        .get_genres(composer.id)
        .await
        .map_err(handle_common_error)?
        .into_iter()
        .map(GenreTemplate::from)
        .collect();
    let data = ComposerData {
        shared: SharedHandlerData::new(&app_data.umami_id, &composer.last_name),
        composer,
        genres,
    };
    let html = render_html(&tmpl, "pages/composer.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
