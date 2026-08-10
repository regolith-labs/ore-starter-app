use dioxus::prelude::*;

use steel::*;

use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_wallet, use_wss_subscription, Wallet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolBalance(pub u64);

impl Default for SolBalance {
    fn default() -> Self {
        Self(0)
    }
}

pub(crate) fn use_sol_balance_wss_provider() {
    let signal = use_sol_balance_signal();
    use_context_provider(|| signal);
}

fn use_sol_balance_signal() -> Signal<GatewayResult<SolBalance>> {
    let wallet = use_wallet();
    let address = use_memo(move || match wallet() {
        Wallet::Connected(address) => address,
        Wallet::Disconnected => Pubkey::default(),
    });

    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));

    // Update
    let update_callback =
        move |notif: &AccountNotificationParams| Ok(SolBalance(notif.result.value.lamports));

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(address()));

    data
}

pub fn use_sol_balance_wss() -> Signal<GatewayResult<SolBalance>> {
    use_context()
}
