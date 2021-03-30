use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize)]
pub struct Ticket {
    pub id: Option<i32>,
    pub transaction_id: i32,
    pub quantity: Option<i32>,
}

#[derive(Debug, FromRow, Clone)]
pub struct TicketDB {
    #[sqlx(rename = "id")]
    pub id: i32,
    #[sqlx(rename = "transaction_id")]
    pub transaction_id: i32,
}

impl TicketDB {
    pub fn db_to_ticket(self) -> Ticket {
        Ticket {
            id: Some(self.id),
            transaction_id: self.transaction_id,
            quantity: None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TicketDTO {
    #[serde(rename(deserialize = "transaction"))]
    pub transaction_id: i32,
    #[serde(rename(deserialize = "quantity"))]
    pub quantity: i32,
}

impl TicketDTO {
    pub fn dto_to_ticket(self) -> Ticket {
        let new_ticket = Ticket {
            id: None,
            transaction_id: self.transaction_id,
            quantity: Some(self.quantity),
        };

        return new_ticket;
    }
}
