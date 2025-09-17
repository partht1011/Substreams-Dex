
// Main handlers wired into substreams modules

use substreams::log::info;

mod dapps;

use crate::pb::io::blockchain::v1::dex::trade::{TradeEvents, TradeEvent, Trade};
use crate::pb::io::chainstream::v1::common::{Block as CBlock, Transaction as CTransaction, Instruction as CInstruction, DApp, Chain};

// Solana handler
#[substreams::handlers::map]
fn map_sol_trades(blk: substreams::pb::sf::solana::v1::Block) -> Result<TradeEvents, substreams::errors::Error> {
    let mut out = TradeEvents::default();

    if let Some(transactions) = blk.transactions.as_ref() {
        for (tx_index, trx) in transactions.iter().enumerate() {
            // get metadata
            if let Some(meta) = &trx.meta {
                // pre/post token balances exist in meta
                let pre_token_balances = meta.pre_token_balances.clone();
                let post_token_balances = meta.post_token_balances.clone();

                if let Some(message) = &trx.transaction.as_ref().and_then(|t| t.message.clone()) {
                    for (inst_index, inst) in message.instructions.iter().enumerate() {
                        // resolve program id string
                        let program_index = inst.program_id_index as usize;
                        let program_id = trx.resolved_accounts.get(program_index).map(|a| a.to_string()).unwrap_or_default();
                        if program_id == dapps::raydium::RAYDIUM_PROGRAM_ID {
                            if let Some(te) = dapps::raydium::parse_trade_instruction(
                                inst_index as u32,
                                &program_id,
                                &inst.data,
                                &trx.resolved_accounts.iter().map(|s| s.to_string()).collect(),
                                &pre_token_balances,
                                &post_token_balances,
                            ) {
                                out.events.push(te);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(out)
}

// EVM/BSC handler
#[substreams::handlers::map]
fn map_bsc_uniswapv3_swaps(params: substreams::scalar::Params, blk: substreams_ethereum::pb::eth::v2::Block) -> Result<TradeEvents, substreams::errors::Error> {
    let mut out = TradeEvents::default();

    // read pool address param
    let pool_addr = params.get("pool_address").unwrap_or(&"".to_string()).clone();
    let pool_addr_normalized = pool_addr.to_lowercase();

    for log in blk.logs.iter() {
        let log_addr = format!("0x{}", hex::encode(&log.address));
        if log_addr.to_lowercase() != pool_addr_normalized { continue; }

        match dapps::uniswap_v3::decode_swap_event(log) {
            Ok((sender, recipient, amount0, amount1)) => {
                // Build TradeEvent from decoded fields + block/tx context
                // TODO: fill properly: block, transaction, d_app, trade fields
            }
            Err(_) => {}
        }
    }

    Ok(out)
}


