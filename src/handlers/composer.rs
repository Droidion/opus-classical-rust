use crate::domain::composer::Composer;
use crate::domain::genre::Genre;
use crate::handlers::helpers::{handle_error, ok_response, render_html};
use crate::repositories::database::Database;
use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ComposerData {
    composer: Composer,
    genres: Vec<Genre>,
}

#[get("/composer/{slug}")]
pub async fn composer_handler(
    slug: web::Path<String>,
    database: web::Data<Database>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let composer: Composer = database
        .get_composer(slug.into_inner().as_str())
        .await
        .map_err(handle_error)?;
    let genres: Vec<Genre> = database
        .get_genres(composer.id)
        .await
        .map_err(handle_error)?;
    let data = ComposerData { composer, genres };
    let html = render_html(&tmpl, "composer.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}
