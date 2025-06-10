mod config;
mod models;

use futures::StreamExt;
use governor::{Quota, RateLimiter};
use lapin::{message::Delivery, options::*, types::FieldTable};
use models::hog::Hog;
use mongodb::Collection;
use serde_json;
use std::num::NonZeroU32;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let channel = config::init_rabbitmq()
        .await
        .expect("Failed to connect to RabbitMQ");

    channel
        .basic_qos(1, BasicQosOptions::default())
        .await
        .expect("Failed to set QoS");

    channel
        .queue_declare(
            "hog_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let mut consumer = channel
        .basic_consume(
            "hog_queue",
            "hog_worker_single",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let db = config::init_db()
        .await
        .expect("Failed to connect to MongoDB");

    config::init_db_indexes(&db)
        .await
        .expect("Failed to initialize MongoDB indexes");

    println!("✅ Connected to MongoDB and initialized indexes");

    let collection: Collection<Hog> = db.collection("hog");
    let limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(400).unwrap()));

    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                limiter.until_ready().await;
                if let Err(e) = process_message(&collection, delivery).await {
                    eprintln!("Error processing message: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to consume message: {:?}", e);
            }
        }
    }
    eprintln!("Consumer stream ended, exiting with error code 1...");
    std::process::exit(1);
}

async fn process_message(collection: &Collection<Hog>, delivery: Delivery) -> anyhow::Result<()> {
    let hog: Hog = serde_json::from_slice(&delivery.data)?;

    const MAX_RETRIES: usize = 5;
    for attempt in 1..=MAX_RETRIES {
        match collection.insert_one(&hog).await {
            Ok(_) => {
                delivery.ack(BasicAckOptions::default()).await?;
                return Ok(());
            }
            Err(e) => {
                eprintln!("Attempt {}: Failed to insert document: {:?}", attempt, e);
                if attempt == MAX_RETRIES {
                    delivery.nack(BasicNackOptions::default()).await?;
                    return Err(e.into());
                } else {
                    sleep(Duration::from_millis(500 * attempt as u64)).await;
                }
            }
            
        }
    }

    Ok(())
}
