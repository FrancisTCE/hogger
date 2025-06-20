use axum::extract::rejection::JsonRejection;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: StatusInfo,
    pub errors: Vec<ApiErrorField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
    pub status_code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorField {
    pub field: String,
    pub message: String,
}

#[derive(Debug)]
pub struct ValidationError {
    pub errors: Vec<ApiErrorField>,
}

impl ValidationError {
    pub fn new(errors: Vec<ApiErrorField>) -> Self {
        Self { errors }
    }
}

#[derive(Debug)]
pub struct SomeCreateError {
    pub cause: String,
}

impl SomeCreateError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { cause: msg.into() }
    }

    pub fn root_cause(&self) -> &str {
        &self.cause
    }
}

impl std::fmt::Display for SomeCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }
}

impl Error for SomeCreateError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    BadRequest {
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
    Unauthorized {
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
    Forbidden {
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
    NotFound {
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
    InternalServerError {
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
    Other {
        status_code: u16,
        message: String,
        fields: Option<Vec<ApiErrorField>>,
    },
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusInfo {
            status_code: self.status_code().as_u16(),
            message: self.message().to_string(),
        };

        let body = ErrorResponse {
            status,
            errors: self.fields().clone().unwrap_or_default(),
        };

        (self.status_code(), Json(body)).into_response()
    }
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden { .. } => StatusCode::FORBIDDEN,
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Other { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub fn message(&self) -> &str {
        match self {
            ApiError::BadRequest { message, .. }
            | ApiError::Unauthorized { message, .. }
            | ApiError::Forbidden { message, .. }
            | ApiError::NotFound { message, .. }
            | ApiError::InternalServerError { message, .. }
            | ApiError::Other { message, .. } => message,
        }
    }

    pub fn fields(&self) -> &Option<Vec<ApiErrorField>> {
        match self {
            ApiError::BadRequest { fields, .. }
            | ApiError::Unauthorized { fields, .. }
            | ApiError::Forbidden { fields, .. }
            | ApiError::NotFound { fields, .. }
            | ApiError::InternalServerError { fields, .. }
            | ApiError::Other { fields, .. } => fields,
        }
    }
}

#[macro_export]
macro_rules! api_error {
    ($variant:ident, $msg:expr, $field:expr, $field_msg:expr) => {
        ApiError::$variant {
            message: $msg.to_string(),
            fields: Some(vec![ApiErrorField {
                field: $field.to_string(),
                message: $field_msg.to_string(),
            }]),
        }
    };
}

impl From<JsonRejection> for ApiError {
    fn from(_: JsonRejection) -> Self {
        api_error!(
            BadRequest,
            "Payload must be a valid JSON object",
            "payload",
            "Invalid JSON"
        )
    }
}

impl From<ValidationError> for ApiError {
    fn from(validation_error: ValidationError) -> Self {
        let fields = validation_error
            .errors
            .into_iter()
            .map(|e| ApiErrorField {
                field: e.field,
                message: e.message,
            })
            .collect::<Vec<_>>();

        ApiError::BadRequest {
            message: "Validation error".to_string(),
            fields: Some(fields),
        }
    }
}

impl From<SomeCreateError> for ApiError {
    fn from(err: SomeCreateError) -> Self {
        ApiError::InternalServerError {
            message: "Failed to create hog".to_string(),
            fields: Some(vec![ApiErrorField {
                field: "trace".to_string(),
                message: err.root_cause().to_string(),
            }]),
        }
    }
}
