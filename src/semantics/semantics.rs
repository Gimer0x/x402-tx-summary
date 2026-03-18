use crate::utils::{tools, blockchain::{get_chain_info, get_token_info}};
use crate::models::tx_structs::{InputTxData, TokenInfo};
use std::error::Error;
use alloy_primitives::{Bytes, U256, Address};
use dotenvy::var;
use crate::utils::etherscan;
use std::fmt;
use std::convert::TryInto;

#[derive(Debug)]
pub struct StrError(pub String);
impl fmt::Display for StrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for StrError {}

pub fn get_erc20_transfer_tx(input: &Bytes, chain_id: u64, signer: &str, token_address: &str) -> Result<InputTxData, Box<dyn Error>> {
    let (recipient, amount) = get_amount_and_recipient(input);

    // TODO: Get the name of the ERC20 token
    // TODO: Get the symbol of the ERC20 token
    // TODO: Get the decimals of the ERC20 token
    let (token_name, token_symbol, token_decimals) = get_token_info(chain_id, &token_address.to_string());
    let amount_in_string = tools::from_wei_to_string(amount, token_decimals.try_into().unwrap());
    let (chain_name, _) = get_chain_info(chain_id);
    let summary = format!("Transfer {amount_in_string} {token_symbol} from {signer} to {recipient} on {chain_name}");

    let token_info = TokenInfo {
        name: token_name,
        symbol: token_symbol,
        decimals: token_decimals,
    };

    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "erc20".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        from: signer.to_string(),
        recipient: recipient.to_string(),
        asset_in: vec![token_info],
        asset_out: "".to_string(),
        amount: amount.to_string(),
    };

    Ok(native_tx)
}

/// Decodes ERC20 transfer(address,uint256) calldata.
/// Layout: 4-byte selector, then 32-byte word (address right-padded), then 32-byte word (amount).
pub fn get_amount_and_recipient(input: &Bytes) -> (Address, U256) {
    let recipient = Address::from_slice(&input[16..36]); // last 20 bytes of first 32-byte word
    let bytes: [u8; 32] = input[36..68].try_into().unwrap(); // second 32-byte word
    let amount = U256::from_be_bytes(bytes);
    (recipient, amount)
}

pub fn get_native_tx(signer: &str, recipient: &str, value: U256, chain_id: u64) ->  Result<InputTxData, Box<dyn Error>>{

    let zero_address: Address = Address::from_slice(&[0u8; 20]);

    println!("zero_address: {zero_address}");

    let (token_name, token_symbol, token_decimals) = get_token_info(chain_id, &zero_address.to_string());
    let token_info = TokenInfo {
        name: token_name,
        symbol: token_symbol,
        decimals: token_decimals,
    };

    let value_in_eth = tools::from_wei_to_string(value, token_decimals.try_into().unwrap());
    let (chain_name, native_asset) = get_chain_info(chain_id);

    let summary = format!("Transfer {} {} from {} to {} on {}", value_in_eth, native_asset, signer, recipient, chain_name);
    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "native".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        from: signer.to_string(),
        recipient: recipient.to_string(),
        asset_in: vec![token_info],
        asset_out: "".to_string(),
        amount: value.to_string(),
    };
    
    Ok(native_tx)
}


pub async fn decode_input_data(
    input: &Bytes,
    chain_id: u64,
    to: Address,
) -> Result<String, Box<dyn Error>> {
    let etherscan_api_key =
        var("ETHERSCAN_API_KEY").map_err(|e| -> Box<dyn Error> { Box::new(e) })?;

    if input.len() < 4 {
        return Ok(input.to_string());
    }
    let selector = tools::get_selector(input)
        .map_err(|e| -> Box<dyn Error> { Box::new(StrError(e.to_string())) })?;

    // Let's check if the selector is a well-known selector
    let _abi = etherscan::fetch_etherscan_abi(
        chain_id,
        to.to_string().as_str(),
        selector,
        etherscan_api_key.as_str(),
    )
    .await
    .map_err(|e| -> Box<dyn Error> { Box::new(e) })?;
    Ok(input.to_string())
}