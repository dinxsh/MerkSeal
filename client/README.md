# Client Usage Guide

## Overview

The MerkSeal client verifies file integrity by comparing local Merkle roots with on-chain roots stored on Mantle L2.

---

## Installation

```bash
cargo build --release -p client
```

The binary will be at `target/release/client` (or `client.exe` on Windows).

---

## Commands

### Show Configuration

```bash
cargo run -p client -- config
```

**Output**:
```
MerkSeal Client Configuration
═══════════════════════════════════════════════════════════
  RPC URL: https://rpc.testnet.mantle.xyz
  Chain ID: 5003
  Registry: 0xYourRegistryAddress
  Network: Testnet
  Explorer: https://explorer.testnet.mantle.xyz/address/0x...
```

---

### Verify Batch

Verify a batch against Mantle L2:

```bash
cargo run -p client -- verify --batch-id <LOCAL_BATCH_ID> [--mantle-batch-id <MANTLE_BATCH_ID>]
```

**Arguments**:
- `--batch-id` (required) - Local batch ID (from server upload)
- `--mantle-batch-id` (optional) - Mantle batch ID (if not in metadata.json)

---

## Verification Flow

### Prerequisites

1. **Files uploaded** - Batch exists in `batches/<batch_id>/`
2. **Batch anchored** - Root registered on Mantle via anchor script
3. **Metadata updated** - `metadata.json` contains `mantle_batch_id`

### Example Workflow

#### 1. Upload Files
```bash
curl -X POST http://localhost:8080/upload \
  -F "file1=@doc.pdf" \
  -F "file2=@image.jpg" \
  > response.json

# Extract local_batch_id
LOCAL_BATCH_ID=$(cat response.json | jq -r '.batch.local_batch_id')
ROOT=$(cat response.json | jq -r '.batch.root')
META_URI=$(cat response.json | jq -r '.batch.suggested_meta_uri')
```

#### 2. Anchor on Mantle
```bash
cd scripts
node anchor.js "$ROOT" "$META_URI"

# Save the mantle_batch_id from output
MANTLE_BATCH_ID=1
```

#### 3. Update Metadata (Optional but Recommended)
```bash
# Add mantle_batch_id to metadata.json
jq --arg id "$MANTLE_BATCH_ID" '.mantle_batch_id = ($id | tonumber)' \
  batches/$LOCAL_BATCH_ID/metadata.json > temp.json && \
  mv temp.json batches/$LOCAL_BATCH_ID/metadata.json
```

#### 4. Verify
```bash
cargo run -p client -- verify --batch-id $LOCAL_BATCH_ID
```

**Or with explicit Mantle batch ID**:
```bash
cargo run -p client -- verify \
  --batch-id $LOCAL_BATCH_ID \
  --mantle-batch-id $MANTLE_BATCH_ID
```

---

## Verification Output

### Successful Verification

```
🔍 MerkSeal Batch Verification
═══════════════════════════════════════════════════════════

📂 Loading local batch metadata...
   ✓ Local batch ID: 1
   ✓ Local root: 0x9f86d081...
   ✓ File count: 3

📝 Using Mantle batch ID from metadata: 1

🔗 Querying Mantle L2...
   Network: Testnet
   RPC: https://rpc.testnet.mantle.xyz
   ✓ On-chain root: 0x9f86d081...
   ✓ Owner: 0xYourAddress
   ✓ Meta URI: ipfs://placeholder-1
   ✓ Timestamp: 1705312345

🔐 Verifying Merkle root...
   ✅ ROOT MATCH!
   Local root:    0x9f86d081...
   On-chain root: 0x9f86d081...

📁 Verifying local files...
   ✓ doc.pdf: 12345 bytes
   ✓ image.jpg: 67890 bytes
   ✓ data.csv: 54321 bytes

   Computed root from files: 0x9f86d081...
   ✅ Local files match local root!

═══════════════════════════════════════════════════════════
✅ VERIFICATION SUCCESSFUL!
═══════════════════════════════════════════════════════════

Summary:
  ✓ Local files match local root
  ✓ Local root matches on-chain root
  ✓ Batch is verified and tamper-proof

Batch Details:
  Local Batch ID: 1
  Mantle Batch ID: 1
  File Count: 3
  Merkle Root: 0x9f86d081...

🔍 View on Mantle Explorer:
   https://explorer.testnet.mantle.xyz/address/0x...
```

### Failed Verification (Root Mismatch)

```
🔐 Verifying Merkle root...
   ❌ ROOT MISMATCH!
   Local root:    0x9f86d081...
   On-chain root: 0xabcdef12...

⚠️  WARNING: Roots do not match!
   This could indicate:
   - Wrong Mantle batch ID
   - Local files have been modified
   - Metadata corruption

❌ Verification failed: Root verification failed
```

### Failed Verification (Local Files Modified)

```
📁 Verifying local files...
   ✓ doc.pdf: 12345 bytes
   ✓ image.jpg: 99999 bytes  ← Modified!
   ✓ data.csv: 54321 bytes

   Computed root from files: 0xabcdef12...
   ❌ Local files DO NOT match local root!
   This indicates local files have been modified.

❌ Verification failed: Local file verification failed
```

---

## What Gets Verified?

The client performs **three levels of verification**:

1. **Local File Integrity**
   - Hashes all files in `batches/<batch_id>/`
   - Builds Merkle tree from hashes
   - Compares computed root with stored local root
   - **Detects**: Local file modifications

2. **On-Chain Root Retrieval**
   - Queries Mantle contract `getBatch(mantle_batch_id)`
   - Retrieves on-chain root, owner, metaURI, timestamp
   - **Proves**: Batch was anchored on Mantle

3. **Root Comparison**
   - Compares local root with on-chain root
   - **Proves**: Local files match what was anchored
   - **Guarantees**: Tamper-proof verification

---

## Troubleshooting

### "Batch not found"
```
❌ Verification failed: Batch 1 not found at batches/1/metadata.json
```

**Solution**: Ensure batch exists. Check `batches/` directory.

### "Mantle batch ID not found"
```
❌ Verification failed: Mantle batch ID not found. Provide with --mantle-batch-id or add to metadata.json
```

**Solution**: Either:
- Add `mantle_batch_id` to `metadata.json`
- Provide `--mantle-batch-id` argument

### "RPC error"
```
❌ Verification failed: error sending request for url (https://rpc.testnet.mantle.xyz/)
```

**Solution**: Check internet connection and RPC URL in `.env`

---

## Demo Script

Complete end-to-end demo:

```bash
#!/bin/bash
# demo.sh - Complete MerkSeal demo

echo "=== MerkSeal Demo ==="
echo ""

# 1. Start server
echo "1. Starting server..."
cargo run -p server &
SERVER_PID=$!
sleep 3

# 2. Upload files
echo "2. Uploading files..."
curl -X POST http://localhost:8080/upload \
  -F "file1=@test1.txt" \
  -F "file2=@test2.txt" \
  -F "file3=@test3.txt" \
  > response.json

cat response.json | jq '.'

# 3. Extract metadata
LOCAL_BATCH_ID=$(cat response.json | jq -r '.batch.local_batch_id')
ROOT=$(cat response.json | jq -r '.batch.root')
META_URI=$(cat response.json | jq -r '.batch.suggested_meta_uri')

echo ""
echo "3. Anchoring on Mantle..."
cd scripts
node anchor.js "$ROOT" "$META_URI" | tee anchor_output.txt
MANTLE_BATCH_ID=$(grep "Mantle Batch ID:" anchor_output.txt | awk '{print $4}')
cd ..

# 4. Update metadata
echo ""
echo "4. Updating metadata..."
jq --arg id "$MANTLE_BATCH_ID" '.mantle_batch_id = ($id | tonumber)' \
  batches/$LOCAL_BATCH_ID/metadata.json > temp.json && \
  mv temp.json batches/$LOCAL_BATCH_ID/metadata.json

# 5. Verify
echo ""
echo "5. Verifying batch..."
cargo run -p client -- verify --batch-id $LOCAL_BATCH_ID

# Cleanup
kill $SERVER_PID
echo ""
echo "=== Demo Complete ==="
```

---

**Task 5 Complete!** ✅
