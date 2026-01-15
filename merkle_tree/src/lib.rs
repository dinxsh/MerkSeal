use sha2::{Digest, Sha256};

/// 32-byte hash type (SHA-256 output)
pub type Hash = [u8; 32];

/// Merkle tree for verifiable file integrity
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// All nodes stored in breadth-first order (root at index 0)
    nodes: Vec<Hash>,
    /// Number of leaf nodes
    #[allow(dead_code)]
    leaf_count: usize,
}

impl MerkleTree {
    /// Build a Merkle tree from file hashes
    pub fn new(mut leaf_hashes: Vec<Hash>) -> Self {
        let leaf_count = leaf_hashes.len();
        
        if leaf_count == 0 {
            panic!("Cannot create Merkle tree with zero leaves");
        }
        
        // Pad to next power of 2 for simplicity
        let next_pow2 = leaf_count.next_power_of_two();
        while leaf_hashes.len() < next_pow2 {
            leaf_hashes.push([0u8; 32]); // Pad with zero hashes
        }
        
        let mut nodes = Vec::new();
        let mut current_level = leaf_hashes;
        
        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = &current_level[i + 1];
                let parent = hash_pair(left, right);
                next_level.push(parent);
            }
            
            current_level = next_level;
        }
        
        // Root is the last remaining node
        let root = current_level[0];
        
        // For simplicity, just store root (can expand to full tree if needed)
        nodes.push(root);
        
        Self { nodes, leaf_count }
    }
    
    /// Get the Merkle root hash
    pub fn root(&self) -> Hash {
        self.nodes[0]
    }
    
    /// Get the root as a hex string
    pub fn root_hex(&self) -> String {
        hex::encode(self.root())
    }
}

/// Hash a pair of nodes to create parent hash
fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

/// Hash arbitrary data (e.g., file contents)
pub fn hash_data(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Convert hex string to Hash
pub fn hex_to_hash(hex_str: &str) -> Result<Hash, String> {
    let bytes = hex::decode(hex_str).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", bytes.len()));
    }
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&bytes);
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_file() {
        let hash1 = hash_data(b"file1");
        let tree = MerkleTree::new(vec![hash1]);
        assert_eq!(tree.leaf_count, 1);
    }

    #[test]
    fn test_multiple_files() {
        let hash1 = hash_data(b"file1");
        let hash2 = hash_data(b"file2");
        let hash3 = hash_data(b"file3");
        
        let tree = MerkleTree::new(vec![hash1, hash2, hash3]);
        assert_eq!(tree.leaf_count, 3);
        
        // Root should be deterministic
        let root1 = tree.root();
        let tree2 = MerkleTree::new(vec![hash1, hash2, hash3]);
        let root2 = tree2.root();
        assert_eq!(root1, root2);
    }

    #[test]
    fn test_root_changes_with_data() {
        let hash1 = hash_data(b"file1");
        let hash2 = hash_data(b"file2");
        
        let tree1 = MerkleTree::new(vec![hash1, hash2]);
        let tree2 = MerkleTree::new(vec![hash1, hash_data(b"file2_modified")]);
        
        assert_ne!(tree1.root(), tree2.root());
    }

    #[test]
    fn test_hex_conversion() {
        let hash = hash_data(b"test");
        let hex_str = hex::encode(hash);
        let decoded = hex_to_hash(&hex_str).unwrap();
        assert_eq!(hash, decoded);
    }
}
