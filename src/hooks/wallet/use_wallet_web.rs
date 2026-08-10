use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::hooks::Wallet;

pub fn use_wallet_provider() {
    let mut wallet_signal = use_context_provider(|| Signal::new(Wallet::Disconnected));

    let mut eval = eval(
        r#"
            window.addEventListener("wallet-pubkey", (event) => {
                dioxus.send({
                    pubkey: event.detail.pubkey
                });
            });
        "#,
    );

    let _ = use_resource(move || async move {
        while let Ok(json_val) = eval.recv::<serde_json::Value>().await {
            log::info!("Wallet event: {}", json_val);

            let pubkey_val = json_val.get("pubkey").cloned().unwrap_or(serde_json::Value::Null);
            let pubkey_result: Result<Pubkey, serde_json::Error> = serde_json::from_value(pubkey_val);

            match pubkey_result {
                Ok(pubkey) => {
                    spawn(async move {
                        wallet_signal.set(Wallet::Connected(pubkey));
                    });
                }
                Err(_) => {
                    spawn(async move {
                        wallet_signal.set(Wallet::Disconnected);
                    });
                }
            }
        }
    });
}
