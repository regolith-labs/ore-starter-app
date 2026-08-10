use crate::components::*;
use dioxus::{document::eval, prelude::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Wallet {
    pub name: String,
    pub icon: String,
}

#[component]
pub fn WalletPickerModal(mut show_modal: Signal<bool>) -> Element {
    // Get detected wallets from JavaScript
    let wallets = use_resource(move || async move {
        let mut eval = eval(
            r#"
                if (typeof window.WalletAdapters === 'object') {
                    dioxus.send(window.WalletAdapters);
                } else {
                    dioxus.send([]);
                }
            "#,
        );

        match eval.recv().await {
            Ok(serde_json::Value::Array(wallets)) => {
                let mut wallet_list: Vec<Wallet> = wallets
                    .iter()
                    .filter_map(|wallet| match wallet {
                        serde_json::Value::Object(wallet) => {
                            if let Some(serde_json::Value::String(icon)) = wallet.get("icon") {
                                if let Some(serde_json::Value::String(name)) = wallet.get("name") {
                                    Some(Wallet {
                                        name: name.clone(),
                                        icon: icon.clone(),
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect();
                wallet_list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                wallet_list
            }
            _ => {
                vec![]
            }
        }
    });

    rsx! {
        Fragment {
            // Background overlay
            div {
                class: "fixed inset-0 transition-all duration-300 ease-in-out bg-black/50 backdrop-blur-sm z-[1000]",
                class: if show_modal() { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                style: "height: 100vh; width: 100vw;",
                onclick: move |_| show_modal.set(false),
            }

            div {
                class: "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-11/12 sm:w-96 max-w-md z-[1001]",
                class: if show_modal() { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                Col {
                    class: "bg-surface-elevated elevated-border rounded-md pt-6 pb-6",
                    gap: 4,
                    Col {
                        class: "w-full px-6",
                        gap: 2,
                        Row {
                            class: "justify-between items-center",
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl",
                                "Connect"
                            }
                        }
                        span {
                            class: "text-elements-midEmphasis font-medium",
                            "Select a wallet to sign in."
                        }
                    }

                    // Wallet list
                    if let Some(wallets) = wallets() {
                        if !wallets.is_empty() {
                            Col {
                                class: "w-full",
                                for wallet in wallets {
                                    WalletButton {
                                        wallet: wallet,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WalletButton(wallet: Wallet) -> Element {
    rsx! {
        button {
            class: "w-full flex flex-row justify-between gap-4 px-6 h-12 hover:cursor-pointer hover:bg-controls-secondaryHover",
            onclick: move |_| {
                let name = wallet.name.clone();
                async move {
                    let eval = eval(
                        r#"
                        let name = await dioxus.recv();
                        await window.WalletConnect(name);
                        "#,
                    );
                    match eval.send(serde_json::Value::String(name)) {
                        Ok(()) => {
                            // Connection initiated
                        }
                        Err(e) => {
                            log::error!("Failed to select wallet: {:?}", e);
                        }
                    }
                }
            },
            Row {
                gap: 2,
                img {
                    src: "{wallet.icon}",
                    class: "w-6 h-6 my-auto"
                }
                span {
                    class: "my-auto font-semibold",
                    "{wallet.name}"
                }
            }
            ChevronRightIcon {
                class: "w-5 h-5 my-auto text-elements-lowEmphasis"
            }
        }
    }
}
