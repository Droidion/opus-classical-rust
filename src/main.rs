mod configuration;
mod domain;
mod handlers;
mod helpers;
mod repositories;
mod startup;

use crate::configuration::get_configuration;
use crate::startup::build_app;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read environments variables from .env file.
    dotenv().ok();
    // Set logs level.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Read and parse configuration from environment.
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Create app
    let app = build_app(configuration.clone()).await?;
    // Run app in foreground until stopped.
    app.await?;
    Ok(())
}
