use crate::config::AppConfig;
use crate::controllers::models::account::{Account, AccountDB};
use crate::controllers::models::error::RepositoryError;

pub async fn create_account(
    app_config: &AppConfig,
    account: &Account,
) -> Result<(), RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "INSERT INTO account (`entity_name`, `entity_type`)
    VALUES(?, ?)";

    sqlx::query(sql)
        .bind(&account.entity_name)
        .bind(&account.entity_type)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_account(
    app_config: &AppConfig,
    account_id: u32,
) -> Result<AccountDB, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * FROM account WHERE id = ?";

    let account: AccountDB = sqlx::query_as(sql)
        .bind(account_id)
        .fetch_one(&pool)
        .await?;

    Ok(account)
}

pub async fn get_all_accounts(app_config: &AppConfig) -> Result<Vec<AccountDB>, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * from account";

    let accounts: Vec<AccountDB> = sqlx::query_as(sql).fetch_all(&pool).await?;

    Ok(accounts)
}
