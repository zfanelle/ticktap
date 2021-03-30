use super::super::models::transaction::Transaction;
use super::repositories::{account_repository, transaction_repository};
use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;

pub async fn get_transaction(
    app_config: &AppConfig,
    transaction_id: String,
) -> Result<Transaction, RepositoryError> {
    let transaction_id = transaction_id.parse()?;

    Ok(transaction_repository::get_transaction(app_config, transaction_id).await?)
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

    // check ticketing service and book tickets

    transaction_repository::create_transaction(app_config, &transaction).await?;
    Ok(())
}

pub async fn get_all_transactions(
    app_config: &AppConfig,
) -> Result<Vec<Transaction>, RepositoryError> {
    Ok(transaction_repository::get_all_transactions(app_config).await?)
}
