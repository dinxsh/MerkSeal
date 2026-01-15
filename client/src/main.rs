use clap::{Parser, Subcommand};
use ethers::prelude::*;
use mantle_config::MantleConfig;
use merkle_tree::{hash_data, MerkleTree};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// MerkSeal Client - Verifiable file storage with Mantle L2 anchoring
#[derive(Parser)]
#[command(name = "MerkSeal")]
#[command(about = "MerkSeal client for verifiable file storage", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify a batch against Mantle L2
    Verify {
        /// Local batch ID to verify
        #[arg(short, long)]
        batch_id: u64,
        
        /// Mantle batch ID (optional, will read from metadata if not provided)
        #[arg(short, long)]
        mantle_batch_id: Option<u64>,
    },
    
    /// Show configuration
    Config,
}

/// Batch metadata (matches server format)
#[derive(Debug, Serialize, Deserialize)]
struct BatchMetadata {
    local_batch_id: u64,
    root: String,
    file_count: usize,
    suggested_meta_uri: String,
    registry_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mantle_batch_id: Option<u64>,
}

abigen!(
    MerkleBatchRegistry,
    r#"[
        function getBatch(uint256 batchId) external view returns (bytes32 root, address owner, string memory metaURI, uint256 timestamp)
    ]"#
);

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    // Load Mantle configuration
    let config = match MantleConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            eprintln!("   Make sure MERKLE_BATCH_REGISTRY_ADDRESS is set in .env");
            std::process::exit(1);
        }
    };
    
    match cli.command {
        Commands::Config => {
            println!("MerkSeal Client Configuration");
            println!("═══════════════════════════════════════════════════════════");
            println!("  RPC URL: {}", config.rpc_url);
            println!("  Chain ID: {}", config.chain_id);
            println!("  Registry: {}", config.registry_address);
            println!("  Network: {}", if config.is_testnet() { "Testnet" } else { "Mainnet" });
            println!("  Explorer: {}", config.contract_url());
        }
        
        Commands::Verify { batch_id, mantle_batch_id } => {
            if let Err(e) = verify_batch(&config, batch_id, mantle_batch_id).await {
                eprintln!("\n❌ Verification failed: {}", e);
                std::process::exit(1);
            }
        }
    }
}

async fn verify_batch(
    config: &MantleConfig,
    local_batch_id: u64,
    mantle_batch_id_arg: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 MerkSeal Batch Verification");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // 1. Load local batch metadata
    println!("📂 Loading local batch metadata...");
    let batch_dir = PathBuf::from(format!("batches/{}", local_batch_id));
    let metadata_path = batch_dir.join("metadata.json");
    
    if !metadata_path.exists() {
        return Err(format!("Batch {} not found at {}", local_batch_id, metadata_path.display()).into());
    }
    
    let metadata_str = fs::read_to_string(&metadata_path)?;
    let metadata: BatchMetadata = serde_json::from_str(&metadata_str)?;
    
    println!("   ✓ Local batch ID: {}", metadata.local_batch_id);
    println!("   ✓ Local root: {}", metadata.root);
    println!("   ✓ File count: {}", metadata.file_count);
    println!();
    
    // 2. Determine Mantle batch ID
    let mantle_batch_id = match mantle_batch_id_arg {
        Some(id) => {
            println!("📝 Using provided Mantle batch ID: {}", id);
            id
        }
        None => {
            if let Some(id) = metadata.mantle_batch_id {
                println!("📝 Using Mantle batch ID from metadata: {}", id);
                id
            } else {
                return Err("Mantle batch ID not found. Provide with --mantle-batch-id or add to metadata.json".into());
            }
        }
    };
    println!();
    
    // 3. Query Mantle for on-chain root
    println!("🔗 Querying Mantle L2...");
    println!("   Network: {}", if config.is_testnet() { "Testnet" } else { "Mainnet" });
    println!("   RPC: {}", config.rpc_url);
    
    let provider = Provider::<Http>::try_from(&config.rpc_url)?;
    let registry_address: Address = config.registry_address.parse()?;
    let contract = MerkleBatchRegistry::new(registry_address, provider.into());
    
    let (onchain_root, owner, meta_uri, timestamp) = contract
        .get_batch(U256::from(mantle_batch_id))
        .call()
        .await?;
    
    let onchain_root_hex = format!("0x{}", hex::encode(onchain_root));
    
    println!("   ✓ On-chain root: {}", onchain_root_hex);
    println!("   ✓ Owner: {}", owner);
    println!("   ✓ Meta URI: {}", meta_uri);
    println!("   ✓ Timestamp: {}", timestamp);
    println!();
    
    // 4. Compare roots
    println!("🔐 Verifying Merkle root...");
    
    let local_root = if metadata.root.starts_with("0x") {
        metadata.root.clone()
    } else {
        format!("0x{}", metadata.root)
    };
    
    let roots_match = local_root.to_lowercase() == onchain_root_hex.to_lowercase();
    
    if roots_match {
        println!("   ✅ ROOT MATCH!");
        println!("   Local root:    {}", local_root);
        println!("   On-chain root: {}", onchain_root_hex);
    } else {
        println!("   ❌ ROOT MISMATCH!");
        println!("   Local root:    {}", local_root);
        println!("   On-chain root: {}", onchain_root_hex);
        println!();
        println!("⚠️  WARNING: Roots do not match!");
        println!("   This could indicate:");
        println!("   - Wrong Mantle batch ID");
        println!("   - Local files have been modified");
        println!("   - Metadata corruption");
        return Err("Root verification failed".into());
    }
    println!();
    
    // 5. Verify local files match local root
    println!("📁 Verifying local files...");
    
    let mut file_hashes = Vec::new();
    let mut files: Vec<_> = fs::read_dir(&batch_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.file_name() != Some(std::ffi::OsStr::new("metadata.json"))
        })
        .collect();
    
    files.sort_by_key(|e| e.path());
    
    for entry in &files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        let data = fs::read(&path)?;
        let hash = hash_data(&data);
        file_hashes.push(hash);
        println!("   ✓ {}: {} bytes", filename, data.len());
    }
    
    if file_hashes.is_empty() {
        return Err("No files found in batch directory".into());
    }
    
    let tree = MerkleTree::new(file_hashes);
    let computed_root = tree.root_hex();
    let computed_root_with_prefix = format!("0x{}", computed_root);
    
    println!();
    println!("   Computed root from files: {}", computed_root_with_prefix);
    
    let files_match_local = computed_root_with_prefix.to_lowercase() == local_root.to_lowercase();
    
    if files_match_local {
        println!("   ✅ Local files match local root!");
    } else {
        println!("   ❌ Local files DO NOT match local root!");
        println!("   This indicates local files have been modified.");
        return Err("Local file verification failed".into());
    }
    println!();
    
    // 6. Final summary
    println!("═══════════════════════════════════════════════════════════");
    println!("✅ VERIFICATION SUCCESSFUL!");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Summary:");
    println!("  ✓ Local files match local root");
    println!("  ✓ Local root matches on-chain root");
    println!("  ✓ Batch is verified and tamper-proof");
    println!();
    println!("Batch Details:");
    println!("  Local Batch ID: {}", local_batch_id);
    println!("  Mantle Batch ID: {}", mantle_batch_id);
    println!("  File Count: {}", metadata.file_count);
    println!("  Merkle Root: {}", local_root);
    println!();
    println!("🔍 View on Mantle Explorer:");
    println!("   {}", config.contract_url());
    println!();
    
    Ok(())
}
