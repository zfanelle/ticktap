use super::super::models::account::Account;
use super::repositories::account_repository;
use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;

pub async fn get_account(
    app_config: &AppConfig,
    account_id: String,
) -> Result<Account, RepositoryError> {
    let account_id = account_id.parse()?;
    let account = account_repository::get_account(app_config, account_id).await?;

    Ok(account.convert_to_account())
}

pub async fn create_account(
    app_config: &AppConfig,
    account: &Account,
) -> Result<(), RepositoryError> {
    account_repository::create_account(app_config, &account).await?;
    Ok(())
}

pub async fn get_all_accounts(app_config: &AppConfig) -> Result<Vec<Account>, RepositoryError> {
    let db_accounts = account_repository::get_all_accounts(app_config).await?;

    let accounts: Vec<Account> = db_accounts
        .iter()
        .cloned()
        .map(|e| e.convert_to_account())
        .collect();

    Ok(accounts)
}
