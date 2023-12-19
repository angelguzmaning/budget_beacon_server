use actix_web::{
    delete, get, post,
    web::{self, Json},
    Responder, Scope,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::record::{create_record, delete_record, get_all};
use actix_web::HttpResponse;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionRecordRequest {
    date: NaiveDate,
    description: String,
    amount: f64,
    account_id: i32,
}

#[post("/")]
async fn create_record_handler(
    pool: web::Data<PgPool>,
    record_info: Json<TransactionRecordRequest>,
) -> impl Responder {
    match create_record(
        &pool,
        record_info.date,
        &record_info.description,
        record_info.amount,
        record_info.account_id,
    )
    .await
    {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/")]
async fn get_records_handler(pool: web::Data<PgPool>) -> impl Responder {
    match get_all(&pool).await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}/")]
async fn delete_record_handler(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let record_id = path.into_inner();

    match delete_record(&pool, record_id).await {
        Ok(record) => HttpResponse::Ok().json(record),
        Err(err) => match err {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Record not found"),
            err => {
                print!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

pub fn records_service() -> Scope {
    web::scope("/records")
        .service(create_record_handler)
        .service(get_records_handler)
        .service(delete_record_handler)
}
