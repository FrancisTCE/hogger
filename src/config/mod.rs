use bson::doc;
use mongodb::{options::ClientOptions, Client, Database};
use lapin::{Connection, ConnectionProperties, Channel};
use tokio::time::sleep;
use std::{env, time::Duration};

use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;

static INDEXED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub async fn init_db() -> mongodb::error::Result<Database> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    if !INDEXED.load(Ordering::SeqCst) {
        init_db_indexes(&client.database(&db_name)).await?;
        INDEXED.store(true, Ordering::SeqCst);
    }
    Ok(client.database(&db_name))
}

pub async fn init_db_indexes(db: &Database) -> mongodb::error::Result<()> {
    let collection = db.collection::<serde_json::Value>("hog");
    use mongodb::IndexModel;

    let indexes = vec![
        IndexModel::builder().keys(doc! { "hog_uuid": 1 }).build(),
        IndexModel::builder().keys(doc! { "hog_timestamp": -1 }).build(),
        IndexModel::builder().keys(doc! { "log_timestamp": -1 }).build(),
        IndexModel::builder().keys(doc! { "log_type": 1 }).build(),
        IndexModel::builder().keys(doc! { "log_source": 1, "log_timestamp": -1 }).build(),
    ];

    collection.create_indexes(indexes).await?;
    Ok(())
}

pub async fn init_rabbitmq() -> lapin::Result<Channel> {
    let uri = env::var("RABBITMQ_URI").expect("RABBITMQ_URI must be set");

    let max_retries = 15;
    let delay = Duration::from_secs(3);

    for attempt in 1..=max_retries {
        match Connection::connect(&uri, ConnectionProperties::default()).await {
            Ok(conn) => match conn.create_channel().await {
                Ok(channel) => {
                    println!("✅ Connected to RabbitMQ on attempt {}", attempt);
                    return Ok(channel);
                }
                Err(e) => {
                    eprintln!("❌ Failed to create channel (attempt {}): {}", attempt, e);
                }
            },
            Err(e) => {
                eprintln!("❌ RabbitMQ not ready (attempt {}): {}", attempt, e);
            }
        }
        sleep(delay).await;
    }

    panic!("❌ Failed to connect to RabbitMQ after {} attempts", max_retries);
}