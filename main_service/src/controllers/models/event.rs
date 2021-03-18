use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: Option<i32>,
    pub host: i32,
    pub event_name: String,
    pub maximum_ticket_capacity: i32,
}

#[derive(Deserialize, Serialize)]
pub struct EventDTO {
    #[serde(rename(deserialize = "eventName"))]
    pub event_name: String,
    #[serde(rename(deserialize = "host"))]
    pub host: i32,
    #[serde(rename(deserialize = "maximumTicketCapacity"))]
    pub maximum_ticket_capacity: i32,
}

impl EventDTO {
    pub fn dto_to_event(self) -> Event {
        Event {
            id: None,
            host: self.host,
            event_name: self.event_name,
            maximum_ticket_capacity: self.maximum_ticket_capacity,
        }
    }
}

#[derive(Debug, FromRow, Clone)]
pub struct EventDB {
    #[sqlx(rename = "id")]
    pub id: i32,
    #[sqlx(rename = "host")]
    pub host: i32,
    #[sqlx(rename = "event_name")]
    pub event_name: String,
    #[sqlx(rename = "maximum_ticket_capacity")]
    pub maximum_ticket_capacity: i32,
}

impl EventDB {
    pub fn db_to_event(self) -> Event {
        Event {
            id: Some(self.id),
            host: self.host,
            event_name: self.event_name,
            maximum_ticket_capacity: self.maximum_ticket_capacity,
        }
    }
}
