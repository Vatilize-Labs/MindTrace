# Stellar Wallet Integration Guide

> **Note:** The Soroban contract has moved to its own repository:
> <https://github.com/Vatilize-Labs/smart-contract>. Contract build/deploy
> commands below run from that repo's root — paths like `soroban/` or
> `cd soroban` refer to the old monorepo layout.


## Overview

The MindTrace frontend now supports **Stellar wallets** instead of the previous Puzzle wallet integration. This enables seamless interaction with your Soroban smart contract on the Stellar network.

---

## What Changed

### Removed
- ❌ Puzzle SDK (`@puzzlehq/sdk`)
- ❌ PuzzleWalletProvider
- ❌ Aleo-specific wallet logic

### Added
- ✅ Freighter Wallet integration
- ✅ Stellar Soroban RPC client
- ✅ Custom StellarWalletContext for state management
- ✅ Environment configuration support

---

## Files Modified/Created

### New Files
```
src/
├── context/
│   └── StellarWalletContext.tsx      # Wallet context provider
└── utils/
    └── sorobanClient.ts              # Soroban RPC utilities
.env.example                           # Environment template
STELLAR_WALLET_INTEGRATION.md          # This file
```

### Modified Files
```
src/
├── main.tsx                          # Updated provider
├── App.tsx                           # Uses useStellarWallet hook
├── components/
│   └── Header.tsx                    # Stellar wallet connection
└── pages/
    └── Welcome.tsx                   # Updated copy & error handling
```

---

## Setup Instructions

### 1. Install Freighter Wallet

Freighter is the recommended wallet for Stellar development and testing.

**Get it here**: https://www.freighter.app/

Steps:
1. Visit https://www.freighter.app/
2. Click "Install" for your browser (Chrome, Firefox, etc.)
3. Create or import a Stellar account
4. You're ready to go!

### 2. Configure Environment

Create a `.env.local` file in the project root:

```bash
cp .env.example .env.local
```

Edit `.env.local`:

```env
# Your deployed MindTrace contract ID
VITE_CONTRACT_ID=CADEA5E2XW6B7XWKBNWZTOPD5XGCXQHHWVD5DPSW5QFAIPQUQMCOJLV

# Soroban RPC URL (testnet or mainnet)
VITE_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
```

**To get your CONTRACT_ID**:
- Deploy using: `./deploy.sh` or `DEPLOYMENT_CHECKLIST.md`
- The deployment will output your CONTRACT_ID
- Save it in `.env.local`

### 3. Fund Your Testnet Account

If using testnet, you need test XLM:

1. Get your Stellar address from Freighter
2. Visit: https://lab.stellar.org
3. Paste your address
4. Click "Fund with test XLM"
5. Done! You'll have 10,000 test XLM

### 4. Start the Frontend

```bash
npm run dev
```

Open: http://localhost:4000/

---

## How It Works

### 1. User Connects Wallet

```typescript
import { useStellarWallet } from '@context/StellarWalletContext';

const MyComponent = () => {
  const { account, connect, disconnect, loading } = useStellarWallet();

  return (
    <>
      <button onClick={connect} disabled={loading}>
        {account ? 'Connected' : 'Connect Wallet'}
      </button>
      {account && <p>Account: {account}</p>}
    </>
  );
};
```

### 2. Wallet Context Manages State

The `StellarWalletContext` provides:

```typescript
interface WalletContextType {
  account: string | null;           // Connected Stellar address
  loading: boolean;                 // Connection loading state
  error: string | null;             // Error message if any
  connect: () => Promise<void>;     // Connect to Freighter
  disconnect: () => void;           // Disconnect wallet
  isConnected: boolean;             // Quick connected check
  walletType: 'freighter' | null;   // Current wallet type
}
```

### 3. Contract Interaction

Use `sorobanClient` to interact with your contract:

```typescript
import { sorobanClient, getContractId } from '@utils/sorobanClient';

// Example: Propose a game
const proposeGame = async (
  opponent: string,
  wager: number,
  answer: number
) => {
  const contractId = getContractId();

  // Build transaction with soroban-js-sdk
  // (Full SDK integration details in next section)
};
```

---

## Soroban SDK Integration (Next Steps)

To enable full contract interaction, you'll need to install the Stellar JavaScript SDK:

```bash
npm install @stellar/js-sdk
```

Then use it for contract calls:

```typescript
import {
  Contract,
  Keypair,
  Networks,
  SorobanRpc,
  TransactionBuilder,
} from '@stellar/js-sdk';

const contract = new Contract(contractId);

// Example: Call propose_game
const transaction = new TransactionBuilder(account, {
  fee: 100,
  networkPassphrase: Networks.TESTNET_NETWORK_PASSPHRASE,
})
  .addOperation(
    contract.call(
      'propose_game',
      opponentAddress,
      wagerAmount,
      challengerAnswer
    )
  )
  .setTimeout(30)
  .build();

// Sign with Freighter
const signedTx = await window.freighter.signTransaction(transaction);

// Submit to Soroban
const result = await sorobanClient.submitTransaction(signedTx);
```

---

## API Reference

### StellarWalletContext

```typescript
// Provider (in main.tsx)
<StellarWalletProvider>
  <App />
</StellarWalletProvider>

// Hook (in components)
const { account, connect, disconnect, loading, error } = useStellarWallet();
```

### sorobanClient

```typescript
import { sorobanClient, getContractId } from '@utils/sorobanClient';

// Get contract ID from environment
const contractId = getContractId();

// Soroban RPC server instance
const response = await sorobanClient.getLatestLedger();
```

---

## Common Tasks

### Get Current Account

```typescript
const { account } = useStellarWallet();
console.log('Current account:', account); // GDXST...
```

### Handle Connection

```typescript
const { connect, loading, error } = useStellarWallet();

const handleConnect = async () => {
  try {
    await connect();
    console.log('Connected!');
  } catch (err) {
    console.error('Connection failed:', err);
  }
};
```

### Disconnect

```typescript
const { disconnect } = useStellarWallet();

const handleDisconnect = () => {
  disconnect();
  // User is now disconnected
};
```

### Check Connection Status

```typescript
const { account, isConnected } = useStellarWallet();

if (isConnected) {
  // Render game interface
} else {
  // Render welcome screen
}
```

---

## Troubleshooting

### Error: "Freighter wallet not found"

**Solution**: Install Freighter from https://www.freighter.app/

### Error: "CONTRACT_ID not configured"

**Solution**: 
1. Deploy your contract: `./deploy.sh`
2. Get the CONTRACT_ID from the output
3. Add to `.env.local`: `VITE_CONTRACT_ID=CADEA...`

### Error: "Network unreachable"

**Solution**:
1. Check Soroban RPC URL is correct
2. Verify internet connection
3. Try: https://soroban-testnet.stellar.org

### Wallet not auto-connecting

**Solution**:
1. Check browser console for errors
2. Make sure Freighter is installed and unlocked
3. Try manually clicking "Play!" button

---

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `VITE_CONTRACT_ID` | Yes | — | Your deployed contract ID |
| `VITE_SOROBAN_RPC_URL` | No | testnet | Soroban RPC endpoint |

**How to set**:
```bash
# Create .env.local
VITE_CONTRACT_ID=CADEA5E2XW6B7...
VITE_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
```

---

## Testing Locally

### 1. Start Frontend
```bash
npm run dev
# http://localhost:4000/
```

### 2. Open in Browser
```
http://localhost:4000/
```

### 3. Connect Wallet
- Click "Play!"
- Freighter pops up
- Approve connection
- Ready to play!

### 4. Test Game Flow
- Propose a game (requires contract deployed)
- Wait for opponent
- Accept game
- Reveal answer
- Finish and claim rewards

---

## Mainnet Deployment

To deploy to Stellar mainnet:

1. **Update environment**:
   ```bash
   VITE_SOROBAN_RPC_URL=https://soroban-mainnet.stellar.org
   VITE_CONTRACT_ID=<mainnet-contract-id>
   ```

2. **Deploy contract to mainnet**:
   ```bash
   soroban contract deploy \
     --wasm soroban/target/wasm32-unknown-unknown/release/mindtrace_soroban.wasm \
     --network mainnet \
     --source-account <your-account>
   ```

3. **Update Freighter to mainnet**
4. **Fund account with real XLM**
5. **Deploy frontend**

---

## Resources

- **Freighter Wallet**: https://www.freighter.app/
- **Stellar Docs**: https://developers.stellar.org/
- **Soroban Docs**: https://developers.stellar.org/docs/learn/soroban
- **JS SDK Docs**: https://js-stellar-sdk.readthedocs.io/
- **Lab (Testing)**: https://lab.stellar.org/

---

## Next Steps

1. ✅ Install Freighter wallet
2. ✅ Set up `.env.local` with your CONTRACT_ID
3. ✅ Start frontend: `npm run dev`
4. ✅ Test wallet connection
5. 📋 (Optional) Implement full contract interaction using Stellar SDK

---

## Support

- **Issues**: Check browser console for errors
- **Freighter Help**: https://www.freighter.app/
- **Soroban Questions**: Stellar Discord/Community
- **Code Changes**: See modified files listed above
