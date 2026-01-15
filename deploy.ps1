# MerkleBatchRegistry Deployment Script for Mantle Testnet (PowerShell)
# Usage: .\deploy.ps1

$ErrorActionPreference = "Stop"

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "MerkSeal Contract Deployment" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

# Check if .env exists
if (-not (Test-Path .env)) {
    Write-Host "❌ .env file not found!" -ForegroundColor Red
    Write-Host "   Copy .env.example to .env and fill in your values"
    exit 1
}

# Load environment variables from .env
Get-Content .env | ForEach-Object {
    if ($_ -match '^\s*([^#][^=]*?)\s*=\s*(.*?)\s*$') {
        $name = $matches[1]
        $value = $matches[2]
        Set-Item -Path "env:$name" -Value $value
    }
}

# Validate required variables
if (-not $env:MANTLE_RPC_URL) {
    Write-Host "❌ MANTLE_RPC_URL not set in .env" -ForegroundColor Red
    exit 1
}

if (-not $env:PRIVATE_KEY) {
    Write-Host "❌ PRIVATE_KEY not set in .env" -ForegroundColor Red
    exit 1
}

$chainId = if ($env:MANTLE_CHAIN_ID) { $env:MANTLE_CHAIN_ID } else { "5001" }

Write-Host "📋 Configuration:" -ForegroundColor Yellow
Write-Host "   RPC URL: $env:MANTLE_RPC_URL"
Write-Host "   Chain ID: $chainId"
Write-Host ""

Push-Location contracts

Write-Host "🔨 Deploying MerkleBatchRegistry..." -ForegroundColor Green
Write-Host ""

# Use forge script for deployment (more reliable than forge create)
forge script script/Deploy.s.sol `
    --rpc-url $env:MANTLE_RPC_URL `
    --broadcast `
    --private-key $env:PRIVATE_KEY `
    --legacy `
    -vvv

Pop-Location

Write-Host ""
Write-Host "✅ Deployment complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Yellow
Write-Host "   1. Copy the deployed contract address from above"
Write-Host "   2. Update .env: MERKLE_BATCH_REGISTRY_ADDRESS=<address>"
Write-Host "   3. Run client tests: cargo run --release -p client config"
Write-Host ""
