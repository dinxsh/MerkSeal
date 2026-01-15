import { MerkSeal } from '../src/index';
import * as dotenv from 'dotenv';

dotenv.config();

/**
 * Example 1: Upload and Anchor
 * 
 * This example shows how to upload files and anchor them on Mantle in one call.
 */
async function example1_uploadAndAnchor() {
    console.log('Example 1: Upload and Anchor\n');

    const drive = new MerkSeal({
        serverUrl: process.env.MerkSeal_SERVER_URL || 'http://localhost:8080',
        mantleRpc: process.env.MANTLE_RPC_URL || 'https://rpc.testnet.mantle.xyz',
        registryAddress: process.env.MERKLE_BATCH_REGISTRY_ADDRESS!,
        privateKey: process.env.PRIVATE_KEY
    });

    try {
        // Upload and anchor in one call
        const result = await drive.uploadAndAnchor([
            './examples/sample-file1.txt',
            './examples/sample-file2.txt'
        ]);

        console.log('✅ Success!');
        console.log('Local Batch ID:', result.local_batch_id);
        console.log('Mantle Batch ID:', result.mantleBatchId);
        console.log('Merkle Root:', result.root);
        console.log('Transaction Hash:', result.txHash);
        console.log('Gas Used:', result.gasUsed);
        console.log('\nView on Explorer:');
        console.log(`https://explorer.testnet.mantle.xyz/tx/${result.txHash}`);
    } catch (error) {
        console.error('❌ Error:', error);
    }
}

/**
 * Example 2: Separate Upload and Anchor
 * 
 * This example shows how to upload and anchor separately.
 */
async function example2_separateSteps() {
    console.log('\nExample 2: Separate Upload and Anchor\n');

    const drive = new MerkSeal({
        serverUrl: 'http://localhost:8080',
        mantleRpc: 'https://rpc.testnet.mantle.xyz',
        registryAddress: process.env.MERKLE_BATCH_REGISTRY_ADDRESS!,
        privateKey: process.env.PRIVATE_KEY
    });

    try {
        // Step 1: Upload
        console.log('Uploading files...');
        const batch = await drive.upload(['./examples/sample-file1.txt']);
        console.log('✅ Uploaded! Merkle Root:', batch.root);

        // Step 2: Anchor
        console.log('\nAnchoring on Mantle...');
        const anchorResult = await drive.anchor(batch.root, batch.suggested_meta_uri);
        console.log('✅ Anchored! Batch ID:', anchorResult.mantleBatchId);
    } catch (error) {
        console.error('❌ Error:', error);
    }
}

/**
 * Example 3: Verification
 * 
 * This example shows how to verify a batch against Mantle.
 */
async function example3_verification() {
    console.log('\nExample 3: Verification\n');

    const drive = new MerkSeal({
        serverUrl: 'http://localhost:8080',
        mantleRpc: 'https://rpc.testnet.mantle.xyz',
        registryAddress: process.env.MERKLE_BATCH_REGISTRY_ADDRESS!
    });

    try {
        // Verify batch
        const result = await drive.verify(1, 1); // localBatchId, mantleBatchId

        console.log('Verification Result:');
        console.log('Valid:', result.valid ? '✅' : '❌');
        console.log('Local Root:', result.localRoot);
        console.log('On-chain Root:', result.onchainRoot);
        console.log('Roots Match:', result.rootsMatch ? '✅' : '❌');
    } catch (error) {
        console.error('❌ Error:', error);
    }
}

/**
 * Example 4: Get Batch Info
 * 
 * This example shows how to query batch information from Mantle.
 */
async function example4_getBatch() {
    console.log('\nExample 4: Get Batch Info\n');

    const drive = new MerkSeal({
        serverUrl: 'http://localhost:8080',
        mantleRpc: 'https://rpc.testnet.mantle.xyz',
        registryAddress: process.env.MERKLE_BATCH_REGISTRY_ADDRESS!
    });

    try {
        // Get batch count
        const count = await drive.getBatchCount();
        console.log('Total Batches:', count);

        // Get specific batch
        if (count > 0) {
            const batch = await drive.getBatch(1);
            console.log('\nBatch 1:');
            console.log('Root:', batch.root);
            console.log('Owner:', batch.owner);
            console.log('Meta URI:', batch.metaURI);
            console.log('Timestamp:', new Date(batch.timestamp * 1000).toISOString());
        }
    } catch (error) {
        console.error('❌ Error:', error);
    }
}

// Run examples
async function main() {
    console.log('═══════════════════════════════════════');
    console.log('MerkSeal SDK Examples');
    console.log('═══════════════════════════════════════\n');

    // Uncomment the example you want to run:

    // await example1_uploadAndAnchor();
    // await example2_separateSteps();
    // await example3_verification();
    await example4_getBatch();

    console.log('\n═══════════════════════════════════════');
}

main().catch(console.error);
