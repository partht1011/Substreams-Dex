use crate::pb::io::blockchain::v1::dex::trade::{TradeEvent, Trade};
use crate::pb::io::chainstream::v1::common::{Block as CBlock, Transaction as CTransaction, Instruction as CInstruction, DApp, Chain};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2 as ethpb;
use hex;

// keccak256("Swap(address,address,int256,int256,uint160,uint128,int24)")
const SWAP_TOPIC0: [u8; 32] = hex_literal::hex!(
    "c42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"
);

pub fn is_swap_log(log: &ethpb::Log) -> bool {
    if log.topics.len() < 1 { return false; }
    log.topics[0].as_slice() == SWAP_TOPIC0
}

/// Minimal TradeEvent for a Uniswap V3 Swap: extracts sender/recipient (indexed), amounts default to 0 for simplicity
pub fn build_trade_event(
    log: &ethpb::Log,
    block: &ethpb::Block,
    pool_address: &str,
) -> Result<TradeEvent, Error> {
    // Topics:
    // 0: signature
    // 1: sender (indexed)
    // 2: recipient (indexed)
    let sender = if log.topics.len() > 1 {
        format!("0x{}", hex::encode(&log.topics[1][12..]))
    } else { String::new() };
    let recipient = if log.topics.len() > 2 {
        format!("0x{}", hex::encode(&log.topics[2][12..]))
    } else { String::new() };

    // For simplicity keep amounts as 0 for now (data contains amount0/amount1 as int256)
    let amount0: i128 = 0;
    let amount1: i128 = 0;

    let token0 = pool_address.to_string();
    let token1 = pool_address.to_string();

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
        pool_config_address: String::new(),
    };

    Ok(TradeEvent {
        instruction: Some(CInstruction { index: 0, is_inner_instruction: false, inner_instruction_index: 0, type_: "uniswap_v3_swap".to_string() }),
        block: Some(CBlock { timestamp: 0, hash: hex::encode(&block.hash), height: block.number as u64, slot: 0 }),
        transaction: Some(CTransaction { fee: 0, fee_payer: String::new(), index: 0, signature: String::new(), signer: String::new(), status: 1 }),
        d_app: Some(DApp { program_address: pool_address.to_string(), inner_program_address: String::new(), chain: Chain::CHAIN_BSC as i32 }),
        trade: Some(trade),
        bonding_curve: None,
    })
}
