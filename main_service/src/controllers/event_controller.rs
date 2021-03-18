use super::models::event::EventDTO;
use super::services::event_service::create_event as service_create_event;
use super::services::event_service::get_all_events as service_get_all_events;
use super::services::event_service::get_event as service_get_event;
use crate::config::AppConfig;
use actix_web::{get, post, web, Error, HttpResponse};
use log::debug;
use serde_json::to_string;

#[get("/event/{event_id}")]
pub async fn get_event(
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let event_id = path.into_inner();
    let response_body = service_get_event(app_config.get_ref(), event_id).await?;

    let response_string = to_string(&response_body)?;
    Ok(HttpResponse::Ok().body(response_string))
}

#[post("/event")]
pub async fn create_event(
    event: web::Json<EventDTO>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, Error> {
    let dto_event = event.into_inner();

    let event = dto_event.dto_to_event();

    service_create_event(app_config.get_ref(), &event).await?;
    //HttpResponse::Ok().body(to_string(&event))
    Ok(HttpResponse::Ok().body("event created!"))
}

#[get("/event")]
pub async fn get_events(app_config: web::Data<AppConfig>) -> Result<HttpResponse, Error> {
    let events = service_get_all_events(app_config.get_ref()).await?;

    let response_string = to_string(&events)?;

    Ok(HttpResponse::Ok().body(response_string))
}
