import { ws_ping} from "wasm_sock";

const TEST_WS_ENDPOINT = 'ws://localhost:8082';
const TEST_MESSAGE = 'Hello, WebSocket!';

async function runTest() {
  console.log('Initializing Wasm module...');
  try {

    console.log('Wasm module initialized.');
    console.log(`Testing WebSocket connection to ${TEST_WS_ENDPOINT} with message: "${TEST_MESSAGE}"`);

    // Use the `ws_ping` function and await the response
    const response = await ws_ping(TEST_WS_ENDPOINT, TEST_MESSAGE);
    console.log('WebSocket response received:', response);
  } catch (error) {
    console.error('Error during WebSocket test:', error);
  }
}

runTest();
