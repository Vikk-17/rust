use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;
// use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: String,
    title:String,
    completed: bool,
}


struct AppState {
    todos: Mutex<HashMap<String, Todo>>,
}


// Handlers
// GET /todos
async fn list_todos(data: web::Data<AppState>) -> impl Responder {
    let map = data.todos.lock().unwrap();
    let todos: Vec<Todo> = map.values().cloned().collect();
    HttpResponse::Ok().json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        todos: Mutex::new(HashMap::new()),
    });
}
