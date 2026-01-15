# MerkSeal Demo - Mantle Hackathon

**A verifiable off-chain storage layer with on-chain Merkle root anchoring on Mantle L2**

---

## 🎯 What is MerkSeal?

MerkSeal is a **Mantle-native infrastructure tool** that provides cryptographically-verifiable file storage. Files are stored off-chain, but their Merkle roots are anchored on Mantle L2, creating an immutable audit trail.

**Key Innovation**: Combines off-chain storage efficiency with on-chain verification guarantees.

---

## 🏆 Hackathon Submission

**Track**: Infrastructure & Tooling  
**Secondary Narrative**: RWA/RealFi (document verification)

**Why MerkSeal wins**:
- ✅ **Mantle-native**: Built specifically for Mantle's low-fee L2 environment
- ✅ **Production-ready**: Minimal, auditable smart contract (<100 lines)
- ✅ **Gas-efficient**: ~$0.0001 per batch on Mantle testnet
- ✅ **Real use case**: Verifiable storage for legal docs, supply chain, medical records

---

## 🚀 Complete Demo Walkthrough

### Prerequisites

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install Foundry (for contract deployment)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# 3. Install Node.js (for anchor script)
# Download from https://nodejs.org

# 4. Get testnet MNT
# Visit: https://faucet.testnet.mantle.xyz
```

---

### Step 1: Deploy Contract on Mantle Testnet

```bash
cd contracts

# Set environment variables
export MANTLE_RPC_URL="https://rpc.testnet.mantle.xyz"
export PRIVATE_KEY="your_private_key_here"

# Deploy MerkleBatchRegistry
forge create --rpc-url $MANTLE_RPC_URL \
  --private-key $PRIVATE_KEY \
  MerkleBatchRegistry

# Save the deployed address!
# Example output: Deployed to: 0x1234567890abcdef...
```

**Add to `.env`**:
```bash
echo "MERKLE_BATCH_REGISTRY_ADDRESS=0xYourDeployedAddress" >> ../.env
echo "MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz" >> ../.env
echo "MANTLE_CHAIN_ID=5003" >> ../.env
echo "PRIVATE_KEY=your_private_key" >> ../.env
```

**✅ Checkpoint**: Contract deployed on Mantle testnet

---

### Step 2: Start MerkSeal Server

```bash
cd ..  # Back to project root

# Build and run server
cargo run -p server
```

**Expected output**:
```
MerkSeal Server
✓ Mantle config loaded:
  RPC URL: https://rpc.testnet.mantle.xyz
  Chain ID: 5003
  Registry: 0x1234567890abcdef...
  Network: Testnet

🚀 Server starting on http://127.0.0.1:8080
   POST /upload - Upload files and get batch metadata
   GET  /health - Health check

Ready to accept uploads!
```

**✅ Checkpoint**: Server running on localhost:8080

---

### Step 3: Upload Files

Open a new terminal:

```bash
# Create test files
echo "Document 1 content" > test1.txt
echo "Document 2 content" > test2.txt
echo "Document 3 content" > test3.txt

# Upload to server
curl -X POST http://localhost:8080/upload \
  -F "file1=@test1.txt" \
  -F "file2=@test2.txt" \
  -F "file3=@test3.txt" \
  | jq '.'
```

**Response**:
```json
{
  "success": true,
  "batch": {
    "local_batch_id": 1,
    "root": "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
    "file_count": 3,
    "suggested_meta_uri": "ipfs://placeholder-1",
    "registry_address": "0x1234567890abcdef..."
  }
}
```

**Save the root**:
```bash
ROOT=$(curl -X POST http://localhost:8080/upload \
  -F "file1=@test1.txt" \
  -F "file2=@test2.txt" \
  -F "file3=@test3.txt" | jq -r '.batch.root')

echo "Merkle Root: $ROOT"
```

**✅ Checkpoint**: Files uploaded, Merkle root computed

---

### Step 4: Anchor Root on Mantle

```bash
cd scripts
npm install

# Anchor the Merkle root on Mantle
node anchor.js $ROOT ipfs://placeholder-1
```

**Expected output**:
```
🔗 MerkSeal Anchor Script
═══════════════════════════════════════════════════════════

📡 Connecting to Mantle...
   Network: Testnet (Chain ID: 5003)
   Wallet: 0xYourAddress

💰 Balance: 1.5 MNT

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

**Save the Mantle batch ID**:
```bash
MANTLE_BATCH_ID=1  # From output above
```

**✅ Checkpoint**: Merkle root anchored on Mantle L2

---

### Step 5: Verify Files Against Mantle

```bash
cd ..  # Back to project root

# Verify batch against on-chain root
cargo run -p client -- verify --batch-id 1 --mantle-batch-id $MANTLE_BATCH_ID
```

**Expected output**:
```
🔍 MerkSeal Batch Verification
═══════════════════════════════════════════════════════════

📂 Loading local batch metadata...
   ✓ Local batch ID: 1
   ✓ Local root: 0x9f86d081...
   ✓ File count: 3

📝 Using provided Mantle batch ID: 1

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
   ✓ test1.txt: 19 bytes
   ✓ test2.txt: 19 bytes
   ✓ test3.txt: 19 bytes

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

**✅ Checkpoint**: Files verified against on-chain root!

---

### Step 6: Demonstrate Tamper Detection

```bash
# Modify a local file
echo "TAMPERED CONTENT" > batches/1/test1.txt

# Try to verify again
cargo run -p client -- verify --batch-id 1 --mantle-batch-id $MANTLE_BATCH_ID
```

**Expected output**:
```
📁 Verifying local files...
   ✓ test1.txt: 17 bytes  ← Modified!
   ✓ test2.txt: 19 bytes
   ✓ test3.txt: 19 bytes

   Computed root from files: 0xabcdef12...
   ❌ Local files DO NOT match local root!
   This indicates local files have been modified.

❌ Verification failed: Local file verification failed
```

**✅ Checkpoint**: Tamper detection works!

---

## 🎬 Talking Points for Judges

### Infrastructure & Tooling Track

**Problem**: How do you verify file integrity in distributed storage without downloading everything?

**Solution**: MerkSeal anchors Merkle roots on Mantle L2, enabling O(1) verification with on-chain guarantees.

**Why Mantle?**
- **Low fees**: ~$0.0001 per batch makes on-chain anchoring practical
- **EVM compatibility**: Easy integration with existing tools
- **Immutability**: On-chain roots provide tamper-proof audit trails

**Technical Highlights**:
- Minimal smart contract (<100 lines, easy to audit)
- Gas-efficient design (single `registerBatch` call per batch)
- Event-driven architecture (easy to index with The Graph)
- Three-level verification (local files → local root → on-chain root)

### RWA/RealFi Narrative

**Use Cases**:
- **Legal documents**: Notarize contracts with on-chain timestamps
- **Supply chain**: Verify product authenticity with document trails
- **Medical records**: HIPAA-compliant storage with cryptographic verification
- **Financial audits**: Immutable audit trails for compliance

**Value Proposition**:
- Off-chain storage = privacy + efficiency
- On-chain anchoring = immutability + verifiability
- Mantle L2 = low cost + high throughput

### Demo Flow Summary

1. **Deploy** contract on Mantle testnet (1 command)
2. **Upload** files to server → get Merkle root
3. **Anchor** root on Mantle → get on-chain batch ID
4. **Verify** files against on-chain root → cryptographic proof
5. **Tamper** a file → detection works!

**Total time**: ~5 minutes  
**Total cost**: ~$0.0001 (Mantle testnet)

---

## 📊 Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    MerkSeal System                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐    ┌──────────┐    ┌──────────────────┐ │
│  │  Client  │───▶│  Server  │───▶│  Anchor Script   │ │
│  │  (Rust)  │    │  (Rust)  │    │  (TypeScript)    │ │
│  └──────────┘    └──────────┘    └─────────┬────────┘ │
│       │               │                     │          │
│       │               │                     ▼          │
│       │               │          ┌──────────────────┐  │
│       │               │          │   Mantle L2      │  │
│       │               │          │  MerkleBatch     │  │
│       │               │          │  Registry.sol    │  │
│       │               │          └─────────┬────────┘  │
│       │               │                    │           │
│       │               ▼                    │           │
│       │      ┌────────────────┐            │           │
│       │      │  batches/1/    │            │           │
│       │      │  - file1.txt   │            │           │
│       │      │  - file2.txt   │            │           │
│       │      │  - metadata    │            │           │
│       │      └────────────────┘            │           │
│       │                                    │           │
│       └────────────────────────────────────┘           │
│              Verification Flow                         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 🏅 Why MerkSeal Wins

1. **Mantle-Native**: Built specifically for Mantle's low-fee environment
2. **Production-Ready**: Minimal, auditable code with comprehensive tests
3. **Real Use Case**: Solves actual problems in RWA/RealFi space
4. **Complete Implementation**: Full end-to-end demo working on testnet
5. **Infrastructure Focus**: Provides foundational layer for other dApps

---

## 📝 Next Steps (Post-Hackathon)

- [ ] Add Merkle proof generation for individual file verification
- [ ] Implement batch deletion/updates
- [ ] Add IPFS integration for metadata storage
- [ ] Create web UI for non-technical users
- [ ] Deploy to Mantle mainnet
- [ ] Integrate with The Graph for indexing
- [ ] Add support for encrypted files
- [ ] Build SDK for easy integration

---

**Built with ❤️ for Mantle Hackathon 2026**
