use super::models::ticket::TicketDTO;
use super::services::ticket_service::create_ticket as service_create_ticket;
use super::services::ticket_service::get_all_tickets as service_get_all_tickets;
use super::services::ticket_service::get_all_tickets_by_transaction as service_get_all_tickets_by_transaction;
use super::services::ticket_service::get_ticket as service_get_ticket;
use super::services::ticket_service::get_ticket_quantity_by_transaction as service_get_ticket_quantity_by_transaction;

use crate::config::AppConfig;
use actix_web::{get, post, web, Error, HttpResponse};
use log::debug;
use serde_json::to_string;

#[get("/ticket/{ticket_id}")]
pub async fn get_ticket(
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let ticket_id = path.into_inner();
    let response_body = service_get_ticket(app_config.get_ref(), ticket_id).await?;

    let response_string = to_string(&response_body)?;
    Ok(HttpResponse::Ok().body(response_string))
}

#[get("/ticket/transaction/{transaction_id}")]
pub async fn get_all_tickets_by_transaction(
    path: web::Path<i32>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let transaction_id = path.into_inner();
    let response_body =
        service_get_all_tickets_by_transaction(app_config.get_ref(), transaction_id).await?;

    let response_string = to_string(&response_body)?;
    Ok(HttpResponse::Ok().body(response_string))
}

#[get("/ticket/transaction/{transaction_id}/quantity")]
pub async fn get_ticket_quantity_by_transaction(
    path: web::Path<i32>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let transaction_id = path.into_inner();

    let ticket_quantity =
        service_get_ticket_quantity_by_transaction(app_config.get_ref(), transaction_id).await?;

    Ok(HttpResponse::Ok().body(ticket_quantity.to_string()))
}

#[post("/ticket")]
pub async fn create_ticket(
    ticket: web::Json<TicketDTO>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let dto_ticket = ticket.into_inner();

    let ticket = dto_ticket.dto_to_ticket();

    service_create_ticket(app_config.get_ref(), &ticket).await?;
    Ok(HttpResponse::Ok().body("ticket created!"))
}

#[get("/ticket")]
pub async fn get_tickets(app_config: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let tickets = service_get_all_tickets(app_config.get_ref()).await?;

    let response_string = to_string(&tickets)?;

    Ok(HttpResponse::Ok().body(response_string))
}

#[get("/test")]
pub async fn test() -> HttpResponse {
    debug!("logging works!");
    HttpResponse::Ok().body("Ticktap is up and running!")
}
