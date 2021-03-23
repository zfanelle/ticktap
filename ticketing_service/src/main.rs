use crate::config::Config;
use crate::controllers::ticket_controller::test;
use actix_web::{App, HttpServer};

mod config;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().unwrap();

    let app_config = Config::initialize_application().await;
    let app_config = app_config;

    HttpServer::new(move || App::new().service(test).data(app_config.clone()))
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
