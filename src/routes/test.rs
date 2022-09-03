use crate::repositories::database::Database;
use crate::templates::labels::labels_template;
use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpResponse};

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>, database: web::Data<Database>) -> HttpResponse {
    let labels = database.get_labels().await;
    let markup = labels_template(name.into_inner(), labels);
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(markup)
}
