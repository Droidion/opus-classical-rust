use std::net::TcpListener;
use actix_web::dev::Server;
use crate::configuration::{Settings};
use sqlx::PgPool;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use crate::repositories::database::{Database, get_connection_pool};
use crate::routes::test::greet;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "127.0.0.1:{}",
            configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let server = run(
            listener,
            connection_pool,
        )
            .await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, anyhow::Error> {
    let database = Data::new(Database{
        pg_pool: db_pool
    });
    let server = HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .app_data(database.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}