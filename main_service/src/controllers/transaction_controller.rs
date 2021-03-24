use super::models::transaction::TransactionDTO;
use super::services::transaction_service::create_transaction as service_create_transaction;
use super::services::transaction_service::get_all_transactions as service_get_all_transactions;
use super::services::transaction_service::get_transaction as service_get_transaction;
use crate::config::AppConfig;
use actix_web::{get, post, web, Error, HttpResponse};
use log::debug;
use serde_json::to_string;

#[post("/transaction")]
pub async fn create_transacton(
    transaction: web::Json<TransactionDTO>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let dto_transaction = transaction.into_inner();

    let transaction = dto_transaction.dto_to_transaction();

    service_create_transaction(app_config.get_ref(), &transaction).await?;
    //HttpResponse::Ok().body(to_string(&transaction))
    Ok(HttpResponse::Ok().body("transaction created!"))
}
