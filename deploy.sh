#!/bin/bash

# MerkleBatchRegistry Deployment Script for Mantle Testnet
# Usage: ./deploy.sh

set -e

echo "=================================="
echo "MerkSeal Contract Deployment"
echo "=================================="
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "❌ .env file not found!"
    echo "   Copy .env.example to .env and fill in your values"
    exit 1
fi

# Load environment variables
source .env

# Validate required variables
if [ -z "$MANTLE_RPC_URL" ]; then
    echo "❌ MANTLE_RPC_URL not set in .env"
    exit 1
fi

if [ -z "$PRIVATE_KEY" ]; then
    echo "❌ PRIVATE_KEY not set in .env"
    exit 1
fi

echo "📋 Configuration:"
echo "   RPC URL: $MANTLE_RPC_URL"
echo "   Chain ID: ${MANTLE_CHAIN_ID:-5001}"
echo ""

cd contracts

echo "🔨 Deploying MerkleBatchRegistry..."
echo ""

# Use forge script for deployment (more reliable than forge create)
forge script script/Deploy.s.sol \
    --rpc-url $MANTLE_RPC_URL \
    --broadcast \
    --private-key $PRIVATE_KEY \
    --legacy \
    -vvv

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "   1. Copy the deployed contract address from above"
echo "   2. Update .env: MERKLE_BATCH_REGISTRY_ADDRESS=<address>"
echo "   3. Run client tests: cargo run --release -p client config"
echo ""
