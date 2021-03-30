use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;
use crate::controllers::models::transaction::{Transaction, TransactionDB};

pub async fn create_transaction(
    app_config: &AppConfig,
    transaction: &Transaction,
) -> Result<(), RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "INSERT INTO transaction (`event`, `account`)
    VALUES(?, ?)";

    sqlx::query(sql)
        .bind(&transaction.event)
        .bind(&transaction.account)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_transaction(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<Transaction, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * FROM transaction WHERE id = ?";

    let transaction: TransactionDB = sqlx::query_as(sql)
        .bind(transaction_id)
        .fetch_one(&pool)
        .await?;

    Ok(transaction.db_to_transaction())
}

pub async fn get_all_transactions(
    app_config: &AppConfig,
) -> Result<Vec<Transaction>, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * from transaction";

    let transactions: Vec<TransactionDB> = sqlx::query_as(sql).fetch_all(&pool).await?;

    let transactions: Vec<Transaction> = transactions
        .iter()
        .cloned()
        .map(|e| e.db_to_transaction())
        .collect();

    Ok(transactions)
}
