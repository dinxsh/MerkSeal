use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use futures_util::TryStreamExt;
use mantle_config::MantleConfig;
use merkle_tree::{hash_data, MerkleTree};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Global batch ID counter
static BATCH_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Batch metadata returned after upload
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchMetadata {
    /// Local batch ID (incremental)
    pub local_batch_id: u64,
    /// Merkle root hash (hex string)
    pub root: String,
    /// Number of files in this batch
    pub file_count: usize,
    /// Suggested metadata URI (placeholder for now)
    pub suggested_meta_uri: String,
    /// Registry contract address (for anchoring)
    pub registry_address: String,
}

/// Upload response
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub batch: BatchMetadata,
}

/// POST /upload - Accept files, compute Merkle root, return batch metadata
#[post("/upload")]
async fn upload_files(
    mut payload: Multipart,
    config: web::Data<Arc<MantleConfig>>,
) -> impl Responder {
    let batch_id = BATCH_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    // Create batch directory
    let batch_dir = PathBuf::from(format!("batches/{}", batch_id));
    if let Err(e) = fs::create_dir_all(&batch_dir) {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to create batch directory: {}", e)
        }));
    }
    
    let mut file_hashes = Vec::new();
    let mut file_count = 0;
    
    // Process each uploaded file
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("file_{}", file_count));
        
        let filepath = batch_dir.join(&filename);
        
        // Read file data
        let mut file_data = Vec::new();
        while let Ok(Some(chunk)) = field.try_next().await {
            file_data.extend_from_slice(&chunk);
        }
        
        // Hash the file
        let file_hash = hash_data(&file_data);
        file_hashes.push(file_hash);
        
        // Save file to disk
        if let Err(e) = fs::File::create(&filepath)
            .and_then(|mut f| f.write_all(&file_data))
        {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to save file {}: {}", filename, e)
            }));
        }
        
        file_count += 1;
    }
    
    if file_hashes.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "No files uploaded"
        }));
    }
    
    // Build Merkle tree and compute root
    let tree = MerkleTree::new(file_hashes);
    let root_hex = tree.root_hex();
    
    // Create batch metadata
    let batch_metadata = BatchMetadata {
        local_batch_id: batch_id,
        root: root_hex.clone(),
        file_count,
        suggested_meta_uri: format!("ipfs://placeholder-{}", batch_id),
        registry_address: config.registry_address.clone(),
    };
    
    // Save batch metadata to disk
    let metadata_path = batch_dir.join("metadata.json");
    if let Err(e) = fs::write(
        &metadata_path,
        serde_json::to_string_pretty(&batch_metadata).unwrap(),
    ) {
        eprintln!("Warning: Failed to save metadata: {}", e);
    }
    
    println!("✓ Batch {} uploaded:", batch_id);
    println!("  Files: {}", file_count);
    println!("  Root: {}", root_hex);
    println!("  Saved to: {}", batch_dir.display());
    
    HttpResponse::Ok().json(UploadResponse {
        success: true,
        batch: batch_metadata,
    })
}

/// Health check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "MerkSeal Server"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("MerkSeal Server");
    
    // Load Mantle configuration
    let config = match MantleConfig::from_env() {
        Ok(config) => {
            println!("✓ Mantle config loaded:");
            println!("  RPC URL: {}", config.rpc_url);
            println!("  Chain ID: {}", config.chain_id);
            println!("  Registry: {}", config.registry_address);
            println!("  Network: {}", if config.is_testnet() { "Testnet" } else { "Mainnet" });
            Arc::new(config)
        }
        Err(e) => {
            eprintln!("✗ Failed to load config: {}", e);
            eprintln!("  Make sure MERKLE_BATCH_REGISTRY_ADDRESS is set in .env");
            std::process::exit(1);
        }
    };
    
    // Create batches directory
    fs::create_dir_all("batches").ok();
    
    let host = "127.0.0.1";
    let port = 8080;
    
    println!("\n🚀 Server starting on http://{}:{}", host, port);
    println!("   POST /upload - Upload files and get batch metadata");
    println!("   GET  /health - Health check");
    println!("\nReady to accept uploads!");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(upload_files)
            .route("/health", web::get().to(health))
    })
    .bind((host, port))?
    .run()
    .await
}
