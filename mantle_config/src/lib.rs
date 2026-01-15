use serde::{Deserialize, Serialize};
use std::env;

/// Mantle L2 network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MantleConfig {
    /// Mantle RPC endpoint URL
    pub rpc_url: String,
    
    /// Mantle chain ID (5003 for Sepolia testnet, 5000 for mainnet)
    pub chain_id: u64,
    
    /// Deployed MerkleBatchRegistry contract address
    pub registry_address: String,
}

impl MantleConfig {
    /// Load configuration from environment variables with sensible defaults
    pub fn from_env() -> Result<Self, ConfigError> {
        let rpc_url = env::var("MANTLE_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.sepolia.mantle.xyz".to_string());
        
        let chain_id = env::var("MANTLE_CHAIN_ID")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5003); // Default to Sepolia testnet
        
        let registry_address = env::var("MERKLE_BATCH_REGISTRY_ADDRESS")
            .map_err(|_| ConfigError::MissingRegistryAddress)?;
        
        Ok(Self {
            rpc_url,
            chain_id,
            registry_address,
        })
    }
    
    /// Create config with explicit values (useful for testing)
    pub fn new(rpc_url: String, chain_id: u64, registry_address: String) -> Self {
        Self {
            rpc_url,
            chain_id,
            registry_address,
        }
    }
    
    /// Check if using testnet
    pub fn is_testnet(&self) -> bool {
        self.chain_id == 5003
    }
    
    /// Check if using mainnet
    pub fn is_mainnet(&self) -> bool {
        self.chain_id == 5000
    }
    
    /// Get block explorer URL for this network
    pub fn explorer_url(&self) -> &str {
        if self.is_testnet() {
            "https://explorer.testnet.mantle.xyz"
        } else {
            "https://explorer.mantle.xyz"
        }
    }
    
    /// Get full explorer URL for a transaction
    pub fn tx_url(&self, tx_hash: &str) -> String {
        format!("{}/tx/{}", self.explorer_url(), tx_hash)
    }
    
    /// Get full explorer URL for the registry contract
    pub fn contract_url(&self) -> String {
        format!("{}/address/{}", self.explorer_url(), self.registry_address)
    }
}

impl Default for MantleConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://rpc.sepolia.mantle.xyz".to_string(),
            chain_id: 5003,
            registry_address: String::new(),
        }
    }
}

/// Configuration errors
#[derive(Debug)]
pub enum ConfigError {
    MissingRegistryAddress,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingRegistryAddress => {
                write!(f, "MERKLE_BATCH_REGISTRY_ADDRESS environment variable not set")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testnet_detection() {
        let config = MantleConfig::new(
            "https://rpc.sepolia.mantle.xyz".to_string(),
            5003,
            "0x1234".to_string(),
        );
        assert!(config.is_testnet());
        assert!(!config.is_mainnet());
    }

    #[test]
    fn test_mainnet_detection() {
        let config = MantleConfig::new(
            "https://rpc.mantle.xyz".to_string(),
            5000,
            "0x1234".to_string(),
        );
        assert!(config.is_mainnet());
        assert!(!config.is_testnet());
    }

    #[test]
    fn test_explorer_urls() {
        let config = MantleConfig::new(
            "https://rpc.sepolia.mantle.xyz".to_string(),
            5003,
            "0xABCD1234".to_string(),
        );
        
        assert_eq!(
            config.tx_url("0xdeadbeef"),
            "https://explorer.testnet.mantle.xyz/tx/0xdeadbeef"
        );
        
        assert_eq!(
            config.contract_url(),
            "https://explorer.testnet.mantle.xyz/address/0xABCD1234"
        );
    }
}
