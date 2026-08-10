use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use super::wallet_picker_web::WalletPickerModal;
use crate::components::*;
use crate::hooks::{use_wallet, use_wallet_drawer_state, Wallet};
use crate::utils::Duration;

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();
    let mut mounted = use_signal(|| false);
    let mut drawer_state = use_wallet_drawer_state();

    // Reset drawer state when wallet disconnects
    use_effect(move || {
        if wallet.cloned() == Wallet::Disconnected {
            drawer_state.write().0 = false;
        }
    });

    let mut wallet_mount = use_resource(move || async move {
        if mounted() {
            return;
        }
        if Wallet::Disconnected == wallet.cloned() {
            loop {
                async_std::task::sleep(Duration::from_millis(500)).await;
                let eval = eval(
                    r#"
                        if (typeof window.MountWalletAdapter === 'function') {
                            window.MountWalletAdapter();
                            return 0;
                        } else {
                            return 1;
                        }
                    "#,
                );
                if let Ok(res) = eval.await {
                    if res.as_u64() == Some(0) {
                        log::info!("Wallet adapter mounted");
                        mounted.set(true);
                        break;
                    } else {
                        log::warn!("Wallet adapter not mounted");
                    }
                }
            }
        }
    });

    let mut wallet_remount = use_signal(|| false);

    // Always render ore-wallet-adapter so the wallet bridge stays mounted.
    if *wallet_remount.peek() {
        wallet_mount.restart();
        wallet_remount.set(false);
    }

    rsx! {
        // Hidden container that keeps the wallet bridge mounted
        nav {
            id: "ore-wallet-adapter",
            style: "display: none;"
        }

        if let Wallet::Connected(address) = wallet.cloned() {
            ConnectedWalletAdapter {
                address: address,
                wallet_remount
            }
        } else {
            ConnectButton {}
        }
    }
}

fn ConnectButton() -> Element {
    let mut show_modal = use_signal(|| false);
    rsx! {
        if show_modal() {
            WalletPickerModal {
                show_modal: show_modal
            }
        }
        button {
            class: "my-auto h-10 px-5 controls-primary rounded-full hover:cursor-pointer hover:scale-105 duration-300 ease-in-out",
            onclick: move |_| async move {
                show_modal.set(true);
            },
            span {
                class: "mx-auto my-auto font-semibold",
                "Connect"
            }
        }
    }
}

#[component]
fn ConnectedWalletAdapter(address: Pubkey, wallet_remount: Signal<bool>) -> Element {
    let len = address.to_string().len();
    let first_four = &address.to_string()[0..4];
    let last_four = &address.to_string()[len - 4..len];

    let mut drawer_state = use_wallet_drawer_state();
    let _is_open = drawer_state.read().0;
    let mut is_animating = use_signal(|| false);

    // Close function that handles animation
    let close_drawer = move |_e: MouseEvent| {
        if !is_animating() {
            is_animating.set(true);
            drawer_state.write().0 = false;
            spawn(async move {
                // Keep the animation state active during the transition
                async_std::task::sleep(crate::utils::Duration::from_millis(300)).await;
                is_animating.set(false);
            });
        }
    };

    rsx! {
        div {
            class: "relative",
            button {
                onclick: move |_| {
                    if !is_animating.cloned() {
                        is_animating.set(true);
                        drawer_state.write().0 = true;
                        spawn(async move {
                            async_std::task::sleep(crate::utils::Duration::from_millis(300)).await;
                            is_animating.set(false);
                        });
                    }
                },
                Row {
                    class: "elevated-control elevated-border rounded-full text-sm font-semibold h-10 px-5 hover:cursor-pointer gap-3",
                    gap: 3,
                    span {
                        class: "mx-auto my-auto",
                        "{first_four}...{last_four}"
                    }
                    // DrawerIcon {
                    //     class: "w-3 text-elements-lowEmphasis"
                    // }
                }
            }

            if drawer_state().0 || is_animating() {
                WalletDrawerOverlay {
                    is_open: drawer_state().0,
                    on_close: close_drawer,
                    wallet_remount: wallet_remount
                }
            }
        }
    }
}

#[component]
fn WalletDrawerOverlay(
    is_open: bool,
    on_close: EventHandler<MouseEvent>,
    wallet_remount: Signal<bool>,
) -> Element {
    // Render drawer always, but with proper animation classes
    rsx! {
        Fragment {
            // Background overlay
            div {
                class: "fixed inset-0 transition-all duration-300 ease-in-out bg-black/50 backdrop-blur-sm z-[1000]",
                class: if is_open { "wallet-drawer-fade opacity-100" } else { "wallet-drawer-fade-out opacity-0" },
                style: "height: 100vh; width: 100vw;",
                onclick: move |e| on_close.call(e)
            }

            // Drawer content with slide animation
            div {
                class: "fixed top-0 right-0 h-full w-screen sm:w-96 transition-transform duration-300 ease-in-out transform z-[1001]",
                class: if is_open { "wallet-drawer-slide translate-x-0" } else { "wallet-drawer-slide-out translate-x-full" },
                style: "height: 100vh;",
                WalletDrawer {
                    on_close: on_close.clone(),
                    wallet_remount: wallet_remount
                }
            }
        }
    }
}
