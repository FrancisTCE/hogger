mod config;
mod models;
mod services;
mod controllers;
mod routes;

use dotenv::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = config::init_db().await.expect("Failed to connect to DB");
    let rabbit_channel = config::init_rabbitmq().await.expect("Failed to connect to RabbitMQ");

    let app = routes::create_router(db, rabbit_channel);

    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running at http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
