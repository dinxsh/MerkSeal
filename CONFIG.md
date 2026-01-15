# Task 2: Mantle Configuration Integration

## Overview
This document explains how the Mantle configuration system works in MerkSeal.

---

## Architecture

### Shared Config Crate: `mantle_config/`

A dedicated Rust crate that provides:
- **`MantleConfig` struct** - Holds RPC URL, chain ID, and registry address
- **Environment variable loading** - `from_env()` method with sensible defaults
- **Helper methods** - Network detection, explorer URL generation

**Key Features**:
- ✅ Defaults to Mantle testnet if not specified
- ✅ Validates required variables (fails fast if registry address missing)
- ✅ Provides helper methods for common tasks (tx URLs, contract URLs)
- ✅ Shared between client and server (DRY principle)

---

## Environment Variables

### Required
- **`MERKLE_BATCH_REGISTRY_ADDRESS`** - Deployed contract address (no default)

### Optional (with defaults)
- **`MANTLE_RPC_URL`** - Default: `https://rpc.testnet.mantle.xyz`
- **`MANTLE_CHAIN_ID`** - Default: `5003` (testnet)

---

## Setup Instructions

### 1. Create `.env` file
```bash
# Copy the example
cp .env.example .env

# Edit with your values
# IMPORTANT: Add the contract address after deployment!
```

### 2. Set Required Variables
```bash
# After deploying MerkleBatchRegistry (see contracts/DEPLOYMENT.md)
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourDeployedContractAddress
```

### 3. Test Configuration
```bash
# Test client
cd client
cargo run

# Test server
cd ../server
cargo run
```

**Expected output**:
```
✓ Mantle config loaded:
  RPC URL: https://rpc.testnet.mantle.xyz
  Chain ID: 5003
  Registry: 0x...
  Network: Testnet
  Explorer: https://explorer.testnet.mantle.xyz/address/0x...
```

---

## Usage in Code

### Loading Config
```rust
use mantle_config::MantleConfig;

fn main() {
    // Load from environment
    let config = MantleConfig::from_env()
        .expect("Failed to load Mantle config");
    
    println!("Using registry at: {}", config.registry_address);
}
```

### Creating Config Manually (Testing)
```rust
let config = MantleConfig::new(
    "https://rpc.testnet.mantle.xyz".to_string(),
    5003,
    "0x1234...".to_string(),
);
```

### Helper Methods
```rust
// Check network
if config.is_testnet() {
    println!("Running on testnet");
}

// Get explorer URLs
let tx_url = config.tx_url("0xdeadbeef...");
let contract_url = config.contract_url();
```

---

## Integration Points

### Client (`client/src/main.rs`)
- Loads config on startup
- Will use for:
  - Querying on-chain batch data
  - Verifying local roots against Mantle
  - Displaying explorer links

### Server (`server/src/main.rs`)
- Loads config on startup
- Will use for:
  - Returning registry address in upload responses
  - Generating metadata URIs
  - Logging on-chain anchoring info

---

## File Structure

```
merkletree/
├── .env.example          # Template (commit this)
├── .env                  # Your secrets (DO NOT COMMIT)
├── mantle_config/        # Shared config crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs        # MantleConfig implementation
├── client/
│   ├── Cargo.toml        # Depends on mantle_config
│   └── src/main.rs       # Uses MantleConfig
└── server/
    ├── Cargo.toml        # Depends on mantle_config
    └── src/main.rs       # Uses MantleConfig
```

---

## Troubleshooting

### "MERKLE_BATCH_REGISTRY_ADDRESS environment variable not set"
**Solution**: Deploy the contract first (see `contracts/DEPLOYMENT.md`), then add the address to `.env`

### Config not loading
**Solution**: Make sure `.env` is in the project root (same directory as `Cargo.toml`)

### Wrong network
**Solution**: Check `MANTLE_CHAIN_ID` in `.env`:
- Testnet: `5003`
- Mainnet: `5000`

---

## Next Steps (Task 3)

Now that config is wired up, we'll extend the server to:
- Accept file uploads
- Compute Merkle roots
- Return batch metadata (including registry address for anchoring)

---

**Task 2 Complete!** ✅
