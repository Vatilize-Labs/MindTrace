# Building & Deploying MindTrace on Stellar Soroban

## Prerequisites

1. **Rust & Cargo** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Stellar CLI (soroban-cli)**
   ```bash
   cargo install soroban-cli
   ```

3. **Stellar Account** (Testnet or Mainnet)
   - Get free testnet account: https://lab.stellar.org

4. **Stellar Keypair**
   ```bash
   soroban keys generate --name my_key
   # Save secret key securely
   ```

---

## Building the Contract

### Step 1: Navigate to Soroban Directory
```bash
cd soroban
```

### Step 2: Build WASM Binary
```bash
# Build optimized WASM for Soroban
cargo build --target wasm32-unknown-unknown --release

# Output: soroban/target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm
```

### Step 3: Verify Build
```bash
ls -lh target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm
# Should be ~500KB-1MB
```

---

## Deploying to Stellar Network

### Testnet Deployment

#### 1. Set Network Configuration
```bash
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org/ \
  --network-passphrase "Test SDF Network ; September 2015"
```

#### 2. Get Test XLM
```bash
# Fund your testnet account via:
# https://lab.stellar.org

# Or use faucet (if available)
soroban config identity fund --name my_key
```

#### 3. Deploy Contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
  --network testnet \
  --source-account my_key
```

**Output**: Contract ID (save this!)
```
Contract ID: CADEA...
```

#### 4. Verify Deployment
```bash
# Check contract exists
soroban contract info \
  --id CADEA... \
  --network testnet
```

---

## Interacting with the Contract

### 1. Propose a Game

```bash
# First, create test addresses
CHALLENGER="GXXXXXXX..." # Your account
OPPONENT="GYYYYYY..."   # Another account
WAGER=1000000000        # 100 XLM (stroops)

soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account my_key \
  -- propose_game \
  --challenger $CHALLENGER \
  --opponent $OPPONENT \
  --challenger-wager $WAGER \
  --challenger-answer 0
```

**Returns**: Game ID (e.g., `game-2025-06-12-001`)

### 2. Opponent Submits Wager

```bash
GAME_ID="game-2025-06-12-001"
OPPONENT_ACCOUNT="opponent_key"

soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account $OPPONENT_ACCOUNT \
  -- submit_wager \
  --game-id "$GAME_ID" \
  --opponent $OPPONENT \
  --opponent-wager $WAGER \
  --opponent-answer 0
```

### 3. Accept Game

```bash
soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account my_key \
  -- accept_game \
  --game-id "$GAME_ID"
```

### 4. Reveal Answer

```bash
soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account my_key \
  -- reveal_answer \
  --game-id "$GAME_ID"
```

**Returns**: Winner address

### 5. Finish Game

```bash
TOKEN="CADEA..."  # Native XLM or custom token

soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account my_key \
  -- finish_game \
  --game-id "$GAME_ID" \
  --token $TOKEN
```

---

## Mainnet Deployment

### Prerequisites
- Funded mainnet Stellar account (with XLM)
- Production-ready code review
- Security audit recommended

### Deployment Steps

#### 1. Configure Mainnet Network
```bash
soroban config network add mainnet \
  --rpc-url https://soroban-mainnet.stellar.org/ \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

#### 2. Deploy
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
  --network mainnet \
  --source-account my_key
```

#### 3. Save Contract ID
```bash
# Save in file for reference
echo "MAINNET_CONTRACT_ID=CADEA..." > .env.mainnet
```

---

## Testing

### Unit Tests
```bash
cd soroban
cargo test
```

### Integration Testing

#### Create Test Script
```bash
# test_game.sh
#!/bin/bash

CONTRACT_ID="CADEA..."
TESTNET="--network testnet"
SOURCE="--source-account my_key"

# Test 1: Propose game
echo "=== Test 1: Propose Game ==="
GAME_ID=$(soroban contract invoke \
  --id $CONTRACT_ID $TESTNET $SOURCE \
  -- propose_game \
  --challenger GXXXXXXX \
  --opponent GYYYYYY \
  --challenger-wager 1000000000 \
  --challenger-answer 0 | jq -r '..')

echo "Game ID: $GAME_ID"

# Test 2: Submit wager
echo "=== Test 2: Submit Wager ==="
soroban contract invoke \
  --id $CONTRACT_ID $TESTNET \
  --source-account opponent_key \
  -- submit_wager \
  --game-id "$GAME_ID" \
  --opponent GYYYYYY \
  --opponent-wager 1000000000 \
  --opponent-answer 0

# Test 3: Get game state
echo "=== Test 3: Get Game State ==="
soroban contract invoke \
  --id $CONTRACT_ID $TESTNET $SOURCE \
  -- get_game \
  --game-id "$GAME_ID"
```

#### Run Tests
```bash
chmod +x test_game.sh
./test_game.sh
```

---

## Monitoring & Debugging

### View Contract Events
```bash
# Get recent transactions
soroban events \
  --id CADEA... \
  --network testnet \
  --limit 10
```

### Check Game State
```bash
soroban contract invoke \
  --id CADEA... \
  --network testnet \
  --source-account my_key \
  -- get_game \
  --game-id "game-2025-06-12-001" \
  --output json
```

### Common Issues

| Issue | Solution |
|-------|----------|
| **"Insufficient balance"** | Fund account with XLM |
| **"Contract not found"** | Check contract ID, verify network |
| **"Invalid game state"** | Verify game transitioned correctly |
| **"Not authorized"** | Ensure correct source account |
| **"Game not found"** | Verify game ID spelling |

---

## Performance Considerations

### Gas Costs (Approximate)

| Operation | Est. Cost (stroops) |
|-----------|-------------------|
| **propose_game** | 5,000 |
| **submit_wager** | 5,000 |
| **accept_game** | 3,000 |
| **reveal_answer** | 3,000 |
| **finish_game** | 5,000 |
| **get_game** | 1,000 |

**Total per game**: ~21,000 stroops (~0.002 XLM)

### Optimization Tips
1. Batch operations where possible
2. Minimize storage writes
3. Use indexed game IDs for quick lookups
4. Cache game state on frontend

---

## Upgrading the Contract

### Scenario: Bug Fix

1. **Create new version**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Deploy new contract**
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
     --network testnet \
     --source-account my_key
   ```

3. **Migrate game data** (if state structure changed)
   ```bash
   # Export old game state
   # Transform to new schema
   # Import to new contract
   ```

4. **Notify players** of contract migration

### Scenario: Feature Addition

- New functions don't break existing ones
- Deploy new version alongside old
- Support both contracts during transition
- Deprecate old contract after migration period

---

## Production Checklist

- [ ] Code reviewed by security expert
- [ ] All tests passing
- [ ] Testnet deployment verified
- [ ] Gas cost estimates within budget
- [ ] Monitoring/logging configured
- [ ] Disaster recovery plan ready
- [ ] User documentation updated
- [ ] Legal review complete
- [ ] Mainnet deployment approved
- [ ] Audit trail configured

---

## Resources

- [Soroban Documentation](https://developers.stellar.org/docs/learn/soroban)
- [Soroban CLI Reference](https://developers.stellar.org/docs/reference/soroban-cli)
- [Stellar Network Architecture](https://developers.stellar.org/docs/learn/networks)
- [Contract Best Practices](https://developers.stellar.org/docs/learn/soroban/guides/best-practices)
- [Rust SDK Docs](https://docs.rs/soroban-sdk/)

---

## Support

For issues or questions:
1. Check Stellar Discord: #soroban
2. GitHub Issues: https://github.com/stellar/rs-soroban-sdk
3. Stellar Docs: developers.stellar.org
