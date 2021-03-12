use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub id: Option<i32>,
    pub entity_name: String,
    pub entity_type: String,
}

#[derive(Debug, FromRow, Clone)]
pub struct AccountDB {
    #[sqlx(rename = "id")]
    pub id: i32,
    #[sqlx(rename = "entity_name")]
    pub entity_name: String,
    #[sqlx(rename = "entity_type")]
    pub entity_type: String,
}

impl AccountDB {
    pub fn convert_to_account(self) -> Account {
        Account {
            id: Some(self.id),
            entity_name: self.entity_name,
            entity_type: self.entity_type,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountDTO {
    pub entity_name: String,
    pub entity_type: String,
}

impl AccountDTO {
    pub fn dto_to_account(self) -> Account {
        let new_account = Account {
            id: None,
            entity_name: self.entity_name.clone(),
            entity_type: self.entity_type.clone(),
        };

        return new_account;
    }
}
