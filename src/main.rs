#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use dropshot::{
    ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServerStarter
};
use std::env;

mod db;
mod schema;
mod books;
mod health;
pub mod visitor;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Fetch values from the .env file if available.
    dotenv().ok();

    // Initialize the pool and trigger migrations.
    db::init();

    // Specifying the configuration for dropshot, which simply binds to a given
    // port and assigns a maximum value for the request body.
    let port = env::var("LLIBRES_PORT").unwrap_or(String::from("8080"));
    let config: ConfigDropshot = ConfigDropshot {
        bind_address: format!("0.0.0.0:{}", port).parse().unwrap(),
        request_body_max_bytes: 1024,
    };

    // Default and simply logging for this application.
    let log_config = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = log_config
        .to_logger("llibres")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    // Build a description of the API.
    let mut api = ApiDescription::new();
    books::draw(&mut api);
    health::draw(&mut api);

    // Set up and run the server. For now we are not sharing any state, so an
    // empty one should suffice.
    HttpServerStarter::new(&config, api, (), &log)
        .map_err(|error| format!("failed to create server: {}", error))?
    .start()
    .await
}
