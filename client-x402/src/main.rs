use std::sync::Arc;
use alloy_signer_local::PrivateKeySigner;
use reqwest::Client;
use x402_reqwest::{ReqwestWithPayments, ReqwestWithPaymentsBuild, X402Client};
use x402_chain_eip155::V2Eip155ExactClient;
use dotenvy::var;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Load private key from env (never commit real keys)
    let key_hex = var("EVM_PRIVATE_KEY")
        .or_else(|_| var("X402_PRIVATE_KEY"))
        .expect("Set EVM_PRIVATE_KEY or X402_PRIVATE_KEY (hex string, with or without 0x)");

    let key_hex = key_hex.strip_prefix("0x").unwrap_or(&key_hex);
    let signer: Arc<PrivateKeySigner> = Arc::new(
        format!("0x{key_hex}")
            .parse()
            .expect("Invalid EVM private key"),
    );

    let x402_client = X402Client::new().register(V2Eip155ExactClient::new(signer));

    let http_client = Client::new()
        .with_payments(x402_client)
        .build();

    let network = std::env::args().nth(1).unwrap_or_else(|| "8453".into());

    let transaction_hash = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdff".into());
    let hostname = var("SERVER_HOST").expect("Set SERVER_HOST");
    let url = format!("https://{hostname}/summary");

    println!("URL: {}", url);

    let response = http_client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "network": network,
                "tx_hash": transaction_hash
            })
            .to_string(),
        )
        .send()
        .await?;
    
    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);
    
    Ok(())
}