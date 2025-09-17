use crate::pb::io::blockchain::v1::dex::trade::{TradeEvents, TradeEvent, Trade};
use crate::pb::io::chainstream::v1::common::{Block as CBlock, Transaction as CTransaction, Instruction as CInstruction, DApp, Chain};
use substreams::errors::Error;

pub const RAYDIUM_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

/// Parses Raydium swap instruction using pre/post token balances
pub fn parse_trade_instruction(
    instruction_index: u32,
    program_id: &str,
    instruction_data: &[u8],
    accounts: &Vec<String>,
    pre_token_balances: &Vec<crate::pb::sf::solana::v1::TokenBalance>,
    post_token_balances: &Vec<crate::pb::sf::solana::v1::TokenBalance>,
) -> Option<TradeEvent> {
    if program_id != RAYDIUM_PROGRAM_ID {
        return None;
    }

    // Find user and vault accounts
    if accounts.len() < 4 { return None; }
    let user_a_token_account = accounts[0].clone();
    let user_b_token_account = accounts[1].clone();
    let vault_a = accounts[2].clone();
    let vault_b = accounts[3].clone();

    // Compute amount deltas using pre/post balances
    let mut user_a_amount = 0i128;
    let mut user_b_amount = 0i128;
    let mut vault_a_pre = 0i128;
    let mut vault_b_pre = 0i128;
    let mut vault_a_post = 0i128;
    let mut vault_b_post = 0i128;

    for b in pre_token_balances {
        if b.account == user_a_token_account { user_a_amount -= b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == user_b_token_account { user_b_amount -= b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == vault_a { vault_a_pre = b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == vault_b { vault_b_pre = b.ui_token_amount.parse::<i128>().unwrap_or(0); }
    }

    for b in post_token_balances {
        if b.account == user_a_token_account { user_a_amount += b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == user_b_token_account { user_b_amount += b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == vault_a { vault_a_post = b.ui_token_amount.parse::<i128>().unwrap_or(0); }
        if b.account == vault_b { vault_b_post = b.ui_token_amount.parse::<i128>().unwrap_or(0); }
    }

    let trade = Trade {
        token_a_address: vault_a.clone(),
        token_b_address: vault_b.clone(),
        user_a_token_account_address: user_a_token_account.clone(),
        user_a_account_owner_address: "".to_string(), // TODO: fill owner
        user_b_token_account_address: user_b_token_account.clone(),
        user_b_account_owner_address: "".to_string(),
        user_a_amount: user_a_amount.to_string(),
        user_b_amount: user_b_amount.to_string(),
        user_a_pre_amount: vault_a_pre.to_string(),
        user_a_post_amount: vault_a_post.to_string(),
        user_b_pre_amount: vault_b_pre.to_string(),
        user_b_post_amount: vault_b_post.to_string(),
        was_original_direction: true,
        pool_address: RAYDIUM_PROGRAM_ID.to_string(),
        vault_a,
        vault_b,
        vault_a_owner_address: "".to_string(),
        vault_b_owner_address: "".to_string(),
        vault_a_amount: (vault_a_post - vault_a_pre).to_string(),
        vault_b_amount: (vault_b_post - vault_b_pre).to_string(),
        vault_a_pre_amount: vault_a_pre.to_string(),
        vault_b_pre_amount: vault_b_pre.to_string(),
        vault_a_post_amount: vault_a_post.to_string(),
        vault_b_post_amount: vault_b_post.to_string(),
        pool_config_address: "".to_string(),
    };

    let event = TradeEvent {
        instruction: Some(CInstruction {
            index: instruction_index,
            is_inner_instruction: false,
            inner_instruction_index: 0,
            type_: "raydium_swap".to_string(),
        }),
        block: Some(CBlock {
            timestamp: 0,
            hash: "".to_string(),
            height: 0,
            slot: 0,
        }),
        transaction: Some(CTransaction {
            fee: 0,
            fee_payer: "".to_string(),
            index: 0,
            signature: "".to_string(),
            signer: "".to_string(),
            status: 1,
        }),
        d_app: Some(DApp {
            program_address: RAYDIUM_PROGRAM_ID.to_string(),
            inner_program_address: "".to_string(),
            chain: Chain::CHAIN_SOLANA as i32,
        }),
        trade: Some(trade),
        bonding_curve: None,
    };

    Some(event)
}