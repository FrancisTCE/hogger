use axum::{routing::get, routing::post, Extension, Router};
use std::sync::Arc;

use crate::{controllers::hog_controller, services::hog_service::HogService};
use lapin::Channel;
use mongodb::Database;

type RabbitChannel = Channel;

pub fn create_router(db: Database, rabbit_channel: RabbitChannel) -> Router {
    let hog_service = Arc::new(HogService::new(&db, rabbit_channel));

    Router::new()
        .route("/hogs", get(hog_controller::get_hogs))
        .route("/hogs", post(hog_controller::create_hog))
        .route("/hogs/search", post(hog_controller::handle_search))
        .route("/hogs/statistics", get(hog_controller::hog_statistics))
        .route("/hogs/stats", get(hog_controller::hog_stats))
        .route(
            "/hogs/advancedsearch",
            post(hog_controller::handle_advanced_search),
        )
        .layer(Extension(hog_service))
}
