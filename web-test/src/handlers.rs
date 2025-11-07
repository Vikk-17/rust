use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
};
use serde_json::json;
use crate::models::UserInput;
use crate::{AppState, AppStateWithMutex};

#[get("/testing")]
pub async fn testing() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Testing ..."
    }))
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Index page ..."
    }))
}

pub async fn route_test() -> impl Responder {
     HttpResponse::Ok().json(json!({
        "message": "Random Route testing ..."
    }))
}

#[post("/post")]
pub async fn post_something(req_body: web::Json<UserInput>) -> impl Responder {
    HttpResponse::Ok().json(req_body)
}

#[post("/json")]
pub async fn json_test(req_body: web::Json<UserInput>) -> web::Json<UserInput> {
    let body = req_body.into_inner();
    web::Json(UserInput{
        title: body.title,
        body: body.body,
    })
}

#[get("/appstate")]
pub async fn appstate(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("The name of the app: {app_name}")
}

#[get("/counter")]
pub async fn get_counter(data: web::Data<AppStateWithMutex>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("The counter value is: {counter}")
}

