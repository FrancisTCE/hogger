use axum::{routing::get, routing::post, Router, Extension};
use std::sync::Arc;

use crate::{controllers::hog_controller, services::hog_service::HogService};
use mongodb::Database;

pub fn create_router(db: Database) -> Router {
    let hog_service = Arc::new(HogService::new(&db));

    Router::new()
        .route("/hogs", get(hog_controller::get_hogs))
        .route("/hogs", post(hog_controller::create_hog))
        .layer(Extension(hog_service))
}
