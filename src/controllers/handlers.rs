use axum::response::IntoResponse;
use http::StatusCode;
use axum::extract::Path;
use crate::services::tx_fetcher::fetch_transaction;


pub async fn my_handler() -> impl IntoResponse {
    (StatusCode::OK, "This is VIP content!")
}

pub async fn decoder(Path(tx_hash): Path<String>) -> impl IntoResponse {
    let rpc_url = "https://base-sepolia.g.alchemy.com/v2/JPE2IEwnw2a8MKk00jjNm".to_string();
    let result = fetch_transaction(rpc_url.as_str(),tx_hash.as_str()).await;
    match result {
        Ok(Some(result)) => {
            (StatusCode::OK, format!("This is the tx: {:?}", result))
        }
        Ok(None) => {
            (StatusCode::NOT_FOUND, "Transaction not found".to_string())
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error fetching transaction: {}", e))
        }
    }
}

