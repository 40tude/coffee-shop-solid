# Coffee Shop SOLID - Quick Start Script for Windows
# This script helps you get started with the project on Windows

Write-Host "☕ Coffee Shop Order System - SOLID Principles Demo" -ForegroundColor Cyan
Write-Host "====================================================`n" -ForegroundColor Cyan

# Check if Rust is installed
Write-Host "🔍 Checking for Rust installation..." -ForegroundColor Yellow
$rustInstalled = Get-Command cargo -ErrorAction SilentlyContinue

if (-not $rustInstalled) {
    Write-Host "❌ Rust is not installed." -ForegroundColor Red
    Write-Host "`nPlease install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installation, restart PowerShell and run this script again.`n"
    exit 1
}

Write-Host "✓ Rust is installed" -ForegroundColor Green
cargo --version

# Build the project
Write-Host "`n🔨 Building the project..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Build successful!" -ForegroundColor Green

    # Run tests
    Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
    cargo test

    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n✓ All tests passed!" -ForegroundColor Green

        Write-Host "`n📚 Project is ready! Here are some commands:" -ForegroundColor Cyan
        Write-Host "  cargo run          - Start the interactive demo" -ForegroundColor White
        Write-Host "  cargo test         - Run all tests" -ForegroundColor White
        Write-Host "  cargo doc --open   - Open the documentation" -ForegroundColor White

        Write-Host "`n🚀 Starting the interactive demo..." -ForegroundColor Yellow
        Write-Host "(Press Ctrl+C to exit at any time)`n" -ForegroundColor Gray

        Start-Sleep -Seconds 2
        cargo run
    }
    else {
        Write-Host "`n❌ Tests failed. Please check the output above." -ForegroundColor Red
    }
}
else {
    Write-Host "`n❌ Build failed. Please check the output above." -ForegroundColor Red
}
