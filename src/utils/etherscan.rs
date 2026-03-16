use std::time::Duration;

use alloy::hex;
use alloy_json_abi::{Function, JsonAbi};
use eyre::{bail, eyre};
use reqwest::Client;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct EtherscanResponse {
    status: String,
    result: String,
}

/// Fetches the ABI from Etherscan for the given contract address and looks for a function
/// matching the provided selector.
pub async fn fetch_etherscan_abi(
    chain_id: u64,
    contract_address: &str,
    selector: [u8; 4],
    api_key: &str,
) -> eyre::Result<Function> {
    

    println!("Fetching ABI from Etherscan for contract address: {} and selector on chain {}: {}", contract_address, chain_id, hex::encode(selector));
    
    // Fetch from Etherscan
    let url = format!(
        "https://api.etherscan.io/v2/api?module=contract&action=getabi&address={}&apikey={}&chainid={}",
        contract_address, api_key, chain_id
    );

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let response: EtherscanResponse = client.get(&url).send().await?.json().await?;

    if response.status != "1" {
        bail!("failed to fetch ABI from Etherscan: {}", response.result);
    }

    let full_abi: JsonAbi = serde_json::from_str(&response.result)
        .map_err(|e| eyre!("failed to parse ABI JSON: {}", e))?;

    let functions: Vec<Function> = full_abi.functions().cloned().collect();

    functions
        .into_iter()
        .find(|f| f.selector() == selector)
        .ok_or_else(|| {
            eyre!(
                "function with selector 0x{} not found in ABI",
                hex::encode(selector)
            )
        })
}

#[cfg(test)]
mod tests {
    use dotenvy::var;

    use super::*;

    #[tokio::test]
    async fn test_fetch_etherscan_abi() {
        dotenvy::dotenv().ok();
        let api_key = var("ETHERSCAN_API_KEY").unwrap();
        // USDC contract
        let addr = "0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2";
        // transfer(address,uint256) selector
        let sel = [0xa9, 0x05, 0x9c, 0xbb];

        let func = fetch_etherscan_abi(8453, addr, sel, &api_key).await.unwrap();
        println!("{:?}", func);
        assert_eq!(func.name, "transfer");
        assert_eq!(func.inputs.len(), 2);
    }
}
