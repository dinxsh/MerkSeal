# Anchor Script Documentation

## Overview

The anchor script registers MerkSeal batch roots on the Mantle L2 blockchain, creating an immutable on-chain record of file integrity.

---

## Setup

### 1. Install Dependencies

```bash
cd scripts
npm install
```

This installs:
- `ethers@^6.9.0` - Ethereum library for contract interaction
- `dotenv@^16.3.1` - Environment variable management

### 2. Configure Environment

Ensure your `.env` file (in project root) contains:

```bash
# Required
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourDeployedContractAddress
PRIVATE_KEY=your_private_key_without_0x

# Optional (defaults to testnet)
MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz
MANTLE_CHAIN_ID=5003
```

### 3. Get Testnet MNT

Visit the Mantle testnet faucet:
https://faucet.testnet.mantle.xyz

---

## Usage

### Basic Command

```bash
node anchor.js <root> <metaURI>
```

**Arguments**:
- `<root>` - Merkle root hash (with or without `0x` prefix, 64 hex chars)
- `<metaURI>` - Metadata URI (IPFS, HTTP, or placeholder)

### Example 1: From Server Response

After uploading files to the server, you'll receive:

```json
{
  "success": true,
  "batch": {
    "local_batch_id": 1,
    "root": "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
    "file_count": 3,
    "suggested_meta_uri": "ipfs://placeholder-1",
    "registry_address": "0x..."
  }
}
```

Anchor it on Mantle:

```bash
cd scripts
node anchor.js \
  9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 \
  ipfs://placeholder-1
```

### Example 2: With 0x Prefix

```bash
node anchor.js \
  0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 \
  ipfs://QmTest123
```

---

## Output

### Successful Execution

```
🔗 MerkSeal Anchor Script
═══════════════════════════════════════════════════════════

📡 Connecting to Mantle...
   Network: Testnet (Chain ID: 5003)
   RPC: https://rpc.testnet.mantle.xyz
   Wallet: 0xYourAddress

💰 Balance: 1.5 MNT

📝 Contract Details:
   Address: 0xRegistryAddress

📦 Batch to Anchor:
   Root: 0x9f86d081...
   Meta URI: ipfs://placeholder-1

⛽ Estimating gas...
   Estimated gas: 85000

🚀 Sending transaction...
   Transaction hash: 0xabcdef123456...

⏳ Waiting for confirmation...
   ✅ Confirmed in block 12345678

═══════════════════════════════════════════════════════════
✅ SUCCESS! Batch anchored on Mantle
═══════════════════════════════════════════════════════════

📊 Results:
   Mantle Batch ID: 1
   Transaction Hash: 0xabcdef123456...
   Block Number: 12345678
   Gas Used: 82341

🔍 View on Explorer:
   https://explorer.testnet.mantle.xyz/tx/0xabcdef123456...

💾 Save this information:
   mantle_batch_id: 1
   tx_hash: 0xabcdef123456...
```

---

## Integration Workflow

### Complete Upload → Anchor Flow

```bash
# 1. Upload files to server
curl -X POST http://localhost:8080/upload \
  -F "file1=@document.pdf" \
  -F "file2=@image.jpg" \
  > batch_response.json

# 2. Extract root and metaURI from response
ROOT=$(cat batch_response.json | jq -r '.batch.root')
META_URI=$(cat batch_response.json | jq -r '.batch.suggested_meta_uri')

# 3. Anchor on Mantle
cd scripts
node anchor.js "$ROOT" "$META_URI"

# 4. Save the mantle_batch_id for later verification
```

### Programmatic Usage (Node.js)

```javascript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

async function anchorBatch(root, metaURI) {
  const { stdout } = await execAsync(
    `node anchor.js ${root} ${metaURI}`,
    { cwd: './scripts' }
  );
  
  console.log(stdout);
  
  // Parse output to extract mantle_batch_id
  const match = stdout.match(/Mantle Batch ID: (\d+)/);
  if (match) {
    return parseInt(match[1]);
  }
  
  throw new Error('Failed to parse batch ID');
}

// Usage
const mantleBatchId = await anchorBatch(
  '0x9f86d081...',
  'ipfs://QmTest123'
);
console.log('Anchored as batch', mantleBatchId);
```

---

## Error Handling

### Missing Environment Variables

```
❌ Error: MERKLE_BATCH_REGISTRY_ADDRESS not set in .env
```

**Solution**: Deploy the contract and add address to `.env`

### Insufficient Balance

```
❌ Error: Insufficient balance. Get testnet MNT from:
   https://faucet.testnet.mantle.xyz
```

**Solution**: Visit the faucet and request testnet MNT

### Invalid Root Hash

```
❌ Error: Invalid root hash length (expected 66 chars with 0x, got 64)
   Root: 9f86d081...
```

**Solution**: Root hash should be 64 hex characters (script auto-adds `0x` if missing)

### Transaction Failed

```
❌ Transaction failed: execution reverted
```

**Possible causes**:
- Root is zero (0x0000...)
- Contract address is incorrect
- Network mismatch (mainnet vs testnet)

---

## Files

| File | Description |
|------|-------------|
| `anchor.js` | Main anchor script |
| `MerkleBatchRegistry.abi.json` | Contract ABI |
| `package.json` | Node.js dependencies |

---

## Next Steps

After anchoring:

1. **Save the mapping** - Link `local_batch_id` → `mantle_batch_id`
2. **Update metadata** - Store `mantle_batch_id` in `batches/<id>/metadata.json`
3. **Verify later** - Use `mantle_batch_id` to verify files against on-chain root (Task 5)

---

## Advanced: Batch Anchoring Script

For anchoring multiple batches:

```bash
#!/bin/bash
# anchor_all.sh

for batch_dir in batches/*/; do
  METADATA="$batch_dir/metadata.json"
  if [ -f "$METADATA" ]; then
    ROOT=$(jq -r '.root' "$METADATA")
    META_URI=$(jq -r '.suggested_meta_uri' "$METADATA")
    
    echo "Anchoring batch from $batch_dir..."
    node scripts/anchor.js "$ROOT" "$META_URI"
    echo ""
  fi
done
```

---

**Task 4 Complete!** ✅
