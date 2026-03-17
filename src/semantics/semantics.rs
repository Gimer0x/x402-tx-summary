use crate::utils::tools;
use crate::models::tx_structs::InputTxData;
use alloy_primitives::U256;
use std::error::Error;
use alloy_primitives::{Bytes, Address};
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



pub fn get_erc20_transfer_tx(input: &Bytes, chain_id: u64, signer: &str) -> Result<InputTxData, Box<dyn Error>> {
    let (receipient, amount) = get_amount_and_recipient(input);

    // TODO: Get the name of the ERC20 token
    // TODO: Get the symbol of the ERC20 token
    // TODO: Get the decimals of the ERC20 token
    let amount_in_string = tools::from_wei_to_string(amount, 6);
    let summary = format!("Transfer {} ERC20 from {} to {}", amount_in_string, signer, receipient);

    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "erc20".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        from: signer.to_string(),
        to: receipient.to_string(),
        asset_in: "USDT".to_string(),
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

pub fn get_native_tx(signer: &str, to: &str, value: U256) ->  Result<InputTxData, Box<dyn Error>>{

    let value_in_eth = tools::from_wei_to_string(value, 18);

    let summary = format!("Transfer {} ETH from {} to {}", value_in_eth, signer, to);
    let native_tx = InputTxData {
        r#type: "transfer".to_string(),
        subtype: "native".to_string(),
        intent: "send_money".to_string(),
        summary: summary,
        from: signer.to_string(),
        to: to.to_string(),
        asset_in: "ETH".to_string(),
        asset_out: "".to_string(),
        amount: value.to_string(),
    };
    
    Ok(native_tx)
}


/*pub async fn decode_input_data(
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
} */