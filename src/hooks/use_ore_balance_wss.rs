use base64::{prelude::BASE64_STANDARD, Engine};
use dioxus::prelude::*;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

use crate::gateway::AccountNotificationParams;
use crate::gateway::{GatewayError, GatewayResult};
use crate::hooks::{use_wallet, use_wss_subscription, Wallet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OreBalance(pub u64);

pub(crate) fn use_ore_balance_wss_provider() {
    let ore_balance = use_ore_balance_signal();
    use_context_provider(|| ore_balance);
}

fn use_ore_balance_signal() -> Signal<GatewayResult<OreBalance>> {
    let wallet = use_wallet();
    let wallet_address = use_memo(move || match wallet() {
        Wallet::Connected(address) => address,
        Wallet::Disconnected => Pubkey::default(),
    });
    let ata_address = use_memo(move || {
        get_associated_token_address(&wallet_address(), &ore_api::consts::MINT_ADDRESS)
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
        let ore_balance = spl_token::state::Account::unpack(data.as_slice())
            .map_err(|err| anyhow::anyhow!(err))?;

        Ok(OreBalance(ore_balance.amount))
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(ata_address()));

    data
}

pub fn use_ore_balance_wss() -> Signal<GatewayResult<OreBalance>> {
    use_context()
}
