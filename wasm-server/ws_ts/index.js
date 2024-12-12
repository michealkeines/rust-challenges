"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const wasm_sock_1 = require("wasm_sock");
const TEST_WS_ENDPOINT = 'ws://localhost:8082';
const TEST_MESSAGE = 'Hello, WebSocket!';
function runTest() {
    return __awaiter(this, void 0, void 0, function* () {
        console.log('Initializing Wasm module...');
        try {
            console.log('Wasm module initialized.');
            console.log(`Testing WebSocket connection to ${TEST_WS_ENDPOINT} with message: "${TEST_MESSAGE}"`);
            // Use the `ws_ping` function and await the response
            const response = yield (0, wasm_sock_1.ws_ping)(TEST_WS_ENDPOINT, TEST_MESSAGE);
            console.log('WebSocket response received:', response);
        }
        catch (error) {
            console.error('Error during WebSocket test:', error);
        }
    });
}
runTest();
