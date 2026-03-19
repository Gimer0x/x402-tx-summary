use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};

#[derive(Debug)]
pub enum ApiError {
    InvalidTxHash,
    InvalidNetwork,
    InternalError(String),
    TXNotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::InvalidTxHash => (StatusCode::BAD_REQUEST, "Invalid tx hash".to_string()),
            ApiError::InvalidNetwork => (StatusCode::BAD_REQUEST, "Invalid network".to_string()),
            ApiError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
            ApiError::TXNotFound => (StatusCode::NOT_FOUND, "Transaction not found".to_string()),
        };

        //(status, msg).into_response()
        (status, msg).into_response()
    }
}