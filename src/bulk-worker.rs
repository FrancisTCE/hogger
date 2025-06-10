mod config;
mod models;

use futures::StreamExt;
use governor::{Quota, RateLimiter};
use lapin::{message::Delivery, options::*, types::FieldTable};
use models::hog::Hog;
use mongodb::Collection;
use serde_json;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};
use tokio::time::{interval, sleep};

const BULK_SIZE: usize = 1000;
const TIMING_THRESHOLD_SECS: u64 = 1;
const MAX_RETRIES: usize = 5;

#[tokio::main]
async fn main() {
    let channel = config::init_rabbitmq()
        .await
        .expect("Failed to connect to RabbitMQ");

    channel
        .basic_qos(1000, BasicQosOptions::default())
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
            "hog_worker_bulk",
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

    let mut bulk_order = Vec::with_capacity(BULK_SIZE);
    let mut bulk_acks = Vec::with_capacity(BULK_SIZE);
    let mut _timing_start = Instant::now();

    let mut flush_interval = interval(Duration::from_secs(TIMING_THRESHOLD_SECS));
    loop {
        tokio::select! {
                delivery_result = consumer.next() => {
                    match delivery_result {
                        Some(Ok(delivery)) => {
                            match serde_json::from_slice::<Hog>(&delivery.data) {
                                Ok(hog) => {
                                    bulk_order.push(hog);
                                    bulk_acks.push(delivery);
                                }
                                Err(e) => {
                                    eprintln!("Failed to deserialize Hog: {:?}", e);
                                    continue;
                                }
                            }
                            if bulk_order.len() >= BULK_SIZE {
                                limiter.until_ready().await;
                                if let Err(e) = process_message(&collection, &bulk_acks, &bulk_order).await {
                                    eprintln!("Error processing message batch: {:?}", e);
                                }
                                bulk_acks.clear();
                                bulk_order.clear();
                                _timing_start = Instant::now();
                            }
                        }
                        Some(Err(e)) => {
                            eprintln!("Failed to consume message: {:?}", e);
                        }
                        None => {
                            println!("Consumer stream ended, exiting...");
                            std::process::exit(1);
                        },
                    }
                }
                _ = flush_interval.tick() => {
                    if !bulk_order.is_empty() {
                        limiter.until_ready().await;
                        if let Err(e) = process_message(&collection, &bulk_acks, &bulk_order).await {
                            eprintln!("Error processing message batch: {:?}", e);
                        }
                        bulk_acks.clear();
                        bulk_order.clear();
                        _timing_start = Instant::now();
                    }
                }
            }
    }
}

async fn process_message(
    collection: &Collection<Hog>,
    deliveries: &[Delivery],
    hogs: &[Hog],
) -> anyhow::Result<()> {
    for attempt in 1..=MAX_RETRIES {
        match collection.insert_many(hogs).await {
            Ok(_) => {
                for d in deliveries {
                    if let Err(e) = d.ack(BasicAckOptions::default()).await {
                        eprintln!("Failed to ack message: {:?}", e);
                    } else {
                    }
                }
                return Ok(());
            }
            Err(e) => {
                eprintln!("Attempt {}: Failed to insert documents: {:?}", attempt, e);
                if attempt == MAX_RETRIES {
                    for d in deliveries {
                        if let Err(e) = d.nack(BasicNackOptions::default()).await {
                            eprintln!("Failed to nack message: {:?}", e);
                        }
                    }
                    return Err(e.into());
                }
                sleep(Duration::from_millis(500 * attempt as u64)).await;
            }
        }
    }
    Ok(())
}
