#!/bin/bash

# MindTrace Stellar Wallet Setup Script

echo ""
echo "╔══════════════════════════════════════════════════╗"
echo "║   MindTrace Stellar Wallet Setup Helper          ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""

# Check if .env.local exists
if [ -f ".env.local" ]; then
    echo "✅ .env.local already exists"
else
    echo "📝 Creating .env.local from template..."
    cp .env.example .env.local
    echo "✅ Created .env.local"
    echo ""
    echo "⚠️  IMPORTANT: Edit .env.local and add your CONTRACT_ID:"
    echo "   VITE_CONTRACT_ID=CADEA5E2XW6B7..."
    echo ""
fi

echo "📋 Stellar Wallet Setup Checklist:"
echo ""
echo "1. ✅ Created .env.local (see above for next steps)"
echo ""
echo "2. Install Freighter Wallet (if not already installed):"
echo "   → https://www.freighter.app/"
echo ""
echo "3. Deploy your smart contract (if not already deployed):"
echo "   → ./deploy.sh"
echo "   → or see DEPLOYMENT_CHECKLIST.md"
echo ""
echo "4. Get your CONTRACT_ID:"
echo "   → From deployment output"
echo "   → Add to .env.local: VITE_CONTRACT_ID=..."
echo ""
echo "5. Start the frontend:"
echo "   → npm run dev"
echo ""
echo "6. Open in browser:"
echo "   → http://localhost:4000/"
echo ""
echo "7. Click 'Play!' to connect Freighter"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 For more details, read:"
echo "   STELLAR_WALLET_INTEGRATION.md"
echo ""
echo "✅ Setup helper complete!"
echo ""
