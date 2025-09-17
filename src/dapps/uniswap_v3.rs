use crate::pb::io::blockchain::v1::dex::trade::{TradeEvents, TradeEvent, Trade};
use crate::pb::io::chainstream::v1::common::{Block as CBlock, Transaction as CTransaction, Instruction as CInstruction, DApp, Chain};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2 as ethpb;
use substreams_ethereum::Event;
use substreams_ethereum::rpc::Rpc;
use substreams_ethereum::Abi;
use hex;

// ABI file path (already in abi/IUniswapV3Pool.json)
const UNISWAP_V3_POOL_ABI: &str = "abi/IUniswapV3Pool.json";

/// Decode a Swap event from a Uniswap V3 pool log
pub fn decode_swap_event(log: &ethpb::Log) -> Result<(String, String, i128, i128), Error> {
    // Load ABI once (can be cached in real implementation)
    let abi = Abi::from_path(UNISWAP_V3_POOL_ABI).map_err(|e| Error::from(e.to_string()))?;
    
    let event = abi.event("Swap").map_err(|e| Error::from(e.to_string()))?;

    // Decode log
    let decoded = event.parse_log(&log.data, &log.topics).map_err(|e| Error::from(e.to_string()))?;

    // Extract fields
    let sender = decoded.params[0].value.to_string();
    let recipient = decoded.params[1].value.to_string();
    let amount0: i128 = decoded.params[2].value.to_string().parse().unwrap_or(0);
    let amount1: i128 = decoded.params[3].value.to_string().parse().unwrap_or(0);

    Ok((sender, recipient, amount0, amount1))
}

/// Build a TradeEvent from a decoded Swap event
pub fn build_trade_event(
    log: &ethpb::Log,
    block: &ethpb::Block,
    tx: &ethpb::Transaction,
    pool_address: &str,
) -> Result<TradeEvent, Error> {
    let (sender, recipient, amount0, amount1) = decode_swap_event(log)?;

    // TODO: In real implementation fetch token0/token1 addresses via eth_call
    let token0 = format!("{}", pool_address); // placeholder
    let token1 = format!("{}", pool_address); // placeholder

    let trade = Trade {
        token_a_address: token0.clone(),
        token_b_address: token1.clone(),
        user_a_token_account_address: sender.clone(),
        user_a_account_owner_address: sender.clone(),
        user_b_token_account_address: recipient.clone(),
        user_b_account_owner_address: recipient.clone(),
        user_a_amount: amount0.to_string(),
        user_b_amount: amount1.to_string(),
        user_a_pre_amount: "0".to_string(),
        user_a_post_amount: "0".to_string(),
        user_b_pre_amount: "0".to_string(),
        user_b_post_amount: "0".to_string(),
        was_original_direction: true,
        pool_address: pool_address.to_string(),
        vault_a: token0,
        vault_b: token1,
        vault_a_owner_address: sender.clone(),
        vault_b_owner_address: recipient.clone(),
        vault_a_amount: amount0.to_string(),
        vault_b_amount: amount1.to_string(),
        vault_a_pre_amount: "0".to_string(),
        vault_b_pre_amount: "0".to_string(),
        vault_a_post_amount: "0".to_string(),
        vault_b_post_amount: "0".to_string(),
        pool_config_address: "".to_string(),
    };

    Ok(TradeEvent {
        instruction: Some(CInstruction {
            index: 0,
            is_inner_instruction: false,
            inner_instruction_index: 0,
            type_: "uniswap_v3_swap".to_string(),
        }),
        block: Some(CBlock {
            timestamp: block.timestamp as i64,
            hash: hex::encode(&block.hash),
            height: block.number as u64,
            slot: 0,
        }),
        transaction: Some(CTransaction {
            fee: tx.gas_used as u64,
            fee_payer: tx.from.clone(),
            index: 0,
            signature: tx.hash.clone(),
            signer: tx.from.clone(),
            status: 1,
        }),
        d_app: Some(DApp {
            program_address: pool_address.to_string(),
            inner_program_address: "".to_string(),
            chain: Chain::CHAIN_BSC as i32,
        }),
        trade: Some(trade),
        bonding_curve: None,
    })
}
