use crate::config::Config;
use crate::controllers::account_controller::{create_account, get_account, get_accounts, test};
use crate::controllers::event_controller::{create_event, get_event, get_events};
use actix_web::{App, HttpServer};

mod config;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().unwrap();

    let app_config = Config::initialize_application().await;
    let app_config = app_config;

    HttpServer::new(move || {
        App::new()
            .service(get_account)
            .service(create_account)
            .service(get_accounts)
            .service(get_event)
            .service(create_event)
            .service(get_events)
            .service(test)
            .data(app_config.clone())
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
