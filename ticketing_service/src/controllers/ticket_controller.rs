use actix_web::{get, post, web, Error, HttpResponse};
use log::debug;

#[get("/test")]
pub async fn test() -> HttpResponse {
    debug!("logging works!");
    HttpResponse::Ok().body("Ticktap is up and running!")
}
