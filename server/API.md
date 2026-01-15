# Server Upload API Documentation

## POST /upload

Upload files to the server, compute Merkle root, and receive batch metadata for anchoring on Mantle.

### Request

**Content-Type**: `multipart/form-data`

**Body**: One or more files

### Response

**Status**: `200 OK`

**Content-Type**: `application/json`

```json
{
  "success": true,
  "batch": {
    "local_batch_id": 1,
    "root": "a3f5b8c9d2e1f0a7b6c5d4e3f2a1b0c9d8e7f6a5b4c3d2e1f0a9b8c7d6e5f4a3",
    "file_count": 3,
    "suggested_meta_uri": "ipfs://placeholder-1",
    "registry_address": "0xYourRegistryContractAddress"
  }
}
```

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `success` | boolean | Always `true` on success |
| `batch.local_batch_id` | number | Incremental batch ID (starts at 1) |
| `batch.root` | string | Merkle root hash (hex, 64 chars) |
| `batch.file_count` | number | Number of files in this batch |
| `batch.suggested_meta_uri` | string | Placeholder URI for batch metadata |
| `batch.registry_address` | string | Mantle contract address for anchoring |

### Error Response

**Status**: `400 Bad Request` or `500 Internal Server Error`

```json
{
  "success": false,
  "error": "Error description"
}
```

---

## GET /health

Health check endpoint.

### Response

```json
{
  "status": "ok",
  "service": "MerkSeal Server"
}
```

---

## Storage Structure

Files are stored in the following structure:

```
batches/
├── 1/
│   ├── file1.txt
│   ├── file2.jpg
│   └── metadata.json
├── 2/
│   ├── document.pdf
│   └── metadata.json
└── ...
```

Each `metadata.json` contains the `BatchMetadata` for that batch.

---

## Example Usage

### Using curl

```bash
# Upload files
curl -X POST http://localhost:8080/upload \
  -F "file1=@document.pdf" \
  -F "file2=@image.jpg" \
  -F "file3=@data.csv"

# Response:
# {
#   "success": true,
#   "batch": {
#     "local_batch_id": 1,
#     "root": "a3f5b8c9...",
#     "file_count": 3,
#     "suggested_meta_uri": "ipfs://placeholder-1",
#     "registry_address": "0x..."
#   }
# }
```

### Using JavaScript (fetch)

```javascript
const formData = new FormData();
formData.append('file1', file1);
formData.append('file2', file2);

const response = await fetch('http://localhost:8080/upload', {
  method: 'POST',
  body: formData
});

const data = await response.json();
console.log('Batch ID:', data.batch.local_batch_id);
console.log('Merkle Root:', data.batch.root);
console.log('Registry Address:', data.batch.registry_address);
```

---

## Next Steps

After receiving the batch metadata:

1. **Save the response** - Store `local_batch_id` and `root` locally
2. **Anchor on Mantle** - Use the `root` and `registry_address` to call `registerBatch()` on Mantle (see Task 4)
3. **Store Mantle batch ID** - Link `local_batch_id` to the on-chain `mantle_batch_id` returned from the contract

---

## Configuration

The server requires the following environment variables:

```bash
MERKLE_BATCH_REGISTRY_ADDRESS=0xYourContractAddress
MANTLE_RPC_URL=https://rpc.testnet.mantle.xyz  # Optional, defaults to testnet
MANTLE_CHAIN_ID=5003  # Optional, defaults to 5003
```

See `.env.example` for details.
