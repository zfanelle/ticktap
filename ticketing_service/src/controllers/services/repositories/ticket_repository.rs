use crate::config::AppConfig;
use crate::controllers::models::error::RepositoryError;
use crate::controllers::models::ticket::{Ticket, TicketDB};
use sqlx::Row;

pub async fn create_ticket(app_config: &AppConfig, ticket: &Ticket) -> Result<(), RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "INSERT INTO ticket (`transaction_id`)
    VALUES(?)";

    sqlx::query(sql)
        .bind(&ticket.transaction_id)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_ticket(app_config: &AppConfig, ticket_id: u32) -> Result<Ticket, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * FROM ticket WHERE id = ?";

    let mut ticket: Vec<TicketDB> = sqlx::query_as(sql).bind(ticket_id).fetch_all(&pool).await?;

    if let Some(i) = ticket.pop() {
        return Ok(i.db_to_ticket());
    } else {
        return Err(RepositoryError::TicketNotFound);
    }
}

pub async fn get_all_tickets(app_config: &AppConfig) -> Result<Vec<Ticket>, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * from ticket";

    let tickets: Vec<TicketDB> = sqlx::query_as(sql).fetch_all(&pool).await?;

    let tickets: Vec<Ticket> = tickets.iter().cloned().map(|e| e.db_to_ticket()).collect();

    Ok(tickets)
}

pub async fn check_ticket_quantity(app_config: &AppConfig) -> Result<i32, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT Count(*)
    FROM   `ticktap`.`ticket` t
           JOIN `ticktap`.`transaction` tr
             ON ( tr.`id` = t.`transaction` )
           JOIN `ticktap`.`event` e
             ON ( tr.`event` = e.`id`";

    let ticket_count = sqlx::query(sql).fetch_one(&pool).await?;

    let ticket_count: i32 = ticket_count.get(0);

    Ok(ticket_count)
}

pub async fn get_all_tickets_by_transaction(
    app_config: &AppConfig,
    transaction_id: i32,
) -> Result<Vec<Ticket>, RepositoryError> {
    let pool = app_config.db_pool.clone();

    let sql = "SELECT * from `ticktap`.`ticket` WHERE transaction = ?";

    let tickets: Vec<TicketDB> = sqlx::query_as(sql)
        .bind(transaction_id)
        .fetch_all(&pool)
        .await?;

    let tickets: Vec<Ticket> = tickets.iter().cloned().map(|e| e.db_to_ticket()).collect();

    Ok(tickets)
}
