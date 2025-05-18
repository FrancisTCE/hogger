mod config;
mod models;

use futures::StreamExt;
use lapin::{options::*, types::FieldTable};
use models::hog::Hog;
use mongodb::Collection;
use serde_json;

#[tokio::main]
async fn main() {
    let channel = config::init_rabbitmq().await.expect("Failed to connect to RabbitMQ");

    channel.queue_declare("hog_queue", QueueDeclareOptions::default(), FieldTable::default()).await.unwrap();

    let mut consumer = channel.basic_consume("hog_queue", "hog_worker", BasicConsumeOptions::default(), FieldTable::default()).await.unwrap();

    let db = config::init_db().await.expect("Failed to connect to MongoDB");
    let collection: Collection<Hog> = db.collection("hog");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let hog: Hog = serde_json::from_slice(&delivery.data).unwrap();
            println!("Inserting hog into MongoDB: {:?}", hog);
            collection.insert_one(hog).await.unwrap();
            delivery.ack(BasicAckOptions::default()).await.unwrap();
        }
    }
}


