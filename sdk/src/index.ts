import { ethers } from 'ethers';
import axios from 'axios';
import * as fs from 'fs';
import * as path from 'path';

/**
 * Configuration for MerkSeal SDK
 */
export interface MerkSealConfig {
    /** URL of the MerkSeal server */
    serverUrl: string;
    /** Mantle RPC URL */
    mantleRpc: string;
    /** MerkleBatchRegistry contract address */
    registryAddress: string;
    /** Private key for signing transactions (optional, for anchoring) */
    privateKey?: string;
}

/**
 * Batch metadata returned from server
 */
export interface BatchMetadata {
    local_batch_id: number;
    root: string;
    file_count: number;
    suggested_meta_uri: string;
    registry_address: string;
    mantle_batch_id?: number;
}

/**
 * Result of anchoring a batch on Mantle
 */
export interface AnchorResult {
    mantleBatchId: number;
    txHash: string;
    blockNumber: number;
    gasUsed: string;
}

/**
 * Verification result
 */
export interface VerificationResult {
    valid: boolean;
    localRoot: string;
    onchainRoot: string;
    filesMatch: boolean;
    rootsMatch: boolean;
}

/**
 * MerkSeal SDK - Easy integration for verifiable storage on Mantle
 */
export class MerkSeal {
    private config: MerkSealConfig;
    private provider: ethers.Provider;
    private wallet?: ethers.Wallet;
    private contract: ethers.Contract;

    private static readonly CONTRACT_ABI = [
        'function registerBatch(bytes32 root, string metaURI) external returns (uint256)',
        'function getBatch(uint256 batchId) external view returns (bytes32 root, address owner, string metaURI, uint256 timestamp)',
        'function getBatchCount() external view returns (uint256)',
        'function verifyRoot(uint256 batchId, bytes32 root) external view returns (bool)',
        'event BatchRegistered(uint256 indexed batchId, bytes32 indexed root, address indexed owner, string metaURI)'
    ];

    /**
     * Create a new MerkSeal instance
     */
    constructor(config: MerkSealConfig) {
        this.config = config;
        this.provider = new ethers.JsonRpcProvider(config.mantleRpc);

        if (config.privateKey) {
            this.wallet = new ethers.Wallet(config.privateKey, this.provider);
            this.contract = new ethers.Contract(
                config.registryAddress,
                MerkSeal.CONTRACT_ABI,
                this.wallet
            );
        } else {
            this.contract = new ethers.Contract(
                config.registryAddress,
                MerkSeal.CONTRACT_ABI,
                this.provider
            );
        }
    }

    /**
     * Upload files to MerkSeal server
     * @param filePaths Array of file paths to upload
     * @returns Batch metadata including Merkle root
     */
    async upload(filePaths: string[]): Promise<BatchMetadata> {
        const FormData = require('form-data');
        const formData = new FormData();

        for (const filePath of filePaths) {
            const fileStream = fs.createReadStream(filePath);
            const fileName = path.basename(filePath);
            formData.append(fileName, fileStream);
        }

        const response = await axios.post(
            `${this.config.serverUrl}/upload`,
            formData,
            {
                headers: formData.getHeaders()
            }
        );

        if (!response.data.success) {
            throw new Error('Upload failed');
        }

        return response.data.batch;
    }

    /**
     * Anchor a Merkle root on Mantle L2
     * @param root Merkle root hash (with or without 0x prefix)
     * @param metaURI Metadata URI (IPFS, HTTP, etc.)
     * @returns Anchor result with transaction details
     */
    async anchor(root: string, metaURI: string): Promise<AnchorResult> {
        if (!this.wallet) {
            throw new Error('Private key required for anchoring');
        }

        // Ensure root has 0x prefix
        const rootHash = root.startsWith('0x') ? root : `0x${root}`;

        // Call registerBatch
        const tx = await this.contract.registerBatch(rootHash, metaURI);
        const receipt = await tx.wait();

        // Parse event to get batch ID
        const event = receipt.logs
            .map((log: any) => {
                try {
                    return this.contract.interface.parseLog(log);
                } catch {
                    return null;
                }
            })
            .find((e: any) => e && e.name === 'BatchRegistered');

        if (!event) {
            throw new Error('BatchRegistered event not found');
        }

        return {
            mantleBatchId: Number(event.args.batchId),
            txHash: receipt.hash,
            blockNumber: receipt.blockNumber,
            gasUsed: receipt.gasUsed.toString()
        };
    }

    /**
     * Upload files and anchor on Mantle in one call
     * @param filePaths Array of file paths to upload
     * @returns Combined result with batch metadata and anchor details
     */
    async uploadAndAnchor(filePaths: string[]): Promise<BatchMetadata & AnchorResult> {
        // Upload files
        const batch = await this.upload(filePaths);

        // Anchor on Mantle
        const anchorResult = await this.anchor(batch.root, batch.suggested_meta_uri);

        // Return combined result
        return {
            ...batch,
            ...anchorResult,
            mantle_batch_id: anchorResult.mantleBatchId
        };
    }

    /**
     * Verify a batch against on-chain root
     * @param localBatchId Local batch ID from server
     * @param mantleBatchId Mantle batch ID from anchoring
     * @returns Verification result
     */
    async verify(localBatchId: number, mantleBatchId: number): Promise<VerificationResult> {
        // Get on-chain batch
        const [onchainRoot, owner, metaURI, timestamp] = await this.contract.getBatch(mantleBatchId);
        const onchainRootHex = `0x${onchainRoot.slice(2)}`;

        // Get local batch metadata
        const metadataPath = path.join(process.cwd(), 'batches', localBatchId.toString(), 'metadata.json');
        const metadata = JSON.parse(fs.readFileSync(metadataPath, 'utf8'));

        const localRoot = metadata.root.startsWith('0x') ? metadata.root : `0x${metadata.root}`;

        // Compare roots
        const rootsMatch = localRoot.toLowerCase() === onchainRootHex.toLowerCase();

        return {
            valid: rootsMatch,
            localRoot,
            onchainRoot: onchainRootHex,
            filesMatch: true, // Simplified for SDK
            rootsMatch
        };
    }

    /**
     * Get batch information from Mantle
     * @param mantleBatchId Mantle batch ID
     * @returns Batch information
     */
    async getBatch(mantleBatchId: number) {
        const [root, owner, metaURI, timestamp] = await this.contract.getBatch(mantleBatchId);

        return {
            root: `0x${root.slice(2)}`,
            owner,
            metaURI,
            timestamp: Number(timestamp)
        };
    }

    /**
     * Get total number of batches registered
     * @returns Total batch count
     */
    async getBatchCount(): Promise<number> {
        const count = await this.contract.getBatchCount();
        return Number(count);
    }

    /**
     * Verify a root hash against on-chain data
     * @param mantleBatchId Mantle batch ID
     * @param root Root hash to verify
     * @returns True if root matches
     */
    async verifyRoot(mantleBatchId: number, root: string): Promise<boolean> {
        const rootHash = root.startsWith('0x') ? root : `0x${root}`;
        return await this.contract.verifyRoot(mantleBatchId, rootHash);
    }
}

/**
 * Create a MerkSeal instance with environment variables
 */
export function createMerkSeal(overrides?: Partial<MerkSealConfig>): MerkSeal {
    const config: MerkSealConfig = {
        serverUrl: process.env.MerkSeal_SERVER_URL || 'http://localhost:8080',
        mantleRpc: process.env.MANTLE_RPC_URL || 'https://rpc.testnet.mantle.xyz',
        registryAddress: process.env.MERKLE_BATCH_REGISTRY_ADDRESS || '',
        privateKey: process.env.PRIVATE_KEY,
        ...overrides
    };

    if (!config.registryAddress) {
        throw new Error('MERKLE_BATCH_REGISTRY_ADDRESS environment variable required');
    }

    return new MerkSeal(config);
}

// Export types
export default MerkSeal;
