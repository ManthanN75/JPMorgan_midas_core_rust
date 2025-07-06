mod db;
mod routes;
mod kafka;
mod transaction; 


use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use log::info;

use routes::get_balance;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

   let pool = db::connect().await.expect("Failed to connect to database");
   tokio::spawn(kafka::start_kafka_consumer(pool.clone()));


    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);
    info!("🚀 Server starting at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_balance)
    })

    .bind(&addr)
    .map_err(|e| {
        eprintln!("Failed to bind server to {}: {}", addr, e);
        e
    })?
    .run()
    .await

}
