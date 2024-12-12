use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

// Macros for logging to the console
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
macro_rules! console_err {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

// Initialize WebAssembly module
#[wasm_bindgen(start)]
pub fn init() {
    console_log!("Rust: WebAssembly module initialized");
}

/// Establishes a WebSocket connection to the given endpoint, sends a message, and resolves with the received response.
#[wasm_bindgen]
pub async fn ws_ping(endpoint: &str, message: &str) -> Promise {
    Promise::new(&mut move |resolve, reject| {
        console_log!("Rust: Connecting to {}", endpoint);

        // Create a new WebSocket instance
        let ws = match WebSocket::new(endpoint) {
            Ok(ws) => ws,
            Err(_) => {
                console_err!("Rust: Failed to create WebSocket");
                reject.call1(&JsValue::NULL, &JsValue::from("Rust: Failed to create WebSocket"))
                    .expect("Rust: Failed to reject promise");
                return;
            }
        };

        // Clone resolve and reject to use inside closures
        let resolve_clone = resolve.clone();
        let reject_clone = reject.clone();

        // Handle incoming messages
        let onmessage_callback = Closure::wrap(Box::new(move |evt: MessageEvent| {
            if let Some(txt) = evt.data().as_string() {
                console_log!("Rust: Received message: {}", txt);
                resolve_clone.call1(&JsValue::NULL, &JsValue::from(txt))
                    .expect("Rust: Failed to resolve promise");
            } else {
                console_err!("Rust: Received non-text message");
                reject_clone.call1(&JsValue::NULL, &JsValue::from("Rust: Received non-text message"))
                    .expect("Rust: Failed to reject promise");
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Clone resolve and reject again for the error handler
        let reject_clone = reject.clone();

        // Handle errors
        let onerror_callback = Closure::wrap(Box::new(move |ev: ErrorEvent| {
            console_err!("Rust: WebSocket error: {}", ev.message());
            reject_clone.call1(&JsValue::NULL, &JsValue::from(ev.message()))
                .expect("Rust: Failed to reject promise");
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        // Clone resolve and reject for connection open
        let reject_clone = reject.clone();
        let ws_clone = ws.clone();
        let message = message.to_string();
        let onopen_callback = Closure::wrap(Box::new(move || {
            console_log!("Rust: WebSocket connection opened");
            if let Err(err) = ws_clone.send_with_str(&message) {
                console_err!("Rust: Failed to send message: {:?}", err);
                reject_clone.call1(&JsValue::NULL, &JsValue::from("Rust: Failed to send message"))
                    .expect("Rust: Failed to reject promise");
            } else {
                console_log!("Rust: Message sent: {}", message);
            }
        }) as Box<dyn FnMut()>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    })
}
