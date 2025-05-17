use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

use crate::models::client_request::ClientRequest;
use crate::models::options::{OptionsRequest, SearchType};
use crate::services::hog_service::HogService;

pub async fn get_hogs(Extension(hog_service): Extension<Arc<HogService>>) -> impl IntoResponse {
    println!("Received request to get hogs");
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
    println!("Received search options: {:?}", options);
    let search_type = options.search_type.clone();
    if search_type.is_empty() {
        return (StatusCode::BAD_REQUEST, "Search type is required").into_response();
    }
    let search_type = match options.search_type.parse::<SearchType>() {
        Ok(s) => s,
        Err(_) => {
            let err_body = json!({ "error": "Invalid search type" });
            return (StatusCode::BAD_REQUEST, Json(err_body)).into_response();
        }
    };

    match hog_service.search_hogs(options, search_type).await {
        Ok(hogs) => Json(hogs).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}
