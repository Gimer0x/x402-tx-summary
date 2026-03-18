use alloy::primitives::{Bytes, U256};
use rust_decimal::Decimal;
use dotenvy::var;
use eyre::eyre;

pub enum TxType {
    ETHTransfer,
    ERC20Transfer,
    Unknown,
}

pub fn get_chain_info(chain_id: u64) -> (String, String) {
    match chain_id {
        1 => ("Ethereum".to_string(), "ETH".to_string()),
        5 => ("Ethereum Goerli".to_string(), "ETH".to_string()),
        137 => ("Polygon".to_string(), "MATIC".to_string()),
        80001 => ("Polygon Mumbai".to_string(), "MATIC".to_string()),
        42161 => ("Arbitrum".to_string(), "ETH".to_string()),
        421611 => ("Arbitrum Sepolia".to_string(), "ETH".to_string()),
        8453 => ("Base".to_string(), "ETH".to_string()),
        84532 => ("Base Sepolia".to_string(), "ETH".to_string()),
        _ => ("Unknown".to_string(), "UNKNOWN".to_string()),
    }
}

/// Extracts the first four bytes from the given byte slice to use as a function selector.
pub fn get_selector(data: &Bytes) -> eyre::Result<[u8; 4]> {
    data.get(..4)
        .and_then(|s| s.try_into().ok())
        .ok_or_else(|| eyre!("data too short to extract selector"))
}

pub fn get_rpc_url(network: &str) -> Result<String, eyre::Error> {
    match network {
        "1" => Ok(var("ETHEREUM_RCP_URL")?),         // ETHEREUM MAINNET
        "8453" => Ok(var("BASE_RPC_URL")?),          // BASE MAINNET
        "84532" => Ok(var("BASE_SEPOLIA_RPC_URL")?), // BASE SEPOLIA
        _ => return Err(eyre!("Invalid network: {}, network".to_string())),
    }
}

pub fn match_tx_type(input: &Bytes, value: U256) -> Result<TxType, eyre::Error> {

    if value > 0 && input.is_empty() {
        return Ok(TxType::ETHTransfer);
    } 
    
    match get_selector(input) {
        Ok([0xa9, 0x05, 0x9c, 0xbb]) => Ok(TxType::ERC20Transfer),
        _ => Ok(TxType::Unknown),
    }
    
}

pub fn from_wei_to_string(wei: U256, decimals: u32) -> String {
    // convert U256 -> Decimal
    let wei_u128: u128 = wei.try_into().unwrap(); // ok if it fits in 128 bits
    let mut d = Decimal::from(wei_u128);
    let _ = d.set_scale(decimals);
    d.normalize().to_string()
}
