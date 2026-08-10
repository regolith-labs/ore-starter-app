use base64::Engine;
use dioxus::prelude::*;

use base64::prelude::BASE64_STANDARD;
use ore_api::state::Round;
use steel::*;

use crate::gateway::ore::OreGateway;
use crate::gateway::{AccountNotificationParams, GatewayError, GatewayResult};
use crate::hooks::{use_board_wss, use_gateway, use_wss_subscription};

pub(crate) fn use_round_wss_provider() {
    let signal = use_round_signal();
    use_context_provider(|| signal);
}

fn use_round_signal() -> Signal<GatewayResult<Round>> {
    let board = use_board_wss();
    let round_id = use_memo(move || {
        let Ok(board) = board() else {
            return 0;
        };
        board.round_id
    });
    let round_address = use_memo(move || ore_api::state::round_pda(round_id()).0);

    let mut data = use_signal(|| Err(GatewayError::AccountNotFound));
    let _ = use_resource(move || async move {
        let id = round_id();
        if id == 0 {
            return;
        }
        match use_gateway().rpc.get_round(id).await {
            Ok(round) => data.set(Ok(round)),
            Err(err) => {
                log::error!("Failed to initialize round: {:?}", err);
                data.set(Err(err));
            }
        }
    });

    // Update
    let update_callback = move |notif: &AccountNotificationParams| {
        // Base64 decode
        let data = &notif.result.value.data;
        let data = data.first().ok_or(GatewayError::AccountNotFound)?;
        let data = BASE64_STANDARD
            .decode(data.clone())
            .map_err(|err| anyhow::anyhow!(err))?;

        // Unpack the market account data
        let round = *Round::try_from_bytes(data.as_slice()).map_err(|err| anyhow::anyhow!(err))?;
        Ok(round)
    };

    // Subscribe
    let subscriber = use_wss_subscription(data.clone(), update_callback.clone());
    use_effect(move || subscriber.send(round_address()));

    data
}

pub fn use_round_wss() -> Signal<GatewayResult<Round>> {
    use_context()
}
