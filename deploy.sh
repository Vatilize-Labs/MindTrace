#!/bin/bash

# MindTrace Soroban Deployment Script
# This script guides you through deploying MindTrace to Stellar testnet

set -e

echo ""
echo "╔════════════════════════════════════════╗"
echo "║  MindTrace Soroban Deployment Script   ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Install Rust from https://rustup.rs"
    exit 1
fi

if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI not found."
    echo "   Install with: cargo install soroban-cli"
    exit 1
fi

# The contract lives in its own repo: https://github.com/Vatilize-Labs/smart-contract
# Default: a checkout next to this repo. Override with CONTRACT_DIR.
CONTRACT_DIR="${CONTRACT_DIR:-../smart-contract}"

if [ ! -f "$CONTRACT_DIR/Cargo.toml" ]; then
    echo "❌ Contract repo not found at $CONTRACT_DIR"
    echo "   Clone it with: git clone https://github.com/Vatilize-Labs/smart-contract.git $CONTRACT_DIR"
    echo "   Or set CONTRACT_DIR to your checkout."
    exit 1
fi

echo "✅ Prerequisites OK"
echo ""

# Step 1: Build
echo "📦 Step 1: Building WASM binary..."
cd "$CONTRACT_DIR"
cargo build --target wasm32-unknown-unknown --release
cd - > /dev/null
echo "✅ Build complete"
echo ""

# Step 2: Check if account exists
echo "🔑 Step 2: Checking Stellar account..."
ACCOUNT_NAME="mindtrace_testnet"

if soroban keys list 2>/dev/null | grep -q "$ACCOUNT_NAME"; then
    echo "✅ Account '$ACCOUNT_NAME' found"
else
    echo "⚠️  Account '$ACCOUNT_NAME' not found"
    echo "   Creating new account..."
    soroban keys generate --name "$ACCOUNT_NAME"
    echo "✅ Account created"
fi
echo ""

# Step 3: Get account address
echo "📍 Step 3: Getting account address..."
ACCOUNT=$(soroban keys show "$ACCOUNT_NAME")
echo "✅ Account: $ACCOUNT"
echo ""

# Step 4: Check account funding
echo "💰 Step 4: Checking account balance..."
BALANCE=$(soroban account info --account "$ACCOUNT" --network testnet 2>/dev/null | grep "balance" | head -1 || echo "0")

if [ "$BALANCE" = "0" ]; then
    echo "⚠️  Account has no balance!"
    echo "   Please fund your account with testnet XLM:"
    echo "   1. Go to https://lab.stellar.org"
    echo "   2. Paste your address: $ACCOUNT"
    echo "   3. Click 'Fund with test XLM'"
    echo "   4. Run this script again"
    exit 1
fi

echo "✅ Account is funded: $BALANCE"
echo ""

# Step 5: Configure network (if not already done)
echo "🌐 Step 5: Configuring testnet..."
soroban config network add testnet \
    --rpc-url https://soroban-testnet.stellar.org/ \
    --network-passphrase "Test SDF Network ; September 2015" \
    2>/dev/null || echo "   (network already configured)"
echo "✅ Network configured"
echo ""

# Step 6: Deploy contract
echo "🚀 Step 6: Deploying contract..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm "$CONTRACT_DIR/target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm" \
    --network testnet \
    --source-account "$ACCOUNT_NAME" \
    2>&1 | grep -oP 'Contract ID: \K[^ ]+' || echo "")

if [ -z "$CONTRACT_ID" ]; then
    echo "❌ Deployment failed. Check your internet connection and try again."
    exit 1
fi

echo "✅ Contract deployed!"
echo "   Contract ID: $CONTRACT_ID"
echo ""

# Step 7: Save configuration
echo "💾 Step 7: Saving configuration..."
cat > .env.testnet << EOF
# MindTrace Testnet Configuration
export CONTRACT_ID=$CONTRACT_ID
export ACCOUNT=$ACCOUNT
export ACCOUNT_NAME=$ACCOUNT_NAME
export NETWORK=testnet
EOF
chmod 600 .env.testnet
echo "✅ Configuration saved to .env.testnet"
echo ""

# Success!
echo "╔════════════════════════════════════════╗"
echo "║     ✅ Deployment Successful!          ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "📝 Next steps:"
echo ""
echo "1. Load configuration:"
echo "   source .env.testnet"
echo ""
echo "2. Test the contract:"
echo "   soroban contract invoke --id \$CONTRACT_ID --network testnet \\"
echo "     --source-account \$ACCOUNT_NAME -- get_game --game-id \$ACCOUNT"
echo ""
echo "3. Propose a game:"
echo "   export OPPONENT=GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"
echo "   soroban contract invoke --id \$CONTRACT_ID --network testnet \\"
echo "     --source-account \$ACCOUNT_NAME -- propose_game \\"
echo "     --challenger \$ACCOUNT --opponent \$OPPONENT \\"
echo "     --challenger-wager 1000000 --challenger-answer 0"
echo ""
echo "4. Read the guides:"
echo "   - DEPLOYMENT_CHECKLIST.md (step-by-step)"
echo "   - SOROBAN_QUICK_REFERENCE.md (function signatures)"
echo "   - BUILD_SUCCESS_SUMMARY.md (overview)"
echo ""
