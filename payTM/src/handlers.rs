use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Testing")
}

#[get("/index")]
pub async fn index(db: web::Data<sqlx::PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db.get_ref())
        .await
        .unwrap_or((0,));
    HttpResponse::Ok().json(json!({
        "Total User": row.0
    }))
}

// #[post("/signup")]
// async fn signup(body: web::Json<SingupInput>) -> impl Responder {
//
// }


