use serde::Serialize;
use sqlx::{Executor, FromRow, PgPool, Postgres};

#[derive(Debug, FromRow, Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: f64,
    pub initial_balance: f64,
}

pub async fn create_account(pool: &PgPool, account_name: &str) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        "INSERT INTO accounts (name) VALUES ($1) RETURNING id, name, balance, initial_balance",
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
    let mut tx = pool.begin().await?;

    sqlx::query!("DELETE FROM records WHERE account_id = $1", account_id)
        .execute(&mut *tx)
        .await?;

    let account = sqlx::query_as!(
        Account,
        "DELETE FROM accounts WHERE id = $1 RETURNING id, name, balance, initial_balance",
        account_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(account)
}

pub async fn get_by_id(pool: &PgPool, account_id: i32) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(Account, "SELECT * FROM accounts WHERE id = $1", account_id)
        .fetch_one(pool)
        .await?;

    Ok(account)
}

pub async fn add_to_balance<'e, 'c, E>(
    executor: E,
    account_id: i32,
    amount: f64,
) -> Result<Account, sqlx::Error>
where
    E: 'e + Executor<'c, Database = Postgres>,
{
    let account = sqlx::query_as!(
        Account,
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2 RETURNING id, name, balance, initial_balance",
        amount,
        account_id
    )
    .fetch_one(executor)
    .await?;

    Ok(account)
}
