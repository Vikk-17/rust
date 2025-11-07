mod handlers;
mod models;
mod state;

use std::sync::Mutex;
use actix_web::{
    App,
    HttpServer,
    web,
    middleware::Logger,
};
use handlers::*;
use env_logger::Env;
use crate::state::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // env_logger::init();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Server has been started on http://localhost:8080/");
    let counter = web::Data::new(AppStateWithMutex {
        counter: Mutex::new(0)
    });

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
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
    .workers(4) // <- by default 8 or the number of physical cpu in the system
    .run()
    .await
}
