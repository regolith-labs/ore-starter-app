// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
mod components;
mod gateway;
mod hooks;
mod pages;
mod route;
mod utils;

use dioxus::prelude::*;
use tracing::Level;

use crate::hooks::{
    use_automation_wss_provider, use_board_wss_provider, use_clock_wss_provider,
    use_config_wss_provider, use_global_batch_init, use_last_round_wss_provider,
    use_miner_wss_provider, use_ore_balance_wss_provider, use_round_wss_provider,
    use_rpc_url, use_rpc_url_provider, use_sol_balance_wss_provider,
    use_transaction_status_provider, use_transaction_submitter_provider, use_treasury_wss_provider,
    use_user_batch_init, use_wallet_drawer_state_provider, use_wss_provider,
};
use crate::route::Route;

use crate::hooks::use_wallet_provider;

const CSS: &str = include_str!("../assets/tailwind.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::logger::init(Level::DEBUG).expect("failed to init logger");
    dioxus::launch(App)
}

#[component]
fn App() -> Element {
    use_transaction_status_provider();
    use_transaction_submitter_provider();
    use_wallet_provider();
    use_wallet_drawer_state_provider();
    use_rpc_url_provider();
    let rpc_url = use_rpc_url();
    use_wss_provider(rpc_url);
    use_sol_balance_wss_provider();
    use_ore_balance_wss_provider();
    use_clock_wss_provider();
    use_treasury_wss_provider();
    use_config_wss_provider();
    use_board_wss_provider();
    use_round_wss_provider();
    use_last_round_wss_provider();
    use_miner_wss_provider();
    use_automation_wss_provider();
    use_global_batch_init();
    use_user_batch_init();

    rsx! {
        // Tailwind CSS
        style { "{CSS}" }

        // Wallet adapter scripts are loaded from index.html to guarantee load order.
        // These prefetch links ensure the Dioxus dev server knows about the files.
        document::Link { rel: "prefetch", href: asset!("/assets/vendor/spl-memo.global.js", AssetOptions::builder().with_hash_suffix(false)) }
        document::Link { rel: "prefetch", href: asset!("/assets/vendor/spl-system.global.js", AssetOptions::builder().with_hash_suffix(false)) }
        document::Link { rel: "prefetch", href: asset!("/assets/vendor/spl-token.global.js", AssetOptions::builder().with_hash_suffix(false)) }
        document::Link { rel: "prefetch", href: asset!("/assets/wallet.js", AssetOptions::builder().with_hash_suffix(false)) }

        // Favicon
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        document::Link { rel: "icon", href: asset!("/assets/icon.png") }

        // Router
        Router::<Route> {}
    }
}
