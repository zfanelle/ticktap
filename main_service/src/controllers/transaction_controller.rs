use super::models::transaction::TransactionDTO;
use super::services::transaction_service::create_transaction as service_create_transaction;
use super::services::transaction_service::get_all_transactions as service_get_all_transactions;
use super::services::transaction_service::get_transaction as service_get_transaction;
use crate::config::AppConfig;
use actix_web::{get, post, web, Error, HttpResponse};
use serde_json::to_string;

#[post("/transaction")]
pub async fn create_transaction(
    transaction: web::Json<TransactionDTO>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let dto_transaction = transaction.into_inner();

    let transaction = dto_transaction.dto_to_transaction();

    service_create_transaction(app_config.get_ref(), &transaction).await?;
    //HttpResponse::Ok().body(to_string(&transaction))
    Ok(HttpResponse::Ok().body("transaction created!"))
}

#[get("/transaction/{transaction_id}")]
pub async fn get_transaction(
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let transaction_id = path.into_inner();
    let response_body = service_get_transaction(app_config.get_ref(), transaction_id).await?;

    let response_string = to_string(&response_body)?;
    Ok(HttpResponse::Ok().body(response_string))
}

#[get("/transaction")]
pub async fn get_transactions(app_config: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let transactions = service_get_all_transactions(app_config.get_ref()).await?;

    let response_string = to_string(&transactions)?;

    Ok(HttpResponse::Ok().body(response_string))
}
