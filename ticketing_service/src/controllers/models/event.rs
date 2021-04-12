use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub id: i32,
    pub host: i32,
    pub event_name: String,
    pub maximum_ticket_capacity: i32,
}
