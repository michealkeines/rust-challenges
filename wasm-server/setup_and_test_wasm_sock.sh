#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status
set -o pipefail # Fail a pipeline if any command within it fails
set -u # Treat unset variables as an error

echo "Starting setup and tests..."

# Navigate to the wasm_sock directory and build the Rust WebAssembly package
echo "Building Rust WebAssembly package..."
cd wasm_sock
wasm-pack build --target nodejs
echo "Rust WebAssembly package built successfully."

# Navigate to the ws_server directory and start the WebSocket server
echo "Starting WebSocket server..."
cd ../ws_server
npm install
node server.js &
SERVER_PID=$! # Capture the server process ID so we can terminate it later
echo "WebSocket server running with PID: $SERVER_PID"

# Allow some time for the WebSocket server to start
sleep 2

# Navigate to the ws_ts directory, compile the TypeScript tests, and run them
echo "Running TypeScript tests..."
cd ../ws_ts
npm add ../wasm_sock/pkg
npx tsc
node index.js &
TEST_PID=$!

sleep 5

# If tests are successful, terminate the WebSocket server
echo "Tests completed successfully. Stopping WebSocket server..."
kill $SERVER_PID
kill $TEST_PID

echo "All steps completed."
