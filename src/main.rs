extern crate env_logger;
use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger};
use config::Config;
use std::collections::HashMap;

struct AppState {
    app_name: String,
}

impl AppState {
    fn init() -> AppState {
        AppState {
            app_name: String::from("Actix Web"),
        }
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {name} from {app_name}!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(AppState::init()))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}