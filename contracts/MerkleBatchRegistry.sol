// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title MerkleBatchRegistry
 * @notice Minimal registry contract for anchoring MerkSeal batch roots on Mantle L2
 * @dev Designed for hackathon demo - prioritizes simplicity and clarity
 */
contract MerkleBatchRegistry {
    // ============ State Variables ============

    /// @notice Counter for batch IDs (starts at 1)
    uint256 private _nextBatchId = 1;

    /// @notice Batch metadata structure
    struct Batch {
        bytes32 root; // Merkle root hash
        address owner; // Address that registered this batch
        string metaURI; // URI to batch metadata (IPFS, HTTP, etc.)
        uint256 timestamp; // Block timestamp when registered
    }

    /// @notice Mapping from batch ID to batch data
    mapping(uint256 => Batch) public batches;

    // ============ Events ============

    /// @notice Emitted when a new batch is registered
    /// @param batchId Unique identifier for this batch
    /// @param root Merkle root hash of the batch
    /// @param owner Address that registered the batch
    /// @param metaURI URI pointing to batch metadata
    event BatchRegistered(
        uint256 indexed batchId,
        bytes32 indexed root,
        address indexed owner,
        string metaURI
    );

    // ============ External Functions ============

    /**
     * @notice Register a new Merkle batch on-chain
     * @param root The Merkle root hash representing all files in this batch
     * @param metaURI URI pointing to batch metadata (file list, timestamps, etc.)
     * @return batchId The unique ID assigned to this batch
     */
    function registerBatch(
        bytes32 root,
        string calldata metaURI
    ) external returns (uint256 batchId) {
        require(root != bytes32(0), "MerkleBatchRegistry: root cannot be zero");

        batchId = _nextBatchId++;

        batches[batchId] = Batch({
            root: root,
            owner: msg.sender,
            metaURI: metaURI,
            timestamp: block.timestamp
        });

        emit BatchRegistered(batchId, root, msg.sender, metaURI);
    }

    /**
     * @notice Retrieve batch information by ID
     * @param batchId The batch ID to query
     * @return root The Merkle root hash
     * @return owner The address that registered this batch
     * @return metaURI The metadata URI
     * @return timestamp The block timestamp when registered
     */
    function getBatch(
        uint256 batchId
    )
        external
        view
        returns (
            bytes32 root,
            address owner,
            string memory metaURI,
            uint256 timestamp
        )
    {
        require(
            batchId > 0 && batchId < _nextBatchId,
            "MerkleBatchRegistry: invalid batch ID"
        );

        Batch memory batch = batches[batchId];
        return (batch.root, batch.owner, batch.metaURI, batch.timestamp);
    }

    /**
     * @notice Get the total number of registered batches
     * @return count Total batches registered
     */
    function getBatchCount() external view returns (uint256 count) {
        return _nextBatchId - 1;
    }

    /**
     * @notice Verify if a given root matches the stored root for a batch
     * @param batchId The batch ID to check
     * @param root The root hash to verify
     * @return isValid True if the root matches
     */
    function verifyRoot(
        uint256 batchId,
        bytes32 root
    ) external view returns (bool isValid) {
        require(
            batchId > 0 && batchId < _nextBatchId,
            "MerkleBatchRegistry: invalid batch ID"
        );
        return batches[batchId].root == root;
    }
}
