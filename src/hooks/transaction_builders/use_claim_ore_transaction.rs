use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_miner_wss, use_wallet, Wallet},
};

/// Builds a claim ORE transaction with a variable bps (basis points) amount.
/// bps = 10000 means 100%, bps = 5000 means 50%, etc.
pub fn use_claim_ore_transaction(
    bps: Memo<u64>,
    uncheckpointed_rewards: Memo<(u64, u64)>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let miner = use_miner_wss();

    use_resource(move || async move {
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        let Ok(miner) = miner() else {
            return Err(GatewayError::Unknown);
        };

        let bps = bps();
        if bps == 0 {
            return Err(GatewayError::Unknown);
        }

        let rewards_ore = miner.rewards_ore + miner.refined_ore + uncheckpointed_rewards().1;
        if rewards_ore == 0 {
            return Err(GatewayError::Unknown);
        }

        let mut ixs = vec![];
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));
        ixs.push(ore_api::sdk::checkpoint(
            authority,
            authority,
            miner.round_id,
        ));
        ixs.push(ore_api::sdk::claim_ore(authority, bps));
        ixs.push(tip_ix(&authority));

        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
