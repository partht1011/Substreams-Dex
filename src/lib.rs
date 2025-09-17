// Main handlers wired into substreams modules

pub mod pb {
    // Prost-generated modules live under this path after build.rs runs
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pb/io.chainstream.v1.common.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pb/io.blockchain.v1.dex.trade.rs"));
}

mod dapps;

use crate::pb::io::blockchain::v1::dex::trade::TradeEvents;
use substreams_solana::pb::sf::solana::r#type::v1 as solana;

// Solana handler
#[substreams::handlers::map]
fn map_sol_trades(blk: solana::Block) -> Result<TradeEvents, substreams::errors::Error> {
    let mut out = TradeEvents::default();

    if let Some(transactions) = blk.transactions.as_ref() {
        for trx in transactions.iter() {
            if let Some(meta) = &trx.meta {
                let pre_token_balances = meta.pre_token_balances.clone();
                let post_token_balances = meta.post_token_balances.clone();

                if let Some(message) = &trx.transaction.as_ref().and_then(|t| t.message.clone()) {
                    for (inst_index, inst) in message.instructions.iter().enumerate() {
                        let program_index = inst.program_id_index as usize;
                        let program_id = trx
                            .resolved_accounts
                            .get(program_index)
                            .map(|a| a.to_string())
                            .unwrap_or_default();

                        if program_id == dapps::raydium::RAYDIUM_PROGRAM_ID {
                            let accounts_vec: Vec<String> = trx.resolved_accounts.iter().map(|s| s.to_string()).collect();
                            if let Some(te) = dapps::raydium::parse_trade_instruction(
                                inst_index as u32,
                                &program_id,
                                &inst.data,
                                &accounts_vec,
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
fn map_bsc_uniswapv3_swaps(params: String, blk: substreams_ethereum::pb::eth::v2::Block) -> Result<TradeEvents, substreams::errors::Error> {
    let mut out = TradeEvents::default();

    let pool_addr_normalized = params.to_lowercase();

    for log in blk.logs().iter() {
        let log_addr = format!("0x{}", hex::encode(&log.address));
        if log_addr.to_lowercase() != pool_addr_normalized { continue; }

        if dapps::uniswap_v3::is_swap_log(log) {
            match dapps::uniswap_v3::build_trade_event(log, &blk, &pool_addr_normalized) {
                Ok(event) => out.events.push(event),
                Err(_) => {}
            }
        }
    }

    Ok(out)
}

