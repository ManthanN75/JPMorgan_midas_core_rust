use std::env;
use tokio_stream::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use sqlx::{Pool, Sqlite};
use crate::{transaction::Transaction, db};
use serde_json;

pub async fn start_kafka_consumer(pool: Pool<Sqlite>) {
    let broker = env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set");
    let topic = env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set");
    let group_id = env::var("KAFKA_GROUP_ID").expect("KAFKA_GROUP_ID must be set");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &broker)
        .set("group.id", &group_id)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create Kafka consumer");

    consumer.subscribe(&[&topic]).expect("Failed to subscribe to topic");
    println!("🟢 Kafka consumer started on topic '{}'", topic);

    let mut message_stream = consumer.stream();

    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(msg) => handle_message(msg, &pool).await,
            Err(e) => eprintln!("❌ Kafka error: {}", e),
        }
    }
}

async fn handle_message(msg: BorrowedMessage<'_>, pool: &Pool<Sqlite>) {

    match msg.payload_view::<str>() {
        Some(Ok(payload)) => {
            match serde_json::from_str::<Transaction>(payload) {
                Ok(tx) => {
                    println!("✅ Parsed Transaction: {:?}", tx);
                   if let Err(e) = db::process_transaction(pool, &tx).await {
                        eprintln!("❌ DB insert error: {}", e);
                    }
                }
                Err(e) => println!("❌ JSON parse error: {}", e),
            }
        }
        Some(Err(e)) => println!("❌ Failed to decode payload: {}", e),
        None => println!("⚠️ No payload in Kafka message"),
    }
}
