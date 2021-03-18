use super::super::models::event::Event;
use super::repositories::event_repository;
use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;

pub async fn create_event(app_config: &AppConfig, event: &Event) -> Result<(), RepositoryError> {
    event_repository::create_event(app_config, &event).await?;
    Ok(())
}

pub async fn get_event(app_config: &AppConfig, event_id: String) -> Result<Event, RepositoryError> {
    let event_id = event_id.parse()?;
    let event = event_repository::get_event(app_config, event_id).await?;

    Ok(event.db_to_event())
}

pub async fn get_all_events(app_config: &AppConfig) -> Result<Vec<Event>, RepositoryError> {
    let db_events = event_repository::get_all_events(app_config).await?;

    let events: Vec<Event> = db_events.iter().cloned().map(|e| e.db_to_event()).collect();

    Ok(events)
}
