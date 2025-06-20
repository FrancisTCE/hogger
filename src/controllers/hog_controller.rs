use axum::extract::rejection::JsonRejection;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};

use crate::metrics::{
    HOGS_CREATED_TOTAL, HOGS_FETCHED_TOTAL, HOGS_SEARCHED_TOTAL, REQUEST_DURATION_SECONDS,
};
use prometheus::{gather, Encoder, TextEncoder};

use serde_json::Value;
use std::sync::Arc;

use crate::errors::{ApiError, ApiErrorField};
use crate::models::hog_client_schema::validate;
use crate::models::options::validate_options;
use crate::services::hog_service::HogService;

pub async fn get_hogs(Extension(hog_service): Extension<Arc<HogService>>) -> impl IntoResponse {
    let timer = REQUEST_DURATION_SECONDS.start_timer();
    match hog_service.get_hogs().await {
        Ok(hogs) => {
            HOGS_FETCHED_TOTAL.inc_by(hogs.len() as u64);
            timer.observe_duration();
            Json(hogs).into_response()
        }
        Err(err) => {
            timer.observe_duration();
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

pub async fn create_hog(
    Extension(hog_service): Extension<Arc<HogService>>,
    payload: Result<Json<Value>, JsonRejection>,
) -> impl IntoResponse {
    let timer = REQUEST_DURATION_SECONDS.start_timer();

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

    match hog_service.create_hog(valid_request).await {
        Ok(hog) => {
            timer.observe_duration();
            HOGS_CREATED_TOTAL.inc();
            Json(hog).into_response()
        }
        Err(err) => {
            timer.observe_duration();
            ApiError::InternalServerError {
                message: "Failed to create hog".to_string(),
                fields: Some(vec![ApiErrorField {
                    field: "trace".to_string(),
                    message: err.root_cause().to_string(),
                }]),
            }
        }
        .into_response(),
    }
}

pub async fn handle_search(
    Extension(hog_service): Extension<Arc<HogService>>,
    payload: Result<Json<Value>, JsonRejection>,
) -> impl IntoResponse {
    let timer = REQUEST_DURATION_SECONDS.start_timer();
    let payload = match payload {
        Ok(Json(payload)) => {
            HOGS_SEARCHED_TOTAL.inc_by(payload.as_object().map_or(0, |obj| obj.len() as u64));
            payload
        }
        Err(e) => {
            return {
                timer.observe_duration();
                ApiError::BadRequest {
                    message: "Payload must be a valid JSON object".to_string(),
                    fields: vec![ApiErrorField {
                        field: "trace".to_string(),
                        message: e.to_string(),
                    }]
                    .into(),
                }
                .into_response()
            };
        }
    };

    let valid_request = match validate_options(payload).await {
        Ok(valid_request) => valid_request,
        Err(validation_error) => {
            return {
                timer.observe_duration();
                ApiError::BadRequest {
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
                .into_response()
            };
        }
    };

    let response = match hog_service.search_hogs(valid_request).await {
        Ok(hogs) => Json(hogs).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hogs: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    };
    timer.observe_duration();
    response
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

pub async fn hog_stats(Extension(hog_service): Extension<Arc<HogService>>) -> impl IntoResponse {
    match hog_service.hog_stats().await {
        Ok(stats) => Json(stats).into_response(),
        Err(err) => {
            let error_message = format!("Failed to fetch hog stats: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
        }
    }
}

// GRAFANA

pub async fn metrics() -> String {
    let metric_families = gather();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
