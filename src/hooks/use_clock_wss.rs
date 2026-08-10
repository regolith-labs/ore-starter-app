use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use steel::{sysvar, Clock};

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::use_wss_subscription;

pub(crate) fn use_clock_wss_provider() {
    let signal = use_clock_signal();
    use_context_provider(|| signal);
}

fn use_clock_signal() -> Signal<GatewayResult<Clock>> {
    let clock_address = sysvar::clock::ID;

    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));

    // Update
    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the proof account data
        let clock = bincode::deserialize(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(clock)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(clock_address));

    data
}

pub fn use_clock_wss() -> Signal<GatewayResult<Clock>> {
    use_context()
}
