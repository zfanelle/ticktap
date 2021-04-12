use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    pub id: i32,
    pub event: i32,
    pub account: i32,
    pub quantity: i32,
}
