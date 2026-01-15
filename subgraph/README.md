# MerkSeal Subgraph - The Graph Integration

## Overview

This subgraph indexes all `BatchRegistered` events from the MerkleBatchRegistry contract on Mantle L2, making batch data easily queryable via GraphQL.

### Why The Graph?

- **Ecosystem Integration**: Shows MerkSeal is composable with Web3 infrastructure
- **Developer Experience**: Easy to query batch history without RPC calls
- **Scalability**: Efficient indexing of thousands of batches
- **Real-time**: Automatically updates as new batches are registered

---

## Schema

### Batch Entity
```graphql
type Batch @entity {
  id: ID! # Mantle batch ID
  root: Bytes! # Merkle root
  owner: Bytes! # Registrant address
  metaURI: String! # Metadata URI
  timestamp: BigInt! # Block timestamp
  blockNumber: BigInt!
  transactionHash: Bytes!
}
```

### BatchRegisteredEvent Entity
```graphql
type BatchRegisteredEvent @entity {
  id: ID! # TX hash + log index
  batchId: BigInt!
  root: Bytes!
  owner: Bytes!
  metaURI: String!
  timestamp: BigInt!
  blockNumber: BigInt!
  transactionHash: Bytes!
}
```

---

## Setup

### 1. Install Dependencies

```bash
cd subgraph
npm install
```

### 2. Update Contract Address

Edit `subgraph.yaml`:
```yaml
source:
  address: "YOUR_DEPLOYED_CONTRACT_ADDRESS"
  startBlock: YOUR_DEPLOYMENT_BLOCK
```

### 3. Copy Contract ABI

```bash
# Copy ABI from contracts/out/
mkdir -p abis
cp ../contracts/out/MerkleBatchRegistry.sol/MerkleBatchRegistry.json abis/
```

### 4. Generate Code

```bash
npm run codegen
```

### 5. Build

```bash
npm run build
```

---

## Deployment

### Option 1: The Graph Hosted Service (Recommended for Hackathon)

```bash
# 1. Create subgraph on The Graph Studio
# Visit: https://thegraph.com/studio/

# 2. Authenticate
graph auth --studio YOUR_DEPLOY_KEY

# 3. Deploy
graph deploy --studio MerkSeal-mantle
```

### Option 2: Local Graph Node (For Testing)

```bash
# 1. Start local Graph node
docker-compose up

# 2. Create subgraph
npm run create-local

# 3. Deploy
npm run deploy-local
```

---

## Example Queries

### Get All Batches

```graphql
{
  batches(first: 10, orderBy: timestamp, orderDirection: desc) {
    id
    root
    owner
    metaURI
    timestamp
    blockNumber
  }
}
```

### Get Batches by Owner

```graphql
{
  batches(where: { owner: "0xYourAddress" }) {
    id
    root
    metaURI
    timestamp
  }
}
```

### Get Batch by ID

```graphql
{
  batch(id: "1") {
    id
    root
    owner
    metaURI
    timestamp
    transactionHash
  }
}
```

### Get Recent Events

```graphql
{
  batchRegisteredEvents(
    first: 5
    orderBy: timestamp
    orderDirection: desc
  ) {
    id
    batchId
    root
    owner
    timestamp
  }
}
```

### Search by Merkle Root

```graphql
{
  batches(where: { root: "0x9f86d081..." }) {
    id
    owner
    metaURI
    timestamp
  }
}
```

---

## Integration with Client

### Using GraphQL Client

```typescript
import { ApolloClient, InMemoryCache, gql } from '@apollo/client';

const client = new ApolloClient({
  uri: 'https://api.thegraph.com/subgraphs/name/MerkSeal/mantle',
  cache: new InMemoryCache()
});

// Query all batches
const { data } = await client.query({
  query: gql`
    {
      batches(first: 10) {
        id
        root
        owner
        timestamp
      }
    }
  `
});

console.log('Batches:', data.batches);
```

### Using fetch

```javascript
const query = `
  {
    batch(id: "1") {
      root
      owner
      metaURI
      timestamp
    }
  }
`;

const response = await fetch(
  'https://api.thegraph.com/subgraphs/name/MerkSeal/mantle',
  {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query })
  }
);

const { data } = await response.json();
console.log('Batch:', data.batch);
```

---

## Benefits

### For Developers
- **No RPC calls**: Query batch history without blockchain RPC
- **Fast queries**: Indexed data returns in milliseconds
- **Flexible**: GraphQL allows custom queries
- **Real-time**: Auto-updates as new batches are registered

### For Users
- **Batch explorer**: Build UIs to browse all batches
- **Search**: Find batches by owner, root, or timestamp
- **Analytics**: Track usage over time
- **Verification**: Cross-reference local data with indexed data

### For Ecosystem
- **Composability**: Other dApps can query MerkSeal data
- **Transparency**: All batch registrations are publicly queryable
- **Infrastructure**: Shows MerkSeal as ecosystem building block

---

## Troubleshooting

### "Failed to deploy"
- Check contract address in `subgraph.yaml`
- Ensure ABI file exists in `abis/`
- Verify network name is correct

### "No data indexed"
- Check `startBlock` in `subgraph.yaml`
- Ensure contract has emitted events
- Wait a few minutes for indexing

### "Query failed"
- Verify subgraph is deployed and synced
- Check GraphQL endpoint URL
- Test query in GraphiQL playground

---

## Next Steps

1. **Deploy subgraph** to The Graph Studio
2. **Update client** to query subgraph instead of RPC
3. **Build UI** to display batch history
4. **Add analytics** (batches per day, top owners, etc.)

---

**Ecosystem Integration Complete!** 🎉

This subgraph makes MerkSeal data accessible to the entire Mantle ecosystem.
