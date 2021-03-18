use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;
use crate::controllers::models::event::{Event, EventDB};

pub async fn create_event(app_config: &AppConfig, event: &Event) -> Result<(), RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "INSERT INTO event (`host`, `event_name`, `maximum_ticket_capacity`)
    VALUES(?, ?, ?)";

    sqlx::query(sql)
        .bind(&event.host)
        .bind(&event.event_name)
        .bind(&event.maximum_ticket_capacity)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_event(app_config: &AppConfig, event_id: i32) -> Result<EventDB, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * FROM event WHERE id = ?";

    let event: EventDB = sqlx::query_as(sql).bind(event_id).fetch_one(&pool).await?;

    Ok(event)
}

pub async fn get_all_events(app_config: &AppConfig) -> Result<Vec<EventDB>, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * from event";

    let events: Vec<EventDB> = sqlx::query_as(sql).fetch_all(&pool).await?;

    Ok(events)
}
