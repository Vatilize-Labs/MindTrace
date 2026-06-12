# MindTrace: Aleo to Stellar (Soroban) Conversion

## Overview

This document explains the conversion of MindTrace from **Aleo/Leo** to **Stellar/Soroban (Rust)**, optimized for ecosystem liquidity and public gameplay.

---

## Key Architectural Changes

### From Aleo (Leo) → To Stellar (Soroban/Rust)

| Aspect | Aleo | Stellar |
|--------|------|---------|
| **Language** | Leo | Rust (via Soroban SDK) |
| **Privacy Model** | Zero-Knowledge Proofs (implicit) | Transparent (answers are public) |
| **State Model** | Records + Finalize blocks | Smart contract storage (ledger) |
| **Assets** | Custom multisig pieces | Native Stellar assets |
| **Execution** | Asynchronous proofs | Synchronous transactions |
| **Auth** | Signature-based | Address-based auth |

---

## Core Game Logic Mapping

### Aleo Transitions → Soroban Functions

| Aleo Transition | Soroban Function | Changes |
|-----------------|------------------|---------|
| `propose_game` | `propose_game` | Removes ZK commitment, stores plaintext answer |
| `submit_wager` | `submit_wager` | Opponent directly submits their answer |
| `accept_game` | `accept_game` | Simplified state transition |
| `reveal_answer_game` | `reveal_answer` | Automatic winner determination (no reveal needed) |
| `finish_game` | `finish_game` | Distribute pot synchronously |
| `finish_game_by_timeout` | `finish_game_by_timeout` | Same logic, different block time tracking |
| `challenger_renege_stake` | `challenger_renege_stake` | Return stake before opponent matches |
| `opponent_renege_stake` | `opponent_renege_stake` | Return stakes before game accepted |

---

## Game Flow: Stellar vs Aleo

### Stellar (Soroban) Flow

```
1. Challenger calls propose_game(opponent, wager, answer)
   → Creates Game record with state = PROPOSED
   → Emit GameProposed event

2. Opponent calls submit_wager(game_id, wager, answer)
   → Validates wager matches
   → Updates Game state to WAGER_MATCHED
   → Emit GameAccepted event

3. Either player calls accept_game(game_id)
   → Game moves to ACCEPTED state
   → Timeout tracking starts

4. Challenger calls reveal_answer(game_id)
   → Compare answers: challenger_answer vs opponent_answer
   → Determine winner automatically
   → Game state = REVEALED
   → Emit GameRevealed event

5. Winner calls finish_game(game_id, token)
   → Transfer pot to winner
   → Game state = FINISHED
   → Emit GameFinished event
```

### Aleo (Leo) Flow (for reference)

```
1. Challenger: propose_game(wager_record, answer_hash, nonce)
   → ZK commit to answer
   → Create Game + notifications
   → State = PROPOSED

2. Opponent: submit_wager(wager_record)
   → Match wager
   → State = WAGER_MATCHED

3. Multisig: accept_game()
   → Combine stakes
   → State = ACCEPTED

4. Challenger: reveal_answer_game(answer_record)
   → Reveal plaintext + verify ZK proof
   → Determine winner
   → State = REVEALED

5. Multisig: finish_game()
   → Distribute pot from joint stake
   → State = FINISHED
```

---

## Privacy Trade-offs

### Aleo: Zero-Knowledge Privacy
- Answers stored as cryptographic commitments
- Only revealed at end of game
- Challenger can't see opponent's answer until reveal
- Opponent can't cheat because of ZK verification

### Stellar: Transparent Answers
- Both players' answers visible from step 2 onwards
- Game logic still enforces fairness
- Lower computational cost
- Better ecosystem integration

**Why this works**: Game logic is enforced on-chain. Even though both answers are known, the smart contract determines winner based on matching logic, not human judgment.

---

## State Management

### Soroban Storage

```rust
pub struct Game {
    pub id: String,                    // Unique game ID
    pub challenger: Address,           // Proposer
    pub opponent: Address,             // Challenged player
    pub challenger_wager: i128,        // Stake
    pub opponent_wager: i128,          // Matching stake
    pub challenger_answer: u32,        // 0 or 1 (stored plainly)
    pub opponent_answer: u32,          // 0 or 1 (stored plainly)
    pub total_pot: i128,               // Combined wager
    pub state: i128,                   // PROPOSED | WAGER_MATCHED | ACCEPTED | REVEALED | FINISHED
    pub created_at: u64,               // Timestamp
    pub accepted_at: u64,              // For timeout tracking
}
```

Games stored by ID in contract ledger:
```
storage.instance().set(game_id, game)
```

---

## Asset Handling

### Aleo Approach
- Custom token contract (`TraceToken`)
- Programmable multisig pieces
- Stake records carry ownership

### Stellar Approach
- Native Stellar asset or issued asset
- Simple transfer calls
- Ledger manages balances

**Implementation Note**: The `transfer_token` function can use:
1. **Native XLM**: For testing/demos
2. **Issued asset**: Custom "Trace" token on Stellar network

---

## Deployment Steps

### 1. Build the Contract

```bash
cd soroban
cargo build --target wasm32-unknown-unknown --release
```

### 2. Prepare for Stellar Network

```bash
# Generate contract WASM
soroban contract build

# Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
  --network testnet \
  --source your_keypair
```

### 3. Initialize Game Assets

```bash
# Create or reference token
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source your_keypair \
  -- initialize_token
```

---

## Key Differences from Aleo

### 1. No ZK Commitments
- Aleo: `BHP256::commit_to_field(answer, nonce)`
- Soroban: Direct storage

### 2. No Record Consumption
- Aleo: Records consumed/created in transitions
- Soroban: Storage reads/writes

### 3. Timeout Handling
- Aleo: Finalize block checks
- Soroban: Ledger sequence tracking

### 4. Event Model
- Aleo: Notification records
- Soroban: Contract events (published to ledger)

### 5. Authorization
- Aleo: Signature-based multisig
- Soroban: Address-based `require_auth()`

---

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests (Stellar Network)

```bash
# Use soroban-cli to test on testnet
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- propose_game \
  --challenger <ADDR1> \
  --opponent <ADDR2> \
  --wager 1000000
```

---

## Migration Path

If you already have games on Aleo:

1. **Export** game state from Aleo contracts
2. **Transform** records to Soroban Game struct
3. **Import** into Soroban contract storage
4. **Notify** players of network migration

---

## Security Considerations

### Addressed
- ✅ Proper state transitions
- ✅ Authorization checks (`require_auth`)
- ✅ Timeout mechanisms
- ✅ Stake protection (renegation limits)

### Additional Recommended
- ⚠️ Oracle for block time tracking
- ⚠️ Rate limiting on game creation
- ⚠️ Dispute resolution for edge cases
- ⚠️ Formal verification of winner logic

---

## Future Enhancements

1. **Tournament Mode**: Multiple games, leaderboards
2. **Seasonal Tokens**: Limited-time game assets
3. **Privacy Enhancement**: Use Stellar threshold cryptography for optional privacy
4. **Cross-chain Bridging**: Wrap Stellar tokens on other chains
5. **Replay Protection**: Nonce tracking for game uniqueness

---

## Reference

- [Soroban Docs](https://developers.stellar.org/docs/learn/soroban)
- [Soroban SDK Rust](https://docs.rs/soroban-sdk/)
- [Stellar Asset Docs](https://developers.stellar.org/docs/learn/glossary/assets)
- [Original Aleo Contracts](../program/src/main.leo)
