use actix_web::{
    web::{Data, Json},
    get,
    post,
    HttpResponse,
    Responder,
    Result,
    Error
};
use sqlx::Row;
use serde_json::json;
use crate::model::{SingupInput, UserOut};

#[get("/")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Testing")
}

#[get("/index")]
pub async fn index(db: Data<sqlx::PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db.get_ref())
        .await
        .unwrap_or((0,));
    HttpResponse::Ok().json(json!({
        "Total User": row.0
    }))
}

/// POST /signup
/// Body JSON: { "username": "...", "password": "...", "firstname": "...", "lastname": "..." }
#[post("/signup")]
pub async fn signup(db: Data<sqlx::PgPool>, body: Json<SingupInput>) -> Result<HttpResponse, Error> {
    let input = body.into_inner();
    
    let row = sqlx::query(
        r#"
        INSERT INTO users (username, password, firstname, lastname)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, firstname, lastname
        "#
    )
        .bind(&input.username)
        .bind(&input.password)
        .bind(&input.firstname)
        .bind(&input.lastname)
        .fetch_one(db.get_ref())
        .await;

    match row {
        Ok(r) => {
           let user = UserOut {
               id: r.get("id"),
               username: r.get("username"),
               firstname: r.get("firstname"),
               lastname: r.get("lastname"),
           };
           Ok(HttpResponse::Created().json(json!({"status":"ok","user": user})))
        }
        Err(e) => {
            eprintln!("DB insert error: {}", e);
            Ok(HttpResponse::InternalServerError().json(json!({"error": format!("Db error{}", e)})))
        }
    }

}
