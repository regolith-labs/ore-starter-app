use crate::components::common::{Splice, WebClipboard};
use crate::components::{Col, CopyIcon, Row};
use crate::hooks::{
    detect_preset, save_rpc_url, use_rpc_url, use_wallet, ToWssMsg, Wallet,
    PRESET_RPCS,
};
use dioxus::document::eval;
use dioxus::prelude::*;
use std::str::FromStr;

#[component]
pub fn WalletDrawer(on_close: EventHandler<MouseEvent>, wallet_remount: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-screen sm:w-96 elevated sm:border-l sm:border-gray-800 text-white z-50 relative shadow-xl",
            onclick: move |e| e.stop_propagation(),

            // Close button
            Row {
                class: "justify-start px-2 pt-4",
                gap: 2,
                CloseButton { on_close }
            }

            // Header section
            div {
                class: "px-4 pb-2",
                Col {
                    gap: 2,
                    class: "mt-4",
                    Address {}
                }
            }

            // Action buttons
            Col {
                class: "mt-auto px-4 py-4 mb-4",
                gap: 3,
                RpcEndpointSetting {}
                DisconnectButton { wallet_remount }
            }
        }
    }
}

#[component]
fn Address() -> Element {
    let wallet = use_wallet();
    let clipboard = WebClipboard::new();
    let mut pubkey = use_signal(|| "missing pubkey".to_string());
    let mut pubkey_splice = use_signal(|| Splice::Pubkey("0000...0000".to_string()));
    let mut pubkey_copied = use_signal(|| false);

    use_effect(move || {
        if let Wallet::Connected(pk) = wallet() {
            let pk = pk.to_string();
            pubkey.set(pk.clone());
            if let Ok(splice) = Splice::from_str(pk.as_str()) {
                pubkey_splice.set(splice);
            }
        }
    });

    use_effect(move || {
        if let Splice::Copied = pubkey_splice() {
            spawn(async move {
                async_std::task::sleep(std::time::Duration::from_millis(1500)).await;
                let pk = pubkey.read();
                if let Ok(splice) = Splice::from_str(pk.as_str()) {
                    pubkey_splice.set(splice);
                }
            });
        }
    });

    use_effect(move || {
        if pubkey_copied() {
            spawn(async move {
                async_std::task::sleep(std::time::Duration::from_millis(1500)).await;
                pubkey_copied.set(false);
            });
        }
    });

    rsx! {
        Row {
            class: "justify-between items-center",
            span {
                class: "text-elements-lowEmphasis font-medium",
                "Address"
            }
            Row {
                gap: 2,
                span {
                    class: "text-elements-highEmphasis",
                    "{pubkey_splice.read().to_string()}"
                }
                button {
                    class: "flex items-center justify-center w-6 h-6 rounded text-elements-lowEmphasis hover:bg-controls-secondaryHover hover:text-elements-highEmphasis cursor-pointer",
                    onclick: move |e| {
                        e.stop_propagation();
                        if let Err(err) = clipboard.set(pubkey.to_string()) {
                            log::error!("failed to set clipboard: {:?}", err);
                        }
                        pubkey_copied.set(true);
                    },
                    CopyIcon {
                        class: "h-4 w-4",
                        solid: pubkey_copied()
                    }
                }
            }
        }
    }
}

#[component]
fn CloseButton(on_close: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "rounded-full text-center w-8 h-8 flex items-center justify-center hover:bg-controls-secondaryHover self-center cursor-pointer",
            onclick: move |e| {
                e.stop_propagation();
                on_close.call(e);
            },
            span {
                class: "text-xl font-semibold",
                "×"
            }
        }
    }
}

#[component]
fn DisconnectButton(wallet_remount: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "w-full rounded-full text-center h-12 px-6 controls-secondary hover:cursor-pointer hover:text-elements-highEmphasis",
            onclick: move |_| {
                wallet_remount.set(true);
                let disconnect = eval(r#"window.WalletDisconnect(); return"#);
                spawn(async move {
                    let _ = disconnect.await;
                });
            },
            span {
                class: "mx-auto my-auto",
                "Disconnect"
            }
        }
    }
}

#[component]
fn RpcEndpointSetting() -> Element {
    let rpc_url = use_rpc_url();
    let to_wss = use_coroutine_handle::<ToWssMsg>();

    let current_preset = detect_preset(&rpc_url());
    let is_custom = current_preset == "Custom";

    let mut show_custom_input = use_signal(|| is_custom);
    let mut custom_url_input = use_signal(|| if is_custom { rpc_url() } else { String::new() });
    let mut validation_error = use_signal(|| Option::<String>::None);

    let apply_url = move |url: String| {
        save_rpc_url(rpc_url, url);
        to_wss.send(ToWssMsg::Reconnect);
    };

    let has_changes =
        show_custom_input() && custom_url_input() != rpc_url() && !custom_url_input().is_empty();

    let save_btn_class = if has_changes {
        "px-3 py-2 rounded-lg text-sm font-semibold bg-white text-black hover:bg-gray-200 transition-colors cursor-pointer"
    } else {
        "px-3 py-2 rounded-lg text-sm font-medium controls-secondary text-elements-lowEmphasis transition-colors cursor-default"
    };

    rsx! {
        Col {
            gap: 2,
            class: "w-full mb-4",
            Row {
                gap: 2,
                class: "justify-between w-full",
                span {
                    class: "text-elements-lowEmphasis my-auto font-medium",
                    "RPC"
                }
                select {
                    class: "bg-transparent text-elements-highEmphasis font-medium text-right cursor-pointer outline-none",
                    value: if is_custom { "Custom" } else { current_preset.clone() },
                    onchange: move |e: Event<FormData>| {
                        let val = e.value();
                        if val == "Custom" {
                            show_custom_input.set(true);
                        } else {
                            show_custom_input.set(false);
                            validation_error.set(None);
                            if let Some((_, url)) = PRESET_RPCS.iter().find(|(label, _)| *label == val.as_str()) {
                                apply_url(url.to_string());
                            }
                        }
                    },
                    for (label, _url) in PRESET_RPCS.iter() {
                        option {
                            value: *label,
                            selected: current_preset == *label,
                            "{label}"
                        }
                    }
                    option {
                        value: "Custom",
                        selected: is_custom,
                        "Custom"
                    }
                }
            }
            if show_custom_input() {
                Col {
                    gap: 2,
                    class: "w-full",
                    Row {
                        gap: 2,
                        class: "w-full",
                        input {
                            class: "flex-1 bg-surface-floating text-elements-highEmphasis text-sm rounded-lg px-3 py-2 outline-none border border-gray-700 focus:border-gray-500",
                            r#type: "text",
                            placeholder: "https://...",
                            value: "{custom_url_input}",
                            oninput: move |e: Event<FormData>| {
                                custom_url_input.set(e.value());
                                validation_error.set(None);
                            },
                        }
                        button {
                            class: "{save_btn_class}",
                            disabled: !has_changes,
                            onclick: move |_| {
                                let url = custom_url_input();
                                if !url.starts_with("https://") && !url.starts_with("http://") {
                                    validation_error.set(Some("URL must start with https://".to_string()));
                                    return;
                                }
                                validation_error.set(None);
                                apply_url(url);
                            },
                            "Save"
                        }
                    }
                    if let Some(err) = validation_error() {
                        span {
                            class: "text-red-400 text-xs",
                            "{err}"
                        }
                    }
                }
            }
        }
    }
}
