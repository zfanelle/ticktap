use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub id: Option<i32>,
    pub event: i32,
    pub account: i32,
    pub quantity: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionDTO {
    #[serde(rename(deserialize = "event"))]
    pub event: i32,
    #[serde(rename(deserialize = "account"))]
    pub account: i32,
    #[serde(rename(deserialize = "quantity"))]
    pub quantity: i32,
}

impl TransactionDTO {
    pub fn dto_to_transaction(self) -> Transaction {
        Transaction {
            id: None,
            event: self.event,
            account: self.account,
            quantity: Some(self.quantity),
        }
    }
}

#[derive(Debug, FromRow, Clone)]
pub struct TransactionDB {
    #[sqlx(rename = "id")]
    pub id: i32,
    #[sqlx(rename = "event")]
    pub event: i32,
    #[sqlx(rename = "account")]
    pub account: i32,
}

impl TransactionDB {
    pub fn db_to_transaction(self) -> Transaction {
        Transaction {
            id: Some(self.id),
            event: self.event,
            account: self.account,
            quantity: None,
        }
    }
}
