use super::super::models::ticket::Ticket;
use super::repositories::ticket_repository;
use crate::config::AppConfig;
use crate::controllers::models::error::{ClientError, RepositoryError};

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
        sufficient_tickets_available(app_config, requested_ticket_quantity).await?;

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

async fn sufficient_tickets_available(
    app_config: &AppConfig,
    tickets: i32,
) -> Result<bool, RepositoryError> {
    let tickets_available = ticket_repository::check_ticket_quantity(app_config).await?;

    if tickets_available < tickets {
        return Ok(false);
    } else {
        return Ok(true);
    }
}
