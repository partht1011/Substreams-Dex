### `README.md` (commands & next steps)

# Substreams DEX Trades — Scaffold

## Quick start

1. Install Rust and Substreams CLI (see Substreams docs).
2. Copy real Uniswap V3 pool ABI into `abi/IUniswapV3Pool.json`.
3. Generate protobuf & build:

```bash
substreams protogen ./substreams.yaml
substreams build
```

4. Run (example; replace endpoints and auth as needed):

```bash
substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_sol_trades --start-block <SLOT> --stop-block +10

substreams run -e mainnet.bsc.streamingfast.io:443 substreams.yaml map_bsc_uniswapv3_swaps --start-block <BLOCK> --stop-block +10 --param.pool_address=0xd857e4a8fe599ed936157076674b2756d9df6fe8
```

## Next work to finish the parser

- Implement `parse_trade_instruction` in `src/dapps/raydium.rs`:

  - decode instruction payload OR compute deltas using `pre_token_balances` and `post_token_balances`.
  - find user token accounts, vault accounts and compute pre/post balances.
  - fill `TradeEvent` protobuf fields.

- Implement `decode_swap_event` in `src/dapps/uniswap_v3.rs`:

  - generate abigen bindings or implement manual abi decode for `Swap`.
  - eth_call `token0()` / `token1()` once and cache to fill token addresses.
  - map sign semantics (amount0/amount1) to user input/output amounts.

- Add unit tests using Substreams testing helpers (store sample blocks & logs) and compare expected proto outputs.
