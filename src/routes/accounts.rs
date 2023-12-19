use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse, Responder, Scope,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::models::account::{create_account, delete_account, get_all, get_by_id};

use actix_web::web::Json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountRequest {
    account_name: String,
}

#[post("/")]
async fn create_account_handler(
    pool: web::Data<PgPool>,
    account_info: Json<AccountRequest>,
) -> impl Responder {
    match create_account(&pool, &account_info.account_name).await {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/")]
async fn get_accounts_handler(pool: web::Data<PgPool>) -> impl Responder {
    match get_all(&pool).await {
        Ok(accounts) => HttpResponse::Ok().json(accounts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}/")]
async fn delete_account_handler(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let account_id = path.into_inner();

    match delete_account(&pool, account_id).await {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(err) => match err {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Account not found"),
            err => {
                print!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[get("/{id}/")]
async fn get_account_handler(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match get_by_id(&pool, path.into_inner()).await {
        Ok(account) => HttpResponse::Ok().json(account),
        Err(err) => match err {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Account not found"),
            err => {
                print!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

#[get("/{id}/records/")]
async fn get_records_by_account_id_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let account_id = path.into_inner();
    match get_by_id(&pool, account_id).await {
        Ok(account) => {
            let records = crate::models::record::get_by_account_id(&pool, account.id).await;
            match records {
                Ok(records) => HttpResponse::Ok().json(records),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => HttpResponse::NotFound().json("Account not found"),
            err => {
                print!("{:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
    }
}

pub fn accounts_service() -> Scope {
    web::scope("/accounts")
        .service(create_account_handler)
        .service(get_accounts_handler)
        .service(delete_account_handler)
        .service(get_account_handler)
        .service(get_records_by_account_id_handler)
}
