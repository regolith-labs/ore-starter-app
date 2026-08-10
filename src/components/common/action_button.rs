use dioxus::prelude::*;
use solana_sdk::transaction::VersionedTransaction;

use crate::components::Col;
use crate::gateway::{GatewayError, GatewayResult};
use crate::hooks::use_transaction_submitter;

#[component]
pub fn ActionButton(
    class: Option<String>,
    transaction: Resource<GatewayResult<VersionedTransaction>>,
    children: Element,
) -> Element {
    let class = class.unwrap_or("controls-primary".to_string());
    let submitter = use_transaction_submitter();

    let enabled = if let Some(Ok(_)) = transaction() {
        true
    } else {
        false
    };

    let error_text = match transaction() {
        Some(Err(GatewayError::InsufficientSOL)) => Some("Insufficient SOL"),
        Some(Err(GatewayError::AmountTooSmall)) => None,
        Some(Err(GatewayError::NoTerritorySelected)) => Some("Select a tile"),
        _ => None,
    };

    rsx! {
        Col {
            class: "w-full",
            gap: 4,
            button {
                class: "flex h-12 w-full rounded-full {class} transition hover:not-disabled:scale-105",
                disabled: !enabled,
                onclick: move |_| {
                    if let Some(Ok(transaction)) = transaction.cloned() {
                        submitter.send(transaction);
                    }
                },
                div {
                    class: "mx-auto my-auto font-semibold flex items-center gap-1.5",
                    if let Some(err) = error_text {
                        span { "{err}" }
                    } else {
                        {children}
                    }
                }
            }
        }
    }
}
