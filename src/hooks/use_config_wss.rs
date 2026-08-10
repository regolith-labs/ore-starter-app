use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::Config;
use steel::*;

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::use_wss_subscription;

pub(crate) fn use_config_wss_provider() {
    let signal = use_config_signal();
    use_context_provider(|| signal);
}

fn use_config_signal() -> Signal<GatewayResult<Config>> {
    let clock_address = ore_api::state::config_pda().0;

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
        let config =
            *Config::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(config)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(clock_address));

    data
}

pub fn _use_config_wss() -> Signal<GatewayResult<Config>> {
    use_context()
}
