use crate::utils::{tools::{TxType, match_tx_type, from_wei_to_string}, blockchain::get_chain_info};
use alloy::consensus::transaction::EthereumTxEnvelope;
use alloy::rpc::types::Transaction;
use alloy_primitives::{Address, Bytes, U128, U256};
use eyre::Result;
use std::error::Error;

use crate::models::tx_structs::{FetchedTxData, DecodedTxData, ChainInfo, Gas};
use crate::semantics::semantics::{get_native_tx, get_erc20_transfer_tx};



pub async fn get_tx_data<T>(
    tx: &Transaction<EthereumTxEnvelope<T>>,
    tx_hash: &str,
) -> Result<FetchedTxData, Box<dyn Error>> {
    let recovered = &tx.inner;
    let envelope = recovered.inner();

    let zero_addr = Address::from_slice(&[0u8; 20]);

    let (chain_id, tx_type, value, nonce, gas_limit, gas_price, to, input): (
        u64,
        &'static str,
        U256,
        u64,
        U256,
        U128,
        Address,
        Bytes,
    ) = match envelope {
        EthereumTxEnvelope::Legacy(signed) => {
            let t = signed.tx();
            let to = *t.to.to().unwrap_or(&zero_addr);
            (
                t.chain_id.unwrap_or(0),
                "legacy",
                t.value,
                t.nonce,
                U256::from(t.gas_limit),
                U128::from(t.gas_price),
                to,
                t.input.clone(),
            )
        }
        EthereumTxEnvelope::Eip1559(signed) => {
            let t = signed.tx();
            let to = *t.to.to().unwrap_or(&zero_addr);
            (
                t.chain_id,
                "eip1559",
                t.value,
                t.nonce,
                U256::from(t.gas_limit),
                U128::from(t.max_fee_per_gas),
                to,
                t.input.clone(),
            )
        }
        _ => (
            0,
            "unknown",
            U256::from(0u64),
            0,
            U256::from(0u64),
            U128::from(0u64),
            zero_addr,
            Bytes::from(vec![]),
        ),
    };

    let tx_match = match_tx_type(&input, value)?;

    let input_data = match tx_match {
        TxType::ETHTransfer => {
            get_native_tx(
                &tx.inner.signer().to_string().as_str(), 
                &to.to_string().as_str(), 
                value,
                chain_id
            )
        },
        TxType::ERC20Transfer => {
            
            get_erc20_transfer_tx(&input, chain_id, &tx.inner.signer().to_string().as_str(), &to.to_string().as_str())
        },
        TxType::Unknown => get_native_tx(
            &tx.inner.signer().to_string().as_str(), 
            &to.to_string().as_str(), 
            value,
            chain_id
        )
        
    }?;

    let effective_gas_price = tx.effective_gas_price.unwrap();

    let (chain_name, native_asset) = get_chain_info(chain_id);
    let fee_wei = U256::from(gas_price) * U256::from(gas_limit);
    let fee_eth = from_wei_to_string(fee_wei, 18);

    let gas = Gas {
        limit: gas_limit.to_string(),
        price: gas_price.to_string(),
        effective_gas_price: effective_gas_price.to_string(),
        max_fee_wei: fee_wei.to_string(),
        max_fee_eth: fee_eth.to_string(),
    };
    let data = DecodedTxData {
        chain: ChainInfo {
            chain_id,
            name: chain_name.to_string(),
            native_asset: native_asset.to_string(),
        },
        tx_type,
        nonce,
        gas,
        to,
        actions: vec![input_data],
    };

    let fetched_tx = FetchedTxData {
        schema_version: "0.1.0".to_string(),
        signer: tx.inner.signer().to_string(),
        block_number: tx.block_number.unwrap(),
        block_hash: tx.block_hash.unwrap().to_string(),
        tx_hash: tx_hash.to_string(),
        transaction_index: tx.transaction_index.unwrap(),
        data,
    };
    
    Ok(fetched_tx)
}




