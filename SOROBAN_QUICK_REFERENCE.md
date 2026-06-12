# Soroban Contract Quick Reference

## Function Signatures

### 1. `propose_game()`
**Initiates a new game**

```rust
pub fn propose_game(
    env: Env,
    challenger: Address,
    opponent: Address,
    challenger_wager: i128,
    challenger_answer: u32,  // 0 or 1
) -> String
```

**Returns**: Game ID (String)

**Emits**: `GameProposed` event

**State**: PROPOSED

**Example**:
```bash
soroban contract invoke --id CONTRACT \
  -- propose_game \
  --challenger G... \
  --opponent G... \
  --challenger-wager 1000000000 \
  --challenger-answer 0
```

---

### 2. `submit_wager()`
**Opponent matches challenger's wager**

```rust
pub fn submit_wager(
    env: Env,
    game_id: String,
    opponent: Address,
    opponent_wager: i128,      // Must equal challenger_wager
    opponent_answer: u32,      // 0 or 1
)
```

**Returns**: None (updates game state)

**Emits**: `GameAccepted` event

**State Transition**: PROPOSED → WAGER_MATCHED

**Requirements**:
- Caller must be opponent
- Wager must match challenger's wager exactly
- Answer must be 0 or 1

---

### 3. `accept_game()`
**Game transitions to active state (waiting for reveal)**

```rust
pub fn accept_game(
    env: Env,
    game_id: String,
)
```

**Returns**: None

**Emits**: None

**State Transition**: WAGER_MATCHED → ACCEPTED

**Note**: Timeout timer starts here

---

### 4. `reveal_answer()`
**Compare answers and determine winner**

```rust
pub fn reveal_answer(
    env: Env,
    game_id: String,
) -> Address
```

**Returns**: Winner's address

**Emits**: `GameRevealed` event

**State Transition**: ACCEPTED → REVEALED

**Logic**:
```
if opponent_answer == challenger_answer
    return opponent
else
    return challenger
```

---

### 5. `finish_game()`
**Distribute pot to winner**

```rust
pub fn finish_game(
    env: Env,
    game_id: String,
    token: Address,
)
```

**Returns**: None

**Emits**: `GameFinished` event

**State Transition**: REVEALED → FINISHED

**Action**: Transfer total_pot to winner

---

### 6. `finish_game_by_timeout()`
**Opponent wins if challenger doesn't reveal in time**

```rust
pub fn finish_game_by_timeout(
    env: Env,
    game_id: String,
    token: Address,
)
```

**Returns**: None

**Emits**: `GameFinished` event

**State Transition**: ACCEPTED → TIMEOUT

**Requirements**: 10,000+ blocks since accept

**Winner**: Always opponent

---

### 7. `challenger_renege_stake()`
**Challenger cancels before opponent matches**

```rust
pub fn challenger_renege_stake(
    env: Env,
    game_id: String,
    challenger: Address,
    token: Address,
)
```

**Returns**: None

**Emits**: None

**State Transition**: PROPOSED → RENEGED

**Action**: Refund challenger_wager to challenger

**Requirements**: Must be called by challenger, game in PROPOSED state

---

### 8. `opponent_renege_stake()`
**Opponent cancels after matching wager**

```rust
pub fn opponent_renege_stake(
    env: Env,
    game_id: String,
    opponent: Address,
    token: Address,
)
```

**Returns**: None

**Emits**: None

**State Transition**: WAGER_MATCHED → RENEGED

**Action**: Refund both wagers

**Requirements**: Must be called by opponent, game in WAGER_MATCHED state

---

### 9. `get_game()`
**Query game state (read-only)**

```rust
pub fn get_game(
    env: Env,
    game_id: String,
) -> Game
```

**Returns**: Full Game struct

**Emits**: None

**Struct**:
```rust
pub struct Game {
    pub id: String,
    pub challenger: Address,
    pub opponent: Address,
    pub challenger_wager: i128,
    pub opponent_wager: i128,
    pub challenger_answer: u32,
    pub opponent_answer: u32,
    pub total_pot: i128,
    pub state: i128,
    pub created_at: u64,
    pub accepted_at: u64,
}
```

---

## Game States

```rust
pub const GAME_STATE_PROPOSED: i128 = 1;         // Initial
pub const GAME_STATE_WAGER_MATCHED: i128 = 2;   // Opponent matched
pub const GAME_STATE_ACCEPTED: i128 = 3;        // Active (waiting reveal)
pub const GAME_STATE_REVEALED: i128 = 4;        // Winner determined
pub const GAME_STATE_FINISHED: i128 = 5;        // Pot distributed
pub const GAME_STATE_TIMEOUT: i128 = 6;         // Timeout end
pub const GAME_STATE_RENEGED: i128 = 0;         // Cancelled
```

---

## Answer Legend

```
0 = "in the weeds"
1 = "behind the building"

Winner Logic:
- If opponent_answer == challenger_answer → opponent wins
- If opponent_answer != challenger_answer → challenger wins
```

---

## Event Types

### GameProposed
```rust
pub struct GameProposed {
    pub game_id: String,
    pub challenger: Address,
    pub opponent: Address,
    pub wager: i128,
}
```

### GameAccepted
```rust
pub struct GameAccepted {
    pub game_id: String,
    pub total_pot: i128,
}
```

### GameRevealed
```rust
pub struct GameRevealed {
    pub game_id: String,
    pub challenger_answer: u32,
    pub opponent_answer: u32,
    pub winner: Address,
}
```

### GameFinished
```rust
pub struct GameFinished {
    pub game_id: String,
    pub winner: Address,
    pub pot: i128,
}
```

---

## Constants

```rust
pub const TIMEOUT_BLOCKS: u32 = 10_000;  // ~83 minutes
```

---

## Validation Rules

| Function | Validation |
|----------|-----------|
| `propose_game` | answer ∈ {0,1}, wager > 0 |
| `submit_wager` | state == PROPOSED, wager == challenger_wager |
| `accept_game` | state == WAGER_MATCHED |
| `reveal_answer` | state == ACCEPTED |
| `finish_game` | state == REVEALED |
| `finish_game_by_timeout` | state == ACCEPTED, blocks > TIMEOUT |
| `challenger_renege_stake` | state == PROPOSED, caller == challenger |
| `opponent_renege_stake` | state == WAGER_MATCHED, caller == opponent |

---

## Authorization

All functions that modify state require:
```rust
address.require_auth()  // Caller must have account signature
```

---

## Common Workflows

### Happy Path
```
1. Challenger calls propose_game()
   ↓ (gets game_id)
2. Opponent calls submit_wager()
   ↓
3. Either calls accept_game()
   ↓
4. Challenger calls reveal_answer()
   ↓ (gets winner address)
5. Winner calls finish_game()
   ↓
✓ Game complete, pot distributed
```

### Challenger Cancels Early
```
1. Challenger calls propose_game()
2. Challenger calls challenger_renege_stake()
   ↓
✓ Challenger gets refund, game RENEGED
```

### Opponent Cancels After Matching
```
1. Challenger calls propose_game()
2. Opponent calls submit_wager()
3. Opponent calls opponent_renege_stake()
   ↓
✓ Both get refunds, game RENEGED
```

### Timeout Path
```
1. Challenger calls propose_game()
2. Opponent calls submit_wager()
3. Either calls accept_game()
4. Wait 10,000+ blocks (~83 min)
5. Either calls finish_game_by_timeout()
   ↓
✓ Opponent wins by timeout
```

---

## Error Handling

| Error | Cause | Solution |
|-------|-------|----------|
| "Game not found" | Invalid game_id | Check game_id spelling |
| "Not authorized" | Wrong caller | Use correct account |
| "Invalid game state" | Wrong phase | Check game state first |
| "Answer must be 0 or 1" | Bad answer | Use 0 or 1 only |
| "Wager must be positive" | Zero/negative wager | Use positive amount |
| "Wager must match challenger" | Wrong amount | Match exact amount |
| "Timeout not yet reached" | Too early | Wait more blocks |

---

## Testing Checklist

- [ ] `propose_game()` creates game in PROPOSED state
- [ ] `submit_wager()` transitions to WAGER_MATCHED
- [ ] `accept_game()` transitions to ACCEPTED
- [ ] `reveal_answer()` returns correct winner
- [ ] `finish_game()` distributes pot to winner
- [ ] `finish_game_by_timeout()` gives pot to opponent
- [ ] `challenger_renege_stake()` refunds challenger
- [ ] `opponent_renege_stake()` refunds both
- [ ] `get_game()` returns correct state
- [ ] Events are emitted correctly
- [ ] Authorization checks work
- [ ] State transitions are validated

---

## Gas Estimates

| Function | Gas (stroops) | Cost (XLM) |
|----------|---------------|-----------|
| propose_game | 5,000 | 0.0005 |
| submit_wager | 5,000 | 0.0005 |
| accept_game | 3,000 | 0.0003 |
| reveal_answer | 3,000 | 0.0003 |
| finish_game | 5,000 | 0.0005 |
| finish_game_by_timeout | 5,000 | 0.0005 |
| challenger_renege_stake | 3,000 | 0.0003 |
| opponent_renege_stake | 3,000 | 0.0003 |
| get_game | 1,000 | 0.0001 |
| **Total per game** | **~21,000** | **~0.002** |

---

## Links

- [Soroban Docs](https://developers.stellar.org/docs/learn/soroban)
- [Stellar CLI](https://developers.stellar.org/docs/reference/soroban-cli)
- [Rust SDK](https://docs.rs/soroban-sdk/)
- [GitHub](https://github.com/stellar/rs-soroban-sdk)

---

## Example CLI Commands

### Setup
```bash
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org/ \
  --network-passphrase "Test SDF Network ; September 2015"

soroban keys generate --name my_key
soroban config identity fund --name my_key
```

### Deploy
```bash
cargo build --target wasm32-unknown-unknown --release

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
  --network testnet \
  --source-account my_key
```

### Invoke Function
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source-account my_key \
  -- function_name \
  --param1 value1 \
  --param2 value2
```

### Get Game
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  --source-account my_key \
  -- get_game \
  --game-id "game-2025-06-12-001" \
  --output json
```

---

## Tips & Tricks

1. **Save Contract ID**: `echo "CONTRACT_ID=..." > .env`
2. **Use Variables**: `source .env && soroban ... --id $CONTRACT_ID`
3. **Monitor Events**: Watch contract event logs in explorer
4. **Test Locally**: Use `cargo test` before testnet
5. **Keep Secrets Safe**: Never commit private keys
6. **Cache Game IDs**: Store in database for quick lookup
7. **Batch Operations**: Call multiple functions in sequence
8. **Monitor Gas**: Check gas costs in practice vs estimates
