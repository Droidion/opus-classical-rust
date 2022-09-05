use crate::configuration::Settings;
use crate::repositories::database::{get_connection_pool, Database};
use crate::routes::test::{greet, index, composer};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tera::Tera;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!("127.0.0.1:{}", configuration.application.port);
        let listener = TcpListener::bind(&address)?;
        let server = run(listener, connection_pool).await?;
        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, anyhow::Error> {
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
            .service(index)
            .service(composer)
            .service(greet)
            .app_data(Data::new(tera.clone()))
            .app_data(database.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
