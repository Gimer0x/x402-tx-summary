use crate::services::tx_data::get_tx_data;
use crate::services::tx_fetcher::tx_fetcher;
use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use dotenvy::var;
use http::StatusCode;
use serde_json::{Value, json};

pub async fn fetcher(Path(tx_hash): Path<String>) -> impl IntoResponse {
    let alchemy_api_key = var("BASE_RPC_URL").unwrap();

    let rpc_url = alchemy_api_key.to_string();
    let result = tx_fetcher(rpc_url.as_str(), tx_hash.as_str()).await;
    match result {
        Ok(Some(tx)) => {
            match get_tx_data(&tx).await {
                Ok(tx_data) => {
                    // Turn DecodedTx into serde_json::Value
                    let body: Value = serde_json::to_value(&tx_data)
                        .unwrap_or_else(|_| json!({ "error": "failed to serialize decoded tx" }));

                    (StatusCode::OK, Json(body))
                }
                Err(e) => {
                    let body = json!({ "error": e.to_string() });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(body))
                }
            }
        }
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
