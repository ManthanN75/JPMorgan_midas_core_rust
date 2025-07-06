use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize)]
struct Balance {
    balance: f64,
}

#[get("/balance")]
async fn get_balance(
    db_pool: web::Data<SqlitePool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let user_id = match query.get("userId") {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Missing userId parameter"),
    };

    let result: Option<(f64,)> = sqlx::query_as("SELECT balance FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(db_pool.get_ref())
        .await
        .unwrap_or(None);

    let balance = result.map(|(b,)| b).unwrap_or(0.0);

    HttpResponse::Ok().json(Balance { balance })
}


pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(get_balance);
}
