use super::models::account::AccountDTO;
use super::services::account_service::create_account as service_create_account;
use super::services::account_service::get_account as service_get_account;
use super::services::account_service::get_all_accounts as service_get_all_accounts;
use crate::config::AppConfig;
use actix_web::{get, post, web, Error, HttpResponse};
use log::debug;
use serde_json::to_string;

#[get("/account/{account_id}")]
pub async fn get_account(path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let account_id = path.into_inner();
    let response_body = service_get_account(app_config.get_ref(), account_id).await?;

    let response_string = to_string(&response_body)?;
    Ok(HttpResponse::Ok().body(response_string))
}

#[post("/account")]
pub async fn create_account(
    account: web::Json<AccountDTO>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let dto_account = account.into_inner();

    let account = dto_account.dto_to_account();

    service_create_account(app_config.get_ref(), &account).await?;
    //HttpResponse::Ok().body(to_string(&account))
    Ok(HttpResponse::Ok().body("Account created!"))
}

#[get("/account")]
pub async fn get_accounts(app_config: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let accounts = service_get_all_accounts(app_config.get_ref()).await?;

    let response_string = to_string(&accounts)?;

    Ok(HttpResponse::Ok().body(response_string))
}

#[get("/test")]
pub async fn test() -> HttpResponse {
    debug!("logging works!");
    HttpResponse::Ok().body("Ticktap is up and running!")
}
