use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::models::client_request::ClientRequest;
use crate::models::options::OptionsRequest;
use crate::services::hog_service::HogService;

pub async fn get_hogs(Extension(hog_service): Extension<Arc<HogService>>) -> impl IntoResponse {
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
    print!("Received client request: {:?}", client_request);
    match hog_service.create_hog(client_request).await {
        Ok(hog) => Json(hog).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

pub async fn handle_search(
    Extension(hog_service): Extension<Arc<HogService>>,
    Json(options): Json<OptionsRequest>,
) -> impl IntoResponse {

    match hog_service.search_hogs(options).await {
        Ok(hogs) => Json(hogs).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

pub async fn hog_statistics(
    Extension(hog_service): Extension<Arc<HogService>>
) -> impl IntoResponse {
    match hog_service.hog_statistics().await {
        Ok(statistics) => Json(statistics).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}
