use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::Miner;
use steel::*;

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_wallet, use_wss_subscription, Wallet};

pub(crate) fn use_miner_wss_provider() {
    let signal = use_miner_signal();
    use_context_provider(|| signal);
}

pub fn use_miner_signal() -> Signal<GatewayResult<Miner>> {
    let wallet = use_wallet();
    let miner_address = use_memo(move || match wallet() {
        Wallet::Connected(address) => ore_api::state::miner_pda(address).0,
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

        // Unpack the market account data
        let miner = *Miner::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(miner)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(miner_address()));

    data
}

pub fn use_miner_wss() -> Signal<GatewayResult<Miner>> {
    use_context()
}
