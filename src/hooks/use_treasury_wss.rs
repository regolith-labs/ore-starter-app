use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::Treasury;
use steel::*;

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::use_wss_subscription;

pub(crate) fn use_treasury_wss_provider() {
    let signal = use_treasury_signal();
    use_context_provider(|| signal);
}

fn use_treasury_signal() -> Signal<GatewayResult<Treasury>> {
    let treasury_address = ore_api::state::treasury_pda().0;

    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));

    // Update
    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the market account data
        let treasury =
            *Treasury::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(treasury)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(treasury_address));

    data
}

pub fn use_treasury_wss() -> Signal<GatewayResult<Treasury>> {
    use_context()
}
