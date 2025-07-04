use axum::{Extension, Router, routing::get, routing::post};
use std::sync::Arc;

use crate::{controllers::hog_controller, services::hog_service::HogService};
use lapin::Channel;
use mongodb::Database;

type RabbitChannel = Channel;

pub fn create_router(db: Database, rabbit_channel: RabbitChannel) -> Router {
    let hog_service = Arc::new(HogService::new(&db, rabbit_channel));

    Router::new()
        .route("/metrics", get(hog_controller::metrics))
        .route("/hogs", get(hog_controller::get_hogs))
        .route("/hogs", post(hog_controller::create_hog))
        .route("/hogs/search", post(hog_controller::handle_search))
        .route("/hogs/statistics", get(hog_controller::hog_statistics))
        .route("/hogs/stats", get(hog_controller::hog_stats))
        .layer(Extension(hog_service))
}
