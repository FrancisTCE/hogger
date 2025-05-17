use mongodb::{options::ClientOptions, Client, Database};
use lapin::{Connection, ConnectionProperties, Channel};
use tokio::time::sleep;
use std::{env, time::Duration};

pub async fn init_db() -> mongodb::error::Result<Database> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database(&db_name))
}

pub async fn init_rabbitmq() -> lapin::Result<Channel> {
    let uri = env::var("RABBITMQ_URI").expect("RABBITMQ_URI must be set");

    let max_retries = 10;
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