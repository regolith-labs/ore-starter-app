use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_miner_wss, use_wallet, Wallet},
};

pub fn use_claim_sol_transaction(
    uncheckpointed_rewards: Memo<(u64, u64)>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let miner = use_miner_wss();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get miner
        let Ok(miner) = miner() else {
            return Err(GatewayError::Unknown);
        };

        // Get non-checkpointed rewards
        let rewards_sol = miner.rewards_sol + uncheckpointed_rewards().0;

        // Exit if no rewards
        if rewards_sol == 0 {
            return Err(GatewayError::Unknown);
        }

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        // Push checkpoint instruction
        ixs.push(ore_api::sdk::checkpoint(
            authority,
            authority,
            miner.round_id,
        ));

        // Push claim instruction
        ixs.push(ore_api::sdk::claim_sol(authority));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
