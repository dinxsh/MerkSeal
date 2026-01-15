#!/usr/bin/env node

/**
 * Anchor Script - Register Merkle Batch on Mantle L2
 * 
 * Usage:
 *   node anchor.js <root> <metaURI>
 * 
 * Example:
 *   node anchor.js 0x9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08 ipfs://QmTest123
 */

import { ethers } from 'ethers';
import { readFileSync } from 'fs';
import { config } from 'dotenv';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Load environment variables
config({ path: join(dirname(fileURLToPath(import.meta.url)), '../.env') });

// Load contract ABI
const __dirname = dirname(fileURLToPath(import.meta.url));
const abi = JSON.parse(readFileSync(join(__dirname, 'MerkleBatchRegistry.abi.json'), 'utf8'));

// Configuration
const RPC_URL = process.env.MANTLE_RPC_URL || 'https://rpc.testnet.mantle.xyz';
const CHAIN_ID = parseInt(process.env.MANTLE_CHAIN_ID || '5003');
const REGISTRY_ADDRESS = process.env.MERKLE_BATCH_REGISTRY_ADDRESS;
const PRIVATE_KEY = process.env.PRIVATE_KEY;

// Validate configuration
if (!REGISTRY_ADDRESS) {
    console.error('❌ Error: MERKLE_BATCH_REGISTRY_ADDRESS not set in .env');
    process.exit(1);
}

if (!PRIVATE_KEY) {
    console.error('❌ Error: PRIVATE_KEY not set in .env');
    process.exit(1);
}

// Parse command line arguments
const args = process.argv.slice(2);
if (args.length < 2) {
    console.error('Usage: node anchor.js <root> <metaURI>');
    console.error('');
    console.error('Example:');
    console.error('  node anchor.js 0x9f86d081... ipfs://QmTest123');
    process.exit(1);
}

const [rootArg, metaURI] = args;

// Ensure root is properly formatted (0x prefix + 64 hex chars)
let root = rootArg;
if (!root.startsWith('0x')) {
    root = '0x' + root;
}

if (root.length !== 66) {
    console.error(`❌ Error: Invalid root hash length (expected 66 chars with 0x, got ${root.length})`);
    console.error(`   Root: ${root}`);
    process.exit(1);
}

async function anchorBatch() {
    console.log('🔗 MerkSeal Anchor Script');
    console.log('═══════════════════════════════════════════════════════════');
    console.log('');

    // Setup provider and wallet
    console.log('📡 Connecting to Mantle...');
    const provider = new ethers.JsonRpcProvider(RPC_URL, {
        chainId: CHAIN_ID,
        name: CHAIN_ID === 5003 ? 'Mantle Testnet' : 'Mantle Mainnet'
    });

    const wallet = new ethers.Wallet(PRIVATE_KEY, provider);
    const address = await wallet.getAddress();

    console.log(`   Network: ${CHAIN_ID === 5003 ? 'Testnet' : 'Mainnet'} (Chain ID: ${CHAIN_ID})`);
    console.log(`   RPC: ${RPC_URL}`);
    console.log(`   Wallet: ${address}`);
    console.log('');

    // Check balance
    const balance = await provider.getBalance(address);
    console.log(`💰 Balance: ${ethers.formatEther(balance)} MNT`);

    if (balance === 0n) {
        console.error('');
        console.error('❌ Error: Insufficient balance. Get testnet MNT from:');
        console.error('   https://faucet.testnet.mantle.xyz');
        process.exit(1);
    }
    console.log('');

    // Connect to contract
    console.log('📝 Contract Details:');
    console.log(`   Address: ${REGISTRY_ADDRESS}`);
    const contract = new ethers.Contract(REGISTRY_ADDRESS, abi, wallet);
    console.log('');

    // Display batch info
    console.log('📦 Batch to Anchor:');
    console.log(`   Root: ${root}`);
    console.log(`   Meta URI: ${metaURI}`);
    console.log('');

    // Estimate gas
    console.log('⛽ Estimating gas...');
    try {
        const gasEstimate = await contract.registerBatch.estimateGas(root, metaURI);
        console.log(`   Estimated gas: ${gasEstimate.toString()}`);
    } catch (error) {
        console.error('❌ Gas estimation failed:', error.message);
        process.exit(1);
    }
    console.log('');

    // Send transaction
    console.log('🚀 Sending transaction...');
    try {
        const tx = await contract.registerBatch(root, metaURI);
        console.log(`   Transaction hash: ${tx.hash}`);
        console.log('');

        console.log('⏳ Waiting for confirmation...');
        const receipt = await tx.wait();
        console.log(`   ✅ Confirmed in block ${receipt.blockNumber}`);
        console.log('');

        // Parse event to get batch ID
        const event = receipt.logs
            .map(log => {
                try {
                    return contract.interface.parseLog(log);
                } catch {
                    return null;
                }
            })
            .find(e => e && e.name === 'BatchRegistered');

        if (event) {
            const mantleBatchId = event.args.batchId.toString();

            console.log('═══════════════════════════════════════════════════════════');
            console.log('✅ SUCCESS! Batch anchored on Mantle');
            console.log('═══════════════════════════════════════════════════════════');
            console.log('');
            console.log('📊 Results:');
            console.log(`   Mantle Batch ID: ${mantleBatchId}`);
            console.log(`   Transaction Hash: ${tx.hash}`);
            console.log(`   Block Number: ${receipt.blockNumber}`);
            console.log(`   Gas Used: ${receipt.gasUsed.toString()}`);
            console.log('');

            const explorerUrl = CHAIN_ID === 5003
                ? `https://explorer.testnet.mantle.xyz/tx/${tx.hash}`
                : `https://explorer.mantle.xyz/tx/${tx.hash}`;

            console.log('🔍 View on Explorer:');
            console.log(`   ${explorerUrl}`);
            console.log('');

            console.log('💾 Save this information:');
            console.log(`   mantle_batch_id: ${mantleBatchId}`);
            console.log(`   tx_hash: ${tx.hash}`);
            console.log('');

            // Return structured data for programmatic use
            return {
                success: true,
                mantleBatchId,
                txHash: tx.hash,
                blockNumber: receipt.blockNumber,
                gasUsed: receipt.gasUsed.toString(),
                explorerUrl
            };
        } else {
            console.error('❌ Error: Could not parse BatchRegistered event');
            process.exit(1);
        }
    } catch (error) {
        console.error('');
        console.error('❌ Transaction failed:', error.message);
        if (error.data) {
            console.error('   Error data:', error.data);
        }
        process.exit(1);
    }
}

// Run the script
anchorBatch()
    .then(() => process.exit(0))
    .catch(error => {
        console.error('');
        console.error('❌ Unexpected error:', error);
        process.exit(1);
    });
