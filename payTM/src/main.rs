use env_logger::Builder;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Postgres, Row};
use dotenvy::dotenv;
use anyhow::{Result, Error};
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
struct SingupInput {
    username: String,
    password: String,
    firstname: String,
    lastname: String,
}

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Testing")
}

#[get("/index")]
async fn index(db: web::Data<sqlx::PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db.get_ref())
        .await
        .unwrap_or((0,));
    HttpResponse::Ok().json(json!({
        "Total User": row.0
    }))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {

    // Builder::from_env("RUST_LOG").init();
    env_logger::init();
    dotenv().ok();

    let db_uri = std::env::var("DATABASE_URL").expect("Invalid DATABASE_URL");

    // 1. Create the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_uri)
        .await
        .expect("Pool failed");

    let pool_data = web::Data::new(pool);

    println!("Server is running on http://localhost:8080");

    let _ = HttpServer::new( move || {
        App::new()
            .app_data(pool_data.clone())
            .service(test)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await;

    Ok(())
}
