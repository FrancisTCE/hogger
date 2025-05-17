use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn init_db() -> mongodb::error::Result<Database> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database(&db_name))
}