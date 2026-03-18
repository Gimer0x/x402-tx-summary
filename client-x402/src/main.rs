use std::sync::Arc;
use alloy_signer_local::PrivateKeySigner;
use reqwest::Client;
use x402_reqwest::{ReqwestWithPayments, ReqwestWithPaymentsBuild, X402Client};
use x402_chain_eip155::V1Eip155ExactClient;
use dotenvy::var;

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

    let x402_client = X402Client::new().register(V1Eip155ExactClient::new(signer));

    let http_client = Client::new()
        .with_payments(x402_client)
        .build();

    let network = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "mainnet".into());

    let transaction_hash = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdff".into());
    // Your protected URL with transaction_hash in the path
    let hostname = var("SERVER_HOST").expect("Set HOSTNAME");
    let url = format!(
        "https://{hostname}/summary/{}/{}",
        network,
        transaction_hash
    );

    println!("URL: {}", url);

    let response = http_client.post(&url).send().await?;
    
    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);
    
    Ok(())
}