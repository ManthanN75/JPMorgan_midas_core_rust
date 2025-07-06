use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Transaction {
    sender_id: String,
    recipient_id: String,
    amount: f64,
}

#[derive(Debug, Serialize)]
struct Incentive {
    amount: f64,
}

#[post("/incentive")]
async fn get_incentive(transaction: web::Json<Transaction>) -> impl Responder {
    println!("Received transaction: {:?}", transaction);

    // Generate a dummy incentive value (you can customize logic)
    let incentive_amount = (transaction.amount * 0.1).min(15.0); // Max ₹15

    HttpResponse::Ok().json(Incentive {
        amount: incentive_amount,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Mock Incentive API running at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(get_incentive)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
