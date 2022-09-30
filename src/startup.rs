use crate::configuration::Settings;
use crate::handlers::about::about_handler;
use crate::handlers::composer::composer_handler;
use crate::handlers::index::index_handler;
use crate::handlers::search::search_handler;
use crate::handlers::work::work_handler;
use crate::repositories::database::{get_connection_pool, Database};
use actix_web::dev::Server;
use actix_web::middleware::DefaultHeaders;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer, web};
use std::net::TcpListener;
use actix_web::http::header;
use tera::Tera;
use crate::handlers::error::error_handler;
use crate::handlers::not_found::not_found_handler;

/// Application data for rendering in html templates.
pub struct AppData {
    pub static_assets_url: String,
    pub umami_id: String,
}

impl AppData {
    /// Creates new application data package.
    pub fn new(static_assets_url: &str, umami_id: &str) -> Self {
        Self {
            static_assets_url: static_assets_url.to_string(),
            umami_id: umami_id.to_string(),
        }
    }
}

/// Initialises tera html templates.
fn init_templates() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}

/// Returns default headers to be applied to all served resources.
fn add_cache_headers() -> DefaultHeaders {
    DefaultHeaders::new().add((header::CACHE_CONTROL, "public, max-age=604800"))
}

fn add_no_cache_headers() -> DefaultHeaders {
    DefaultHeaders::new().add((header::CACHE_CONTROL, "private, max-age=0"))
}

/// Builds web server.
pub async fn build_app(configuration: Settings) -> Result<Server, anyhow::Error> {
    let address = format!("0.0.0.0:{}", configuration.application.port);
    let listener = TcpListener::bind(&address)?;
    let database = Data::new(Database {
        pg_pool: get_connection_pool(&configuration.database),
    });
    let templates = init_templates();
    let server = HttpServer::new(move || {
        let app_data = AppData::new(&configuration.static_assets_url, &configuration.umami_id);
        App::new()
            // Middleware
            .wrap(middleware::Compress::default())
            .wrap(add_no_cache_headers())
            // Routes
            .service(web::scope("/static").wrap(add_cache_headers()).service(actix_files::Files::new("/", "./static")))
            .service(index_handler)
            .service(work_handler)
            .service(composer_handler)
            .service(about_handler)
            .service(search_handler)
            .service(error_handler)
            .default_service(web::route().to(not_found_handler))
            // App data shared in handlers
            .app_data(Data::new(app_data))
            .app_data(Data::new(templates.clone()))
            .app_data(database.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
