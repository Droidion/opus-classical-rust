use crate::configuration::Settings;
use crate::handlers::about::about_handler;
use crate::handlers::composer::composer_handler;
use crate::handlers::index::index_handler;
use crate::handlers::search::search_handler;
use crate::handlers::work::work_handler;
use crate::repositories::database::{get_connection_pool, Database};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tera::Tera;

pub struct Application {
    server: Server,
}

impl Application {
    /// Builds application.
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let static_assets_url = configuration.static_assets_url;
        let address = format!("0.0.0.0:{}", configuration.application.port);
        let listener = TcpListener::bind(&address)?;
        let server = run(listener, connection_pool, static_assets_url).await?;
        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Runs web server.
async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    static_assets_url: String,
) -> Result<Server, anyhow::Error> {
    let database = Data::new(Database { pg_pool: db_pool });
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new().add(("cache-control", "public, max-age=604800")),
            )
            .service(actix_files::Files::new("/static", "./static"))
            .service(index_handler)
            .service(work_handler)
            .service(composer_handler)
            .service(about_handler)
            .service(search_handler)
            .app_data(Data::new(tera.clone()))
            .app_data(Data::new(static_assets_url.clone()))
            .app_data(database.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
