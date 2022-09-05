use crate::domain::label::Label;
use crate::domain::period::Period;
use crate::repositories::database::Database;
use actix_web::http::header::ContentType;
use actix_web::{error, get, web, Error, HttpResponse};
use log::error;
use serde::Serialize;
use tera::Context;
use crate::domain::composer::Composer;
use crate::domain::genre::Genre;

#[derive(Serialize)]
struct LabelsTemplate {
    name: String,
    labels: Vec<Label>,
}


#[derive(Serialize)]
struct IndexTemplate {
    periods: Vec<Period>,
}

#[derive(Serialize)]
struct ComposerTemplate {
    composer: Composer,
    genres: Vec<Genre>,
}

#[get("/")]
pub async fn index(
    database: web::Data<Database>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let data = IndexTemplate {
        periods: database.get_periods().await.map_err(handle_error)?,
    };
    let html = render_html(&tmpl, "periods.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}

#[get("/composer/{slug}")]
pub async fn composer(
    slug: web::Path<String>,
    database: web::Data<Database>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let composer = database.get_composer(slug.into_inner().as_str()).await.map_err(handle_error)?;
    let genres = database.get_genres(composer.id).await.map_err(handle_error)?;
    let data = ComposerTemplate {
        composer,
        genres,
    };
    let html = render_html(&tmpl, "composer.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}

#[get("/hello/{name}")]
pub async fn greet(
    name: web::Path<String>,
    database: web::Data<Database>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let data = LabelsTemplate {
        name: name.into_inner(),
        labels: database.get_labels().await.map_err(handle_error)?,
    };
    let html = render_html(&tmpl, "labels.html", &data).map_err(handle_error)?;
    Ok(ok_response(html))
}

fn ok_response(html: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

fn handle_error(err: anyhow::Error) -> Error {
    error!("{}", err);
    error::ErrorInternalServerError("Server error happened")
}

fn render_html<T: Serialize>(
    tmpl: &web::Data<tera::Tera>,
    name: &str,
    data: &T,
) -> anyhow::Result<String> {
    let context = Context::from_serialize(data)?;
    let html = tmpl.render(name, &context)?;
    Ok(html)
}
