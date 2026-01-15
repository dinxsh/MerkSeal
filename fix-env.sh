#!/bin/bash

# Fix script to update .env with correct Mantle Sepolia RPC URL

echo "🔧 Fixing Mantle RPC URL in .env..."

if [ ! -f .env ]; then
    echo "❌ .env file not found, copying from .env.example"
    cp .env.example .env
    echo "✅ Created .env from template"
    echo "⚠️  Please update your PRIVATE_KEY in .env"
    exit 0
fi

# Update RPC URL
sed -i 's|MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz|MANTLE_RPC_URL=https://rpc.sepolia.mantle.xyz|g' .env

# Update Chain ID
sed -i 's|MANTLE_CHAIN_ID=5001|MANTLE_CHAIN_ID=5003|g' .env

echo "✅ Updated .env with correct Mantle Sepolia configuration"
echo "   RPC URL: https://rpc.sepolia.mantle.xyz"
echo "   Chain ID: 5003"
echo ""
echo "Current .env configuration:"
grep "MANTLE_" .env
