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

    let network = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "8453".into());

    let transaction_hash = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdff".into());

    let hostname = var("SERVER_HOST").expect("Set SERVER_HOST");
    // network / tx_hash are hex-safe for query strings (no spaces).
    let url = format!(
        "https://{hostname}/summary?network={network}&tx_hash={transaction_hash}"
    );

    println!("URL: {}", url);

    let response = http_client.get(&url).send().await?;

    println!("Response: {:?}", response);

    let status = response.status().as_u16();
    let body_text = response.text().await?;

    // If the response body is valid JSON, embed it as JSON; otherwise store it as a string.
    let body_value = serde_json::from_str::<serde_json::Value>(&body_text).ok();
    let result = match body_value {
        Some(value) => json!({  
            "body": value,
            "url": url,
        }),
        None => json!({"body": body_text, "url": url}),
    };

    let out_path = format!("./result.json");
    let result_json = serde_json::to_string_pretty(&result)?;
    std::fs::write(&out_path, result_json)?;

    println!("Status: {}", status);
    println!("Wrote response to {}", out_path);

    Ok(())
}