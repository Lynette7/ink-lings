#!/bin/bash

# Setup script for integration testing environment

set -e

echo "🔧 Setting up integration test environment..."

# Check Rust installation
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed: $(rustc --version)"

# Install substrate-contracts-node
echo "📦 Installing substrate-contracts-node..."
if ! command -v substrate-contracts-node &> /dev/null; then
    cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force --locked
else
    echo "✅ substrate-contracts-node already installed"
fi

# Install cargo-contract
echo "📦 Installing cargo-contract..."
if ! command -v cargo-contract &> /dev/null; then
    cargo install cargo-contract --force
else
    echo "✅ cargo-contract already installed"
fi

# Verify installations
echo ""
echo "🎉 Environment setup complete!"
echo ""
echo "Installed tools:"
echo "  - substrate-contracts-node: $(substrate-contracts-node --version 2>/dev/null || echo 'installation pending')"
echo "  - cargo-contract: $(cargo-contract --version)"
echo ""
echo "Run integration tests with: ./scripts/run_integration_tests.sh"
