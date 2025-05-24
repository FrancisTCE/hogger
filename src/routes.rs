use axum::{routing::get, routing::post, Router, Extension};
use std::sync::Arc;

use crate::{controllers::hog_controller, services::hog_service::HogService};
use mongodb::Database;
use lapin::Channel;

type RabbitChannel = Channel; 

pub fn create_router(db: Database, rabbit_channel: RabbitChannel) -> Router {
    let hog_service = Arc::new(HogService::new(&db, rabbit_channel));

    Router::new()
        .route("/hogs", get(hog_controller::get_hogs))
        .route("/hogs", post(hog_controller::create_hog))
        .route("/hogs/statistics", get(hog_controller::hog_statistics))
        .route("/hogs/search", post(hog_controller::handle_search))
        .layer(Extension(hog_service))
}
