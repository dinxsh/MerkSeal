# End-to-End Testing Guide

## 🎯 Overview

This guide explains how to run end-to-end tests for MerkSeal to validate the complete workflow.

---

## 📋 Prerequisites

### 1. Environment Setup

Create `.env` file from `.env.example`:
```bash
cp .env.example .env
```

Configure required variables:
```bash
# Required
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourDeployedContractAddress
PRIVATE_KEY=your_private_key_here

# Optional (defaults to testnet)
MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz
MANTLE_CHAIN_ID=5003
```

### 2. Deploy Contract (First Time Only)

```bash
cd contracts
forge create --rpc-url https://rpc.testnet.mantle.xyz \
  --private-key $PRIVATE_KEY \
  MerkleBatchRegistry

# Copy the deployed address to .env
```

### 3. Get Testnet MNT

Visit: https://faucet.testnet.mantle.xyz

---

## 🚀 Running E2E Tests

### Option 1: Automated Script (Linux/Mac)

```bash
chmod +x test-e2e.sh
./test-e2e.sh
```

### Option 2: Automated Script (Windows)

```powershell
.\test-e2e.ps1
```

### Option 3: Manual Step-by-Step

#### Step 1: Build Components
```bash
cargo build --release
cd scripts && npm install && cd ..
```

#### Step 2: Start Server
```bash
cargo run --release -p server &
```

#### Step 3: Upload Files
```bash
curl -X POST http://localhost:8080/upload \
  -F "file1=@test1.txt" \
  -F "file2=@test2.txt" \
  > response.json

# Extract root
ROOT=$(cat response.json | jq -r '.batch.root')
BATCH_ID=$(cat response.json | jq -r '.batch.local_batch_id')
```

#### Step 4: Anchor on Mantle
```bash
cd scripts
node anchor.js $ROOT "ipfs://test-metadata"
# Note the Mantle batch ID from output
cd ..
```

#### Step 5: Verify
```bash
cargo run --release -p client -- verify \
  --batch-id $BATCH_ID \
  --mantle-batch-id 1
```

---

## ✅ Expected Results

### Successful E2E Test

```
═══════════════════════════════════════════════════════════
MerkSeal End-to-End Test
═══════════════════════════════════════════════════════════

📋 Checking prerequisites...
✅ Environment configured

🔨 Step 1: Building all components...
✅ Rust components built
✅ Script dependencies installed

🚀 Step 2: Starting MerkSeal server...
✅ Server running on http://localhost:8080

📄 Step 3: Creating test files...
✅ Created 3 test files

📤 Step 4: Uploading files to server...
✅ Files uploaded
   Batch ID: 1
   Merkle Root: 0x9f86d081...

⚓ Step 5: Anchoring Merkle root on Mantle...
✅ Anchored on Mantle
   Mantle Batch ID: 1
   Transaction: 0xabcd...

🔍 Step 6: Verifying batch...
✅ Batch metadata exists
✅ Verification successful

🔐 Step 7: Testing tamper detection...
✅ Tamper detection ready

═══════════════════════════════════════════════════════════
E2E Test Summary
═══════════════════════════════════════════════════════════
✅ All tests passed!
```

---

## 🐛 Troubleshooting

### Server Won't Start

**Error**: `Address already in use`

**Solution**:
```bash
# Find and kill existing server
lsof -i :8080
kill -9 <PID>

# Or use different port
export SERVER_PORT=8081
cargo run --release -p server
```

### Upload Fails

**Error**: `Connection refused`

**Solution**:
- Check server is running: `curl http://localhost:8080/health`
- Check firewall settings
- Verify port 8080 is not blocked

### Anchor Fails

**Error**: `Insufficient funds`

**Solution**:
- Get testnet MNT from faucet
- Check balance: `cast balance $YOUR_ADDRESS --rpc-url https://rpc.testnet.mantle.xyz`

**Error**: `Invalid contract address`

**Solution**:
- Verify `MERKLE_BATCH_REGISTRY_ADDRESS` in `.env`
- Ensure contract is deployed on Mantle testnet
- Check address format (should start with 0x)

### Verification Fails

**Error**: `Batch not found`

**Solution**:
- Ensure anchor step completed successfully
- Check Mantle batch ID matches
- Verify contract address is correct

**Error**: `Root mismatch`

**Solution**:
- Files may have been modified
- Re-upload files
- Check batch metadata is intact

---

## 🧪 Test Scenarios

### Scenario 1: Happy Path
1. Upload 3 files
2. Anchor on Mantle
3. Verify successfully
4. ✅ Expected: All steps pass

### Scenario 2: Tamper Detection
1. Upload files
2. Anchor on Mantle
3. Modify one file locally
4. Verify
5. ✅ Expected: Verification fails with "Root mismatch"

### Scenario 3: Large Batch
1. Upload 100 files
2. Anchor on Mantle
3. Verify
4. ✅ Expected: Same gas cost as 3 files (constant!)

### Scenario 4: Multiple Batches
1. Upload batch 1
2. Anchor batch 1
3. Upload batch 2
4. Anchor batch 2
5. Verify both
6. ✅ Expected: Both verify independently

---

## 📊 Performance Benchmarks

### Expected Timings

| Operation | Time | Notes |
|-----------|------|-------|
| Build | 30-60s | First time only |
| Server start | 1-2s | Instant after build |
| Upload (3 files) | <100ms | Local operation |
| Anchor on Mantle | 2-3s | Network dependent |
| Verification | <500ms | RPC query + local compute |

### Gas Costs

| Operation | Gas | USD (Mantle) | USD (Ethereum) |
|-----------|-----|--------------|----------------|
| Register batch | ~82,000 | $0.0001 | $50 |
| Verify (view) | 0 | $0 | $0 |

---

## 🔄 Continuous Integration

### GitHub Actions (Example)

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Build
        run: cargo build --release
      
      - name: Run E2E Tests
        env:
          MERKLE_BATCH_REGISTRY_ADDRESS: ${{ secrets.REGISTRY_ADDRESS }}
          PRIVATE_KEY: ${{ secrets.PRIVATE_KEY }}
        run: ./test-e2e.sh
```

---

## 📝 Test Checklist

Before submitting/deploying:

- [ ] All Rust components compile
- [ ] Server starts without errors
- [ ] Upload endpoint accepts files
- [ ] Merkle root is computed correctly
- [ ] Batch metadata is saved
- [ ] Anchor script connects to Mantle
- [ ] Transaction is confirmed on-chain
- [ ] Client can query Mantle
- [ ] Verification compares roots correctly
- [ ] Tamper detection works
- [ ] Documentation is up to date

---

## 🎯 Success Criteria

A successful E2E test demonstrates:

1. ✅ **Upload**: Files are uploaded and Merkle root is computed
2. ✅ **Storage**: Batch metadata is saved locally
3. ✅ **Anchor**: Root is registered on Mantle L2
4. ✅ **Verification**: On-chain root matches local root
5. ✅ **Integrity**: Tampered files are detected

---

**Ready to test!** 🚀

Run `./test-e2e.sh` (Linux/Mac) or `.\test-e2e.ps1` (Windows) to validate your setup.
