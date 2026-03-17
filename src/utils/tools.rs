use alloy::primitives::{Bytes, U256};
use rust_decimal::Decimal;
use dotenvy::var;
use eyre::eyre;

pub enum TxType {
    ETHTransfer,
    ERC20Transfer,
    Unknown,
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
        Ok(TxType::ETHTransfer)
    } else if input.starts_with(&[0xa9, 0x05, 0x9c, 0xbb]) {
        Ok(TxType::ERC20Transfer)
    } else {
        Ok(TxType::Unknown)
    }
}

pub fn wei_to_eth_string(wei: U256) -> String {
    // convert U256 -> Decimal
    let wei_u128: u128 = wei.try_into().unwrap(); // ok if it fits in 128 bits
    let mut d = Decimal::from(wei_u128);
    let _ = d.set_scale(18);
    d.normalize().to_string()
}
