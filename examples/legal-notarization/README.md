# Legal Document Notarization Example

## Overview

This example demonstrates how MerkSeal can be used for **legal document notarization** - a real-world RWA/RealFi use case.

### Use Case

A law firm needs to notarize a contract and provide cryptographic proof that:
1. The document existed at a specific time
2. The document hasn't been tampered with
3. The proof is verifiable by anyone, anywhere

### Why Mantle?

- **Low cost**: Notarization costs ~$0.0001 on Mantle vs ~$50 on Ethereum
- **Immutable**: Once anchored, the proof is permanent
- **Verifiable**: Anyone can verify using the Mantle explorer
- **Compliant**: Blockchain timestamps are legally recognized in many jurisdictions

---

## Step-by-Step Guide

### 1. Prepare Documents

```bash
cd examples/legal-notarization

# Place your legal documents here
# For this example, we'll use a sample contract
```

**Sample documents included**:
- `sample-contract.pdf` - Employment agreement
- `sample-nda.pdf` - Non-disclosure agreement

### 2. Notarize Documents

```bash
# Upload documents to MerkSeal server
curl -X POST http://localhost:8080/upload \
  -F "contract=@sample-contract.pdf" \
  -F "nda=@sample-nda.pdf" \
  > notarization-response.json

# Extract the Merkle root
ROOT=$(cat notarization-response.json | jq -r '.batch.root')
BATCH_ID=$(cat notarization-response.json | jq -r '.batch.local_batch_id')

echo "Documents uploaded. Merkle Root: $ROOT"
```

### 3. Anchor on Mantle Blockchain

```bash
# Anchor the Merkle root on Mantle
cd ../../scripts
node anchor.js "$ROOT" "Legal documents notarized on $(date -u +%Y-%m-%dT%H:%M:%SZ)"

# Save the transaction hash and Mantle batch ID
# Example output: Mantle Batch ID: 5, TX: 0xabcd...
```

### 4. Generate Certificate of Authenticity

```bash
cd ../examples/legal-notarization

# Run the certificate generator
node generate-certificate.js \
  --batch-id $BATCH_ID \
  --mantle-batch-id 5 \
  --tx-hash 0xabcd... \
  --documents "Employment Contract, NDA"

# Generates: certificate-of-authenticity.pdf
```

**Certificate includes**:
- Document names and file hashes
- Merkle root hash
- Mantle blockchain transaction hash
- Block number and timestamp
- QR code linking to Mantle explorer
- Legal disclaimer

### 5. Verify Authenticity (Anytime, Anywhere)

```bash
# Anyone can verify the documents
cd ../../
cargo run -p client -- verify --batch-id $BATCH_ID --mantle-batch-id 5
```

**Verification proves**:
- ✅ Documents match the Merkle root
- ✅ Merkle root is anchored on Mantle blockchain
- ✅ Documents existed at the recorded timestamp
- ✅ Documents haven't been tampered with

---

## Real-World Benefits

### For Law Firms
- **Cost savings**: $0.0001 vs traditional notary fees
- **Instant**: No waiting for notary appointments
- **Global**: Works across jurisdictions
- **Auditable**: Permanent blockchain record

### For Enterprises
- **Compliance**: Blockchain timestamps for regulatory requirements
- **Dispute resolution**: Cryptographic proof of document state
- **Supply chain**: Verify contracts with suppliers
- **IP protection**: Timestamp invention disclosures

### For Individuals
- **Wills & trusts**: Prove document existence
- **Real estate**: Timestamp purchase agreements
- **Employment**: Verify contract terms
- **Creative work**: Copyright protection

---

## Legal Considerations

### Admissibility

Blockchain timestamps are increasingly recognized in legal proceedings:
- **USA**: Blockchain records admissible under Federal Rules of Evidence 902(14)
- **EU**: eIDAS regulation recognizes electronic timestamps
- **China**: Supreme Court recognizes blockchain evidence (2018)

### Limitations

⚠️ **Important**: This is **not** a replacement for traditional notarization in all cases:
- Some jurisdictions require physical notary presence
- Real estate transactions may have specific requirements
- Consult legal counsel for your jurisdiction

### Best Practices

1. **Combine with traditional methods**: Use MerkSeal as additional proof
2. **Keep original files**: Blockchain only stores the hash, not the document
3. **Document the process**: Save transaction hashes and certificates
4. **Regular verification**: Periodically verify documents remain unchanged

---

## Technical Details

### What Gets Anchored

```
Documents → SHA-256 Hashes → Merkle Tree → Root Hash → Mantle Blockchain
```

**On-chain data** (32 bytes):
- Merkle root: `0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08`

**Off-chain data** (stored by you):
- Original documents
- Individual file hashes
- Batch metadata

### Verification Process

1. **Hash documents**: Recompute SHA-256 hashes
2. **Build Merkle tree**: Reconstruct tree from hashes
3. **Compare roots**: Local root vs on-chain root
4. **Check blockchain**: Query Mantle for timestamp and owner

### Gas Costs

| Operation | Mantle Testnet | Ethereum Mainnet |
|-----------|----------------|------------------|
| Notarize 2 documents | ~82,000 gas (~$0.0001) | ~82,000 gas (~$50) |
| Verify | Free (view function) | Free (view function) |

**Savings**: 99.9998% cheaper on Mantle! 🎉

---

## Example Output

### Certificate of Authenticity

```
═══════════════════════════════════════════════════════════
        CERTIFICATE OF AUTHENTICITY
        Powered by MerkSeal on Mantle L2
═══════════════════════════════════════════════════════════

Documents Notarized:
  1. sample-contract.pdf
     Hash: 0x5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03

  2. sample-nda.pdf
     Hash: 0x7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730

Merkle Root: 0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08

Blockchain Proof:
  Network: Mantle Testnet
  Transaction: 0xabcdef123456789...
  Block: 12,345,678
  Timestamp: 2026-01-15 11:15:23 UTC
  
Verify at: https://explorer.testnet.mantle.xyz/tx/0xabcdef...

[QR CODE]

This certificate proves that the above documents existed in their
current state at the recorded timestamp and have not been modified.

═══════════════════════════════════════════════════════════
```

---

## Next Steps

1. **Try it yourself**: Follow the steps above with your own documents
2. **Integrate**: Use the MerkSeal SDK to add notarization to your app
3. **Customize**: Modify the certificate template for your branding
4. **Deploy**: Use Mantle mainnet for production notarization

---

## Files in This Example

- `README.md` - This file
- `sample-contract.pdf` - Example employment contract
- `sample-nda.pdf` - Example NDA
- `generate-certificate.js` - Certificate generator script
- `certificate-template.html` - HTML template for certificates
- `verify-documents.sh` - Quick verification script

---

**Built with MerkSeal on Mantle L2** 🔐
