use super::super::models::ticket::TicketDTO;
use super::super::models::transaction::Transaction;
use super::repositories::{account_repository, transaction_repository};
use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;

pub async fn get_transaction(
    app_config: &AppConfig,
    transaction_id: String,
) -> Result<Transaction, RepositoryError> {
    let transaction_id = transaction_id.parse()?;

    let mut transaction =
        transaction_repository::get_transaction(app_config, transaction_id).await?;

    transaction.quantity = Some(get_ticket_qantity(app_config, transaction_id).await?);

    Ok(transaction)
}

pub async fn create_transaction(
    app_config: &AppConfig,
    transaction: &Transaction,
) -> Result<(), RepositoryError> {
    // check if account exists, and is a personal or corporate
    let account = account_repository::get_account(app_config, transaction.account).await;

    let account = match account {
        Ok(x) => x,
        Err(RepositoryError::AccountNotFound) => return Err(RepositoryError::AccountNotFound),
        _ => return Err(RepositoryError::UnexpectedError),
    };

    book_tickets(app_config, transaction).await?;

    transaction_repository::create_transaction(app_config, &transaction).await?;
    Ok(())
}

pub async fn get_all_transactions(
    app_config: &AppConfig,
) -> Result<Vec<Transaction>, RepositoryError> {
    Ok(transaction_repository::get_all_transactions(app_config).await?)
}

async fn book_tickets(
    app_config: &AppConfig,
    transaction: &Transaction,
) -> Result<(), RepositoryError> {
    // check ticketing service and book tickets

    let client = app_config.http_client.clone();

    let ticketing_service_endpoint = app_config.ticketing_service_host.clone();

    let payload = TicketDTO::transaction_to_dto(transaction)?;

    let payload = serde_json::to_string(&payload)?;

    let _response = client
        .post(ticketing_service_endpoint)
        .body(payload)
        .send()
        .await?;

    Ok(())
}

async fn get_ticket_qantity(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<i32, RepositoryError> {
    let client = app_config.http_client.clone();

    let transaction_id = transaction_id.to_string();

    let ticketing_service_endpoint = app_config.ticketing_service_host.clone()
        + "/ticket/transaction/"
        + &transaction_id.to_string()
        + "/quantity";

    let response = client
        .get(ticketing_service_endpoint)
        .send()
        .await?
        .text()
        .await?;

    let ticket_quantity = response.parse::<i32>()?;

    Ok(ticket_quantity)
}
