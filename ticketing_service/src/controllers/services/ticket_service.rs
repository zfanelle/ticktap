use super::super::models::ticket::Ticket;
use super::repositories::ticket_repository;
use crate::config::AppConfig;
use crate::controllers::models::error::{ClientError, RepositoryError};
use crate::controllers::models::event::Event;
use crate::controllers::models::transaction::Transaction;

pub async fn get_ticket(
    app_config: &AppConfig,
    ticket_id: String,
) -> Result<Ticket, RepositoryError> {
    let ticket_id = ticket_id.parse()?;

    Ok(ticket_repository::get_ticket(app_config, ticket_id).await?)
}

pub async fn create_ticket(app_config: &AppConfig, ticket: &Ticket) -> Result<(), RepositoryError> {
    // check remaining tickets

    let requested_ticket_quantity = ticket
        .quantity
        .ok_or_else(|| ClientError::QuantityNotFound)?;

    let purchase_tickets_available =
        sufficient_tickets_available(app_config, ticket.transaction_id, requested_ticket_quantity)
            .await?;

    if purchase_tickets_available {
        for _x in 0..requested_ticket_quantity {
            ticket_repository::create_ticket(app_config, &ticket).await?;
        }
        Ok(())
    } else {
        Err(RepositoryError::InsufficientTicketsAvailable)
    }
}

pub async fn get_all_tickets(app_config: &AppConfig) -> Result<Vec<Ticket>, RepositoryError> {
    Ok(ticket_repository::get_all_tickets(app_config).await?)
}

pub async fn get_all_tickets_by_transaction(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<Vec<Ticket>, RepositoryError> {
    Ok(ticket_repository::get_all_tickets_by_transaction(app_config, transaction_id).await?)
}

pub async fn get_ticket_quantity_by_transaction(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<i32, RepositoryError> {
    Ok(ticket_repository::check_booked_tickets(app_config, transaction_id).await?)
}

async fn sufficient_tickets_available(
    app_config: &AppConfig,
    transaction_id: i32,
    requested_ticket_quantity: i32,
) -> Result<bool, RepositoryError> {
    let tickets_booked = get_ticket_quantity_by_transaction(app_config, transaction_id).await?;

    let max_ticket_capacity = get_event_ticket_max(app_config, transaction_id).await?;

    if tickets_booked + requested_ticket_quantity < max_ticket_capacity {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

async fn get_event_ticket_max(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<i32, RepositoryError> {
    let client = app_config.http_client.clone();

    // Get the event id
    let transaction_endpoint =
        app_config.main_service_host.clone() + "/transaction/" + &transaction_id.to_string();
    let response = client
        .get(transaction_endpoint)
        .send()
        .await?
        .text()
        .await?;
    let transaction: Transaction = serde_json::from_str(&response)?;

    let event_endpoint =
        app_config.main_service_host.clone() + "/event/" + &transaction.event.to_string();
    let response = client.get(event_endpoint).send().await?.text().await?;
    let event: Event = serde_json::from_str(&response.to_owned())?;

    Ok(event.maximum_ticket_capacity)
}
