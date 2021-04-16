use super::error::RepositoryError;
use super::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TicketDTO {
    pub transaction: i32,
    pub quantity: i32,
}

impl TicketDTO {
    pub fn transaction_to_dto(transaction: &Transaction) -> Result<TicketDTO, RepositoryError> {
        let transaction_id = transaction.id.ok_or(RepositoryError::UnexpectedError)?;
        let quantity = transaction
            .quantity
            .ok_or(RepositoryError::UnexpectedError)?;
        Ok(TicketDTO {
            transaction: transaction_id,
            quantity: quantity,
        })
    }
}
