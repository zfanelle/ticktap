use crate::config::Config;
use crate::controllers::ticket_controller::{
    create_ticket, get_all_tickets_by_transaction, get_ticket, get_ticket_quantity_by_transaction,
    get_tickets, test,
};
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
            .service(test)
            .service(create_ticket)
            .service(get_ticket_quantity_by_transaction)
            .service(get_ticket)
            .service(get_tickets)
            .service(get_all_tickets_by_transaction)
            .data(app_config.clone())
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
