use dioxus::prelude::*;
use futures::StreamExt;
use solana_sdk::pubkey::Pubkey;

use crate::gateway::{AccountNotificationParams, GatewayResult};

use super::{next_sub_request_id, use_notif_store, use_wss, ToWssMsg};

/// End to end management of websocket subscriptions.
///
/// We have exactly one wss connection and all notifications come thru the same channel.
/// This means we need to differentiate between subscribers, and route the correct notifs to
/// the respective subscribing components.
///
/// This hook manages
/// 1) Creating new subscriptions
/// 2) Routing notifications
/// 3) Closing subscriptions when the parent component unmounts
pub fn use_wss_subscription<T, U>(
    mut data: Signal<GatewayResult<T>>,
    update_callback: U,
) -> Coroutine<Pubkey>
where
    T: Clone + 'static,
    U: Fn(&AccountNotificationParams) -> GatewayResult<T> + 'static,
{
    let (from_wss, to_wss) = use_wss();
    let (mut notif_store, notif_version) = use_notif_store();
    let mut sub_id = use_signal(|| 0u64);
    let mut sub_request_id = use_signal(|| 0u32);
    let mut last_processed_version = use_signal(|| 0u64);

    // Subscribe when component mounts
    let pubkey_tx = use_coroutine(move |mut rx: UnboundedReceiver<Pubkey>| async move {
        while let Some(pubkey) = rx.next().await {
            let rid = next_sub_request_id();
            // Set sub request id
            sub_request_id.set(rid);
            // Unsubscribe from previous wallet first
            let current_sub_id = sub_id();
            if current_sub_id > 0 {
                to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
                // Clean up stale notification and reset sub_id so the data effect
                // won't replay old data while waiting for the new subscription ID
                notif_store.write().remove(&current_sub_id);
                sub_id.set(0);
            }
            // Then subscribe to new wallet
            to_wss.send(ToWssMsg::Subscribe(rid, pubkey));
        }
    });

    // Handle subscription ID tracking
    use_effect(move || {
        let map = from_wss();
        let my_rid = sub_request_id();
        if my_rid == 0 {
            return;
        }
        if let Some(&sid) = map.get(&my_rid) {
            sub_id.set(sid);
        }
    });

    // Handle data updates from the per-subscription notification store
    use_effect(move || {
        let current_version = notif_version();

        // Skip if no new notifications since last process
        if current_version <= last_processed_version() {
            return;
        }

        let my_sub_id = sub_id();
        if my_sub_id == 0 {
            return; // Not yet subscribed
        }

        // Read our subscription's notification from the store
        if let Some(notif) = notif_store.read().get(&my_sub_id) {
            data.set(update_callback(notif));
        }

        // Mark as processed
        last_processed_version.set(current_version);
    });

    // Unsubscribe and clean up when component is dropped
    crate::dioxus_core::use_drop(move || {
        let current_sub_id = sub_id();
        if current_sub_id > 0 {
            to_wss.send(ToWssMsg::Unsubscribe(current_sub_id));
            // Clean up our entry from the notification store
            notif_store.write().remove(&current_sub_id);
        }
    });

    pubkey_tx
}
