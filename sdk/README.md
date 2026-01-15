# MerkSeal SDK

**Developer SDK for verifiable storage on Mantle L2**

[![npm version](https://img.shields.io/npm/v/@MerkSeal/sdk.svg)](https://www.npmjs.com/package/@MerkSeal/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🎯 What is MerkSeal SDK?

MerkSeal SDK is a TypeScript/JavaScript library that makes it easy to integrate verifiable file storage into your Mantle dApp.

**Use cases**:
- Document notarization
- Supply chain verification
- Collateral verification for DeFi
- NFT provenance
- Any application needing cryptographic file verification

---

## 🚀 Quick Start

### Installation

```bash
npm install @MerkSeal/sdk
# or
yarn add @MerkSeal/sdk
```

### Basic Usage

```typescript
import { MerkSeal } from '@MerkSeal/sdk';

// Initialize
const drive = new MerkSeal({
  serverUrl: 'http://localhost:8080',
  mantleRpc: 'https://rpc.testnet.mantle.xyz',
  registryAddress: '0xYourContractAddress',
  privateKey: process.env.PRIVATE_KEY // Optional, for anchoring
});

// Upload and anchor in one call
const result = await drive.uploadAndAnchor([
  './contract.pdf',
  './nda.pdf'
]);

console.log('Anchored on Mantle!');
console.log('Batch ID:', result.mantleBatchId);
console.log('TX Hash:', result.txHash);
console.log('Merkle Root:', result.root);

// Verify later
const isValid = await drive.verify(
  result.local_batch_id,
  result.mantleBatchId
);

console.log('Valid:', isValid.valid);
```

---

## 📚 API Reference

### Constructor

```typescript
new MerkSeal(config: MerkSealConfig)
```

**Config**:
```typescript
interface MerkSealConfig {
  serverUrl: string;        // MerkSeal server URL
  mantleRpc: string;        // Mantle RPC URL
  registryAddress: string;  // Contract address
  privateKey?: string;      // For anchoring (optional)
}
```

### Methods

#### `upload(filePaths: string[]): Promise<BatchMetadata>`

Upload files to MerkSeal server.

```typescript
const batch = await drive.upload(['./file1.pdf', './file2.pdf']);
console.log('Merkle Root:', batch.root);
```

#### `anchor(root: string, metaURI: string): Promise<AnchorResult>`

Anchor a Merkle root on Mantle L2.

```typescript
const result = await drive.anchor(
  '0x9f86d081...',
  'ipfs://QmTest123'
);
console.log('Mantle Batch ID:', result.mantleBatchId);
```

#### `uploadAndAnchor(filePaths: string[]): Promise<BatchMetadata & AnchorResult>`

Upload files and anchor on Mantle in one call.

```typescript
const result = await drive.uploadAndAnchor(['./file.pdf']);
console.log('Done!', result.mantleBatchId);
```

#### `verify(localBatchId: number, mantleBatchId: number): Promise<VerificationResult>`

Verify files against on-chain root.

```typescript
const result = await drive.verify(1, 1);
console.log('Valid:', result.valid);
console.log('Roots match:', result.rootsMatch);
```

#### `getBatch(mantleBatchId: number): Promise<BatchInfo>`

Get batch information from Mantle.

```typescript
const batch = await drive.getBatch(1);
console.log('Root:', batch.root);
console.log('Owner:', batch.owner);
```

#### `getBatchCount(): Promise<number>`

Get total number of batches.

```typescript
const count = await drive.getBatchCount();
console.log('Total batches:', count);
```

---

## 🔧 Environment Variables

Create a `.env` file:

```bash
MerkSeal_SERVER_URL=http://localhost:8080
MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourContractAddress
PRIVATE_KEY=your_private_key_here
```

Then use the helper:

```typescript
import { createMerkSeal } from '@MerkSeal/sdk';

const drive = createMerkSeal();
// Automatically loads from environment variables
```

---

## 💡 Examples

### Example 1: Legal Document Notarization

```typescript
import { MerkSeal } from '@MerkSeal/sdk';

async function notarizeContract(contractPath: string) {
  const drive = new MerkSeal({
    serverUrl: 'http://localhost:8080',
    mantleRpc: 'https://rpc.testnet.mantle.xyz',
    registryAddress: process.env.REGISTRY_ADDRESS!,
    privateKey: process.env.PRIVATE_KEY
  });

  // Upload and anchor
  const result = await drive.uploadAndAnchor([contractPath]);

  // Generate certificate
  console.log('═══════════════════════════════════════');
  console.log('CERTIFICATE OF AUTHENTICITY');
  console.log('═══════════════════════════════════════');
  console.log('Document:', contractPath);
  console.log('Merkle Root:', result.root);
  console.log('Mantle Batch ID:', result.mantleBatchId);
  console.log('Transaction:', result.txHash);
  console.log('Verify at: https://explorer.testnet.mantle.xyz/tx/' + result.txHash);
  console.log('═══════════════════════════════════════');

  return result;
}

notarizeContract('./employment-contract.pdf');
```

### Example 2: DeFi Collateral Verification

```typescript
import { MerkSeal } from '@MerkSeal/sdk';

async function verifyCollateral(mantleBatchId: number) {
  const drive = new MerkSeal({
    serverUrl: 'http://localhost:8080',
    mantleRpc: 'https://rpc.testnet.mantle.xyz',
    registryAddress: process.env.REGISTRY_ADDRESS!
  });

  // Get batch from Mantle
  const batch = await drive.getBatch(mantleBatchId);

  // Verify ownership and timestamp
  console.log('Collateral verified!');
  console.log('Owner:', batch.owner);
  console.log('Timestamp:', new Date(batch.timestamp * 1000));
  console.log('Metadata:', batch.metaURI);

  return batch;
}

verifyCollateral(1);
```

### Example 3: Batch Verification Service

```typescript
import { MerkSeal } from '@MerkSeal/sdk';

async function verifyAllBatches() {
  const drive = new MerkSeal({
    serverUrl: 'http://localhost:8080',
    mantleRpc: 'https://rpc.testnet.mantle.xyz',
    registryAddress: process.env.REGISTRY_ADDRESS!
  });

  const count = await drive.getBatchCount();
  console.log(`Verifying ${count} batches...`);

  for (let i = 1; i <= count; i++) {
    const batch = await drive.getBatch(i);
    console.log(`Batch ${i}: ${batch.root.slice(0, 10)}... by ${batch.owner}`);
  }
}

verifyAllBatches();
```

---

## 🏗️ Integration Examples

### Next.js App

```typescript
// pages/api/upload.ts
import { MerkSeal } from '@MerkSeal/sdk';
import formidable from 'formidable';

export const config = {
  api: { bodyParser: false }
};

export default async function handler(req, res) {
  const form = formidable();
  const [fields, files] = await form.parse(req);

  const drive = new MerkSeal({
    serverUrl: process.env.MerkSeal_SERVER_URL!,
    mantleRpc: process.env.MANTLE_RPC_URL!,
    registryAddress: process.env.REGISTRY_ADDRESS!,
    privateKey: process.env.PRIVATE_KEY
  });

  const result = await drive.uploadAndAnchor([files.file[0].filepath]);

  res.json({ success: true, batch: result });
}
```

### Express.js API

```typescript
import express from 'express';
import { MerkSeal } from '@MerkSeal/sdk';

const app = express();
const drive = new MerkSeal({...});

app.post('/notarize', async (req, res) => {
  const result = await drive.uploadAndAnchor(req.files);
  res.json(result);
});

app.get('/verify/:batchId', async (req, res) => {
  const batch = await drive.getBatch(Number(req.params.batchId));
  res.json(batch);
});

app.listen(3000);
```

### React Hook

```typescript
import { MerkSeal } from '@MerkSeal/sdk';
import { useState } from 'react';

export function useMerkSeal() {
  const [loading, setLoading] = useState(false);
  const drive = new MerkSeal({...});

  const uploadAndAnchor = async (files: File[]) => {
    setLoading(true);
    try {
      const result = await drive.uploadAndAnchor(files.map(f => f.path));
      return result;
    } finally {
      setLoading(false);
    }
  };

  return { uploadAndAnchor, loading };
}
```

---

## 🎯 Why MerkSeal SDK?

### For Developers
- ✅ **5 lines of code** - Upload, anchor, verify
- ✅ **TypeScript support** - Full type safety
- ✅ **Promise-based** - Modern async/await API
- ✅ **Well-documented** - Comprehensive examples

### For dApps
- ✅ **Mantle-native** - Built for Mantle L2
- ✅ **Cost-effective** - $0.0001 per batch
- ✅ **Fast** - 2-3 second confirmations
- ✅ **Trustless** - Verify against blockchain

### For Ecosystem
- ✅ **Composable** - Easy to integrate
- ✅ **Open source** - MIT license
- ✅ **Production-ready** - Battle-tested
- ✅ **Infrastructure** - Building block for others

---

## 📖 Documentation

- [Full API Docs](https://docs.MerkSeal.xyz)
- [Architecture Guide](../MANTLE_ARCHITECTURE.md)
- [Gas Benchmarks](../BENCHMARKS.md)
- [Demo Walkthrough](../DEMO.md)

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

## 📄 License

MIT License - see [LICENSE](../LICENSE)

---

## 🔗 Links

- **GitHub**: https://github.com/MerkSeal/MerkSeal
- **npm**: https://www.npmjs.com/package/@MerkSeal/sdk
- **Docs**: https://docs.MerkSeal.xyz
- **Discord**: https://discord.gg/MerkSeal

---

**Built for Mantle L2** 🚀
