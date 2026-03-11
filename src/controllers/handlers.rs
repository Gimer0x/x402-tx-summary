use axum::response::IntoResponse;
use http::StatusCode;
use axum::extract::Path;

pub async fn my_handler() -> impl IntoResponse {
    (StatusCode::OK, "This is VIP content!")
}

pub async fn decoder(Path(tx_hash): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, format!("This is the tx hash: {}", tx_hash))
}

