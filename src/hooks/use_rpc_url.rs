use dioxus::prelude::*;
use web_sys::window;

use crate::gateway::DEFAULT_RPC_URL;

const STORAGE_KEY: &str = "rpc_url";

pub const PRESET_RPCS: &[(&str, &str)] = &[("Default", DEFAULT_RPC_URL)];

fn get_local_storage_item(key: &str) -> Option<String> {
    window()?.local_storage().ok()??.get_item(key).ok()?
}

fn set_local_storage_item(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        let _ = storage.set_item(key, value);
    }
}

pub fn use_rpc_url_provider() {
    let saved = get_local_storage_item(STORAGE_KEY).unwrap_or_else(|| DEFAULT_RPC_URL.to_string());
    use_context_provider(|| Signal::new(saved));
}

pub fn use_rpc_url() -> Signal<String> {
    use_context::<Signal<String>>()
}

/// Derive WSS URL from HTTP URL (https→wss, http→ws)
pub fn rpc_to_wss_url(rpc: &str) -> String {
    if rpc.starts_with("https://") {
        format!("wss://{}", &rpc["https://".len()..])
    } else if rpc.starts_with("http://") {
        format!("ws://{}", &rpc["http://".len()..])
    } else {
        rpc.to_string()
    }
}

/// Detect which preset matches a given URL, returns the label or "Custom"
pub fn detect_preset(url: &str) -> String {
    for (label, preset_url) in PRESET_RPCS {
        if url == *preset_url {
            return label.to_string();
        }
    }
    "Custom".to_string()
}

/// Save RPC URL to localStorage and update the signal
pub fn save_rpc_url(mut rpc_url: Signal<String>, url: String) {
    set_local_storage_item(STORAGE_KEY, &url);
    spawn(async move {
        rpc_url.set(url);
    });
}
