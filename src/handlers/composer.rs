use crate::domain::composer::Composer;
use crate::domain::genre::GenreTemplate;
use crate::domain::shared_handler_data::SharedHandlerData;
use crate::handlers::helpers::{handle_common_error, ok_html_response, render_html, CustomError};
use crate::repositories::database::Database;
use crate::startup::AppData;
use axum::extract::Path;
use axum::response::Response;
use axum::Extension;
use serde::Serialize;
use std::sync::Arc;

/// Data for html template of Composer page.
#[derive(Serialize)]
struct ComposerData {
    shared: SharedHandlerData,
    composer: Composer,
    genres: Vec<GenreTemplate>,
}

/// Handler for Composer page.
pub async fn composer_handler(
    Path(slug): Path<String>,
    Extension(database): Extension<Arc<Database>>,
    Extension(tmpl): Extension<Arc<tera::Tera>>,
    Extension(app_data): Extension<Arc<AppData>>,
) -> Result<Response, CustomError> {
    let composer: Composer = database
        .get_composer(slug.as_str())
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
    let html = render_html(tmpl, "pages/composer.html", &data).map_err(handle_common_error)?;
    Ok(ok_html_response(html))
}
