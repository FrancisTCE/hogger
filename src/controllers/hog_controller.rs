use axum::extract::rejection::JsonRejection;
use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};

use serde_json::Value;
use std::sync::Arc;

use crate::errors::{ApiError, ApiErrorField};
use crate::models::client_request::ClientRequest;
use crate::models::hog_client_schema::validate;
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
    match hog_service.create_hog(client_request).await {
        Ok(hog) => Json(hog).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

pub async fn create_hog_validated(
    Extension(hog_service): Extension<Arc<HogService>>,
    payload: Result<Json<Value>, JsonRejection>,
) -> impl IntoResponse {

    let payload = match payload {
        Ok(Json(payload)) => payload,
        Err(e) => {
            return ApiError::BadRequest {
                message: "Payload must be a valid JSON object".to_string(),
                fields: vec![ApiErrorField {
                    field: "trace".to_string(),
                    message: e.to_string(),
                }]
                .into(),
            }
            .into_response();
        }
    };

    let valid_request = match validate(payload).await {
        Ok(valid_request) => valid_request,
        Err(validation_error) => {
            return ApiError::BadRequest {
                message: "Validation error".to_string(),
                fields: validation_error
                    .errors
                    .iter()
                    .map(|e| ApiErrorField {
                        field: e.field.clone(),
                        message: e.message.clone(),
                    })
                    .collect::<Vec<_>>()
                    .into(),
            }
            .into_response();
        }
    };

    match hog_service.create_hog_validated(valid_request).await {
        Ok(hog) => Json(hog).into_response(),
        Err(err) => ApiError::InternalServerError {
            message: "Failed to create hog".to_string(),
            fields: Some(vec![ApiErrorField {
                field: "trace".to_string(),
                message: err.root_cause().to_string(),
            }]),
        }
        .into_response(),
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

pub async fn handle_advanced_search(
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
    Extension(hog_service): Extension<Arc<HogService>>,
) -> impl IntoResponse {
    match hog_service.hog_statistics().await {
        Ok(statistics) => Json(statistics).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}
