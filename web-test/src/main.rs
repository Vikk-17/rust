mod handlers;
mod models;

use std::sync::Mutex;
use actix_web::{
    App, HttpServer, Responder, get, web
};
use handlers::*;

#[allow(unused)]
struct AppState {
    app_name: String,
    author: String,
    year: u16,
}

struct AppStateWithMutex {
    counter: Mutex<i32>,
}

#[get("/appstate")]
async fn appstate(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("The name of the app: {app_name}")
}

#[get("/counter")]
async fn get_counter(data: web::Data<AppStateWithMutex>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("The counter value is: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    println!("Server has been started on http://localhost:8080/");
    let counter = web::Data::new(AppStateWithMutex {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState {
            app_name: String::from("Actix Web"),
            author: String::from("John"),
            year: 2025,
        }))
        .app_data(counter.clone())
        .service(get_counter)
        .service(appstate)
        .service(testing)
        .service(index)
        .route("/route", web::get().to(route_test))
        .service(post_something)
        .service(json_test)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
