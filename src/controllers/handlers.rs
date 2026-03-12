use axum::response::IntoResponse;
use http::StatusCode;
use axum::extract::Path;
use crate::services::tx_fetcher::tx_fetcher;
use dotenvy::var;
use crate::services::tx_decoder::{decoder};
use axum::Json;
use serde_json::{json, Value};

pub async fn fetcher(Path(tx_hash): Path<String>) -> impl IntoResponse {

    let alchemy_api_key = var("ALCHEMY_API_KEY").unwrap();

    let rpc_url = alchemy_api_key.to_string();
    let result = tx_fetcher(rpc_url.as_str(), tx_hash.as_str()).await;
    match result {
        Ok(Some(tx)) => {
            match decoder(&tx).await {
                Ok(decoded) => {
                    // Turn DecodedTx into serde_json::Value
                    let body: Value = serde_json::to_value(&decoded)
                        .unwrap_or_else(|_| json!({ "error": "failed to serialize decoded tx" }));
                    (StatusCode::OK, Json(body))
                }
                Err(e) => {
                    let body = json!({ "error": e.to_string() });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
                }
            }
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Transaction not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Error fetching transaction: {e}") })),
        ),
    }
}