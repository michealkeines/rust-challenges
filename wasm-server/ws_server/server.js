const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8082 }, () => {
  console.log('WS SEVER: WebSocket server started on ws://localhost:8082');
});

wss.on('connection', (ws) => {
  console.log('WS SEVER: Client connected');

  ws.on('message', (message) => {
    console.log(`WS SEVER: Received message: ${message}`);
    ws.send(`Echo: ${message}`);
  });

  ws.on('close', () => {
    console.log('WS SEVER: Client disconnected');
  });

  ws.on('error', (err) => {
    console.error(`WS SEVER: WebSocket error: ${err.message}`);
  });
});
