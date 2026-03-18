// use std::time::Duration;
// 
// use alloy::hex;
// use alloy_json_abi::{Function, JsonAbi};
// use reqwest::Client;
// use serde::Deserialize; 

#[derive(Debug, thiserror::Error)]
pub enum EtherscanAbiError {
    //#[error("Etherscan API error: {0}")]
    //ApiError(String),

    /*#[error(
        "function with selector 0x{selector} not found in ABI (contract: {address}, chain: {chain_id})"
    )]
    SelectorNotFound {
         address: String,
         chain_id: u64,
         selector: String,
    },*/
    

    #[error("failed to parse ABI JSON: {0}")]
    ParseError(#[from] serde_json::Error),

    /*#[error("request failed: {0}")]
    Request(#[from] reqwest::Error), */
}

/*#[derive(Debug, Deserialize)]
struct EtherscanResponse {
    status: String,
    result: String,
} */

// Fetches the ABI from Etherscan for the given contract address and looks for a function
// matching the provided selector.
/*pub async fn fetch_etherscan_abi(
    chain_id: u64,
    contract_address: &str,
    selector: [u8; 4],
    api_key: &str,
) -> Result<Function, EtherscanAbiError> {
    let url = format!(
        "https://api.etherscan.io/v2/api?module=contract&action=getabi&address={}&apikey={}&chainid={}",
        contract_address, api_key, chain_id
    );

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response: EtherscanResponse = client.get(&url).send().await?.json().await?;

    if response.status != "1" {
        return Err(EtherscanAbiError::ApiError(response.result));
    }

    let full_abi: JsonAbi = serde_json::from_str(&response.result)?;
    let functions: Vec<Function> = full_abi.functions().cloned().collect();

    functions
        .into_iter()
        .find(|f| f.selector() == selector)
        .ok_or_else(|| EtherscanAbiError::SelectorNotFound {
            address: contract_address.to_string(),
            chain_id,
            selector: hex::encode(selector),
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

        let func = fetch_etherscan_abi(8453, addr, sel, &api_key)
            .await
            .unwrap();
        println!("{:?}", func);
        assert_eq!(func.name, "transfer");
        assert_eq!(func.inputs.len(), 2);
    }
} */
