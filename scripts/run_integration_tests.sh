#!/bin/bash

# Integration test runner for Inklings
# Starts a local substrate node and runs integration tests

set -e

echo "🚀 Starting integration tests..."

# Check if substrate-contracts-node is installed
if ! command -v substrate-contracts-node &> /dev/null; then
    echo "❌ substrate-contracts-node not found!"
    echo "Install it with: cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git"
    exit 1
fi

# Check if cargo-contract is installed
if ! command -v cargo-contract &> /dev/null; then
    echo "❌ cargo-contract not found!"
    echo "Install it with: cargo install cargo-contract --force"
    exit 1
fi

# Start substrate-contracts-node in background
echo "📦 Starting local substrate node..."
substrate-contracts-node --dev --tmp > /tmp/substrate-node.log 2>&1 &
NODE_PID=$!

# Give the node time to start
echo "⏳ Waiting for node to be ready..."
sleep 5

# Ensure node is killed on script exit
trap "echo '🛑 Stopping node...'; kill $NODE_PID 2>/dev/null || true" EXIT

# Run integration tests
echo "🧪 Running integration tests..."
cargo test --test integration_tests -- --test-threads=1 --nocapture

echo "✅ Integration tests completed!"
