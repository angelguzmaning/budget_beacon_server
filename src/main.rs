use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::accounts::accounts_service;
use sqlx::postgres::PgPoolOptions;

mod models;
mod routes;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(health_check)
            .service(accounts_service())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
