mod handlers;
mod model;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use anyhow::{Result, Error};
use handlers::*;

#[actix_web::main]
async fn main() -> Result<(), Error> {

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
            .service(signup)
            .service(signin)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await;

    Ok(())
}
