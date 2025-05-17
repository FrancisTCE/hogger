use axum::{extract::Extension, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;

use crate::models::client_request::ClientRequest;
use crate::services::hog_service::HogService;

pub async fn get_hogs(
    Extension(hog_service): Extension<Arc<HogService>>,
) -> impl IntoResponse {
    match hog_service.get_hogs().await {
        Ok(hogs) => Json(hogs).into_response(),
         Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

pub async fn create_hog(
    Extension(hog_service): Extension<Arc<HogService>>,
    Json(client_request): Json<ClientRequest>,
) -> impl IntoResponse {
    match hog_service.create_hog(client_request).await {
        Ok(hog) => Json(hog).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create hog").into_response(),
    }
}

