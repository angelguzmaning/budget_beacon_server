use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub id: i32,
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
    pub account_id: i32,
}

pub async fn create_record(
    pool: &PgPool,
    date: NaiveDate,
    description: &str,
    amount: f64,
    account_id: i32,
) -> Result<Record, sqlx::Error> {
    let record = sqlx::query_as!(
    Record,
    "INSERT INTO records (date, description, amount, account_id) VALUES ($1, $2, $3, $4) RETURNING id, date, description, amount, account_id",
    date,
    description,
    amount,
    account_id
  )
  .fetch_one(pool)
  .await?;

    Ok(record)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(Record, "SELECT * FROM records")
        .fetch_all(pool)
        .await?;

    Ok(records)
}

pub async fn get_by_account_id(pool: &PgPool, account_id: i32) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(
        Record,
        "SELECT * FROM records WHERE account_id = $1",
        account_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}
