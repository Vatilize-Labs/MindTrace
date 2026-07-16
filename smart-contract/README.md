# MindTrace Smart Contract

Soroban (Stellar) smart contract for MindTrace.

## Layout

- `src/lib.rs` — contract entry points
- `src/game_state.rs` — game state types and storage
- `src/events.rs` — contract events
- `examples/game_flow.rs` — example game flow

## Build

Requires Rust (the version is pinned in `rust-toolchain.toml`) with the `wasm32-unknown-unknown` target.

```sh
cargo test
cargo build --target wasm32-unknown-unknown --release
```

The compiled contract lands at `target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm`.

## Deploy

See `../SOROBAN_BUILD_DEPLOY.md` or run `../deploy.sh` from the repo root.
