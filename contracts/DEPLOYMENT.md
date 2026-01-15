# MerkleBatchRegistry Deployment Guide

## Issue Fixed ✅

The HTTP 400 error was caused by using the **old Mantle testnet RPC URL**. Mantle now uses **Sepolia Testnet** with a different RPC endpoint.

## Changes Made

1. **Updated `.env.example`**: Changed `MANTLE_CHAIN_ID` from 5003 to 5001
2. **Updated `mantle_config/src/lib.rs`**: Fixed all chain ID references
3. **Created deployment scripts**: Added `deploy.sh` (Linux/WSL) and `deploy.ps1` (PowerShell)

## Quick Start

### Step 1: Create your `.env` file

```bash
# In WSL or Linux
cp .env.example .env

# In PowerShell  
Copy-Item .env.example .env
```

### Step 2: Update your `.env`

Edit `.env` and set your private key:

```bash
PRIVATE_KEY=0xyour_private_key_here
```

⚠️ **Important**: Use a testnet wallet with  Mantle testnet ETH. Get testnet funds from [Mantle Faucet](https://faucet.testnet.mantle.xyz/)

### Step 3: Deploy using the script

**Option A: Using WSL/Linux (Recommended)**

```bash
bash deploy.sh
```

**Option B: Using PowerShell**

```powershell
.\deploy.ps1
```

**Option C: Manual deployment (if scripts fail)**

```bash
cd contracts

# Using forge script (recommended)
forge script script/Deploy.s.sol \
  --rpc-url https://rpc.testnet.mantle.xyz \
  --broadcast \
  --private-key $PRIVATE_KEY \
  --legacy \
  -vvv

# OR using forge create directly
forge create --rpc-url https://rpc.testnet.mantle.xyz \
  --private-key $PRIVATE_KEY \
  --legacy \
  MerkleBatchRegistry
```

### Step 4: Save the contract address

After successful deployment, copy the contract address and update `.env`:

```bash
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourDeployedContractAddress
```

### Step 5: Verify configuration

```bash
cargo run --release -p client config
```

## Why the `--legacy` flag?

Mantle testnet may not fully support EIP-1559 transactions. The `--legacy` flag forces Foundry to use legacy transaction format (with `gasPrice` instead of `maxFeePerGas`/`maxPriorityFeePerGas`).

## Network Details

> ⚠️ **IMPORTANT**: The old "Mantle Testnet" (RPC `rpc.testnet.mantle.xyz`) is **OFFLINE** (returns 404). You **MUST** use Mantle Sepolia Testnet.

- **Network**: Mantle Sepolia Testnet
- **RPC URL**: https://rpc.sepolia.mantle.xyz
- **Chain ID**: 5003
- **Explorer**: https://explorer.sepolia.mantle.xyz
- **Faucet**: https://faucet.sepolia.mantle.xyz

## Troubleshooting

### Still getting HTTP 400?

1. Verify Chain ID is 5001 in `.env`
2. Try using the `--legacy` flag
3. Check your private key format (should start with `0x`)
4. Ensure you have testnet ETH in your wallet

### Transaction underpriced?

Increase gas price manually:

```bash
forge create --rpc-url https://rpc.testnet.mantle.xyz \
  --private-key $PRIVATE_KEY \
  --legacy \
  --gas-price 20000000000 \
  MerkleBatchRegistry
```

### RPC connection issues?

Try alternative RPC (if available) or check your internet connection.
