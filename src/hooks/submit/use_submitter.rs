use dioxus::prelude::*;
use futures::StreamExt;
use solana_sdk::transaction::VersionedTransaction;

use crate::hooks::use_transaction_status;

use super::submit_transaction;

///
/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.

pub fn use_transaction_submitter() -> Coroutine<VersionedTransaction> {
    use_coroutine_handle::<VersionedTransaction>()
}

/// Two way channel backed by a WebSocket
/// for subscribing to notifications from the RPC server.
pub fn use_transaction_submitter_provider() {
    let transaction_status = use_transaction_status();

    let _to_submitter = use_coroutine(
        move |mut rx: UnboundedReceiver<VersionedTransaction>| async move {
            while let Some(tx) = rx.next().await {
                submit_transaction(tx, transaction_status.clone());
            }
        },
    );
}
