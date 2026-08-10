use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::Automation;
use steel::*;

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_wallet, use_wss_subscription, Wallet};

pub(crate) fn use_automation_wss_provider() {
    let signal = use_automation_signal();
    use_context_provider(|| signal);
}

pub fn use_automation_signal() -> Signal<GatewayResult<Automation>> {
    let wallet = use_wallet();
    let automation_address = use_memo(move || match wallet() {
        Wallet::Connected(address) => ore_api::state::automation_pda(address).0,
        Wallet::Disconnected => Pubkey::default(),
    });

    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));

    // Update
    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Check size
        if data.is_empty() {
            return Err(GatewayError::AccountNotFound);
        }

        // Unpack the automation account data
        let automation =
            *Automation::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(automation)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(automation_address()));

    data
}

pub fn use_automation_wss() -> Signal<GatewayResult<Automation>> {
    use_context()
}
