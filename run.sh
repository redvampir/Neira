#!/bin/bash

echo "Starting Neira..."

# Create necessary directories
mkdir -p data logs

# Set environment variables
export RUST_LOG=info
export NEIRA_DATA_DIR=./data
export NEIRA_SUCCESS_THRESHOLD=0.8

# Build and run
echo "Building Neira..."
cargo build --release || {
    echo "Build failed!"
    exit 1
}

# Start server
echo "Starting server..."
./target/release/neira &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Open web interface
echo "Opening web interface..."
if command -v xdg-open &> /dev/null; then
    xdg-open http://localhost:9090/ui
elif command -v open &> /dev/null; then
    open http://localhost:9090/ui
fi

echo "Neira is running! Press Ctrl+C to stop."
trap "kill $SERVER_PID" INT
wait
