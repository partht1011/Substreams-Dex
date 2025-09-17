use crate::pb::io::blockchain::v1::dex::trade::{TradeEvent, Trade};
use crate::pb::io::chainstream::v1::common::{Block as CBlock, Transaction as CTransaction, Instruction as CInstruction, DApp, Chain};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2 as ethpb;
use hex;

// NOTE: substreams-ethereum v0.5 does not expose an Abi or Rpc helper here.
// Keep a placeholder decoder for now; replace with manual decoding or abigen later.

/// Decode a Swap event from a Uniswap V3 pool log (placeholder)
pub fn decode_swap_event(_log: &ethpb::Log) -> Result<(String, String, i128, i128), Error> {
    // TODO: implement proper ABI decoding; returning dummy values for now to keep build green
    Ok((String::new(), String::new(), 0, 0))
}

/// Build a TradeEvent from a decoded Swap event
pub fn build_trade_event(
    _log: &ethpb::Log,
    block: &ethpb::Block,
    pool_address: &str,
) -> Result<TradeEvent, Error> {
    let (sender, recipient, amount0, amount1) = (String::new(), String::new(), 0i128, 0i128);

    let token0 = pool_address.to_string(); // placeholder
    let token1 = pool_address.to_string(); // placeholder

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
            // ethpb::Block doesn't expose timestamp directly; leave placeholder
            timestamp: 0,
            hash: hex::encode(&block.hash),
            height: block.number as u64,
            slot: 0,
        }),
        transaction: Some(CTransaction {
            fee: 0,
            fee_payer: String::new(),
            index: 0,
            signature: String::new(),
            signer: String::new(),
            status: 1,
        }),
        d_app: Some(DApp {
            program_address: pool_address.to_string(),
            inner_program_address: String::new(),
            chain: Chain::CHAIN_BSC as i32,
        }),
        trade: Some(trade),
        bonding_curve: None,
    })
}
