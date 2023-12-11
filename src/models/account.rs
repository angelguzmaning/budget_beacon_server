use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
}

pub async fn create_account(pool: &PgPool, account_name: &str) -> Result<Account, sqlx::Error> {
  let account = sqlx::query_as!(
      Account,
      "INSERT INTO accounts (name) VALUES ($1) RETURNING id, name",
      account_name
  )
  .fetch_one(pool)
  .await?;

  Ok(account)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
  let accounts = sqlx::query_as!(Account, "SELECT * FROM accounts")
      .fetch_all(pool)
      .await?;

  Ok(accounts)
}

pub async fn delete_account(pool: &PgPool, account_id: i32) -> Result<Account, sqlx::Error> {
  let account = sqlx::query_as!(
      Account,
      "DELETE FROM accounts WHERE id = $1 RETURNING id, name",
      account_id
  )
  .fetch_one(pool)
  .await?;

  Ok(account)
}

pub async fn get_by_id(pool: &PgPool, account_id: i32) -> Result<Account, sqlx::Error> {
  let account = sqlx::query_as!(
      Account,
      "SELECT * FROM accounts WHERE id = $1",
      account_id
  )
  .fetch_one(pool)
  .await?;

  Ok(account)
}