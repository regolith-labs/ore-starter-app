use dioxus::prelude::*;
use ore_api::state::AutomationConditions;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    transaction::{Transaction, VersionedTransaction},
};
use steel::Pubkey;

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_wallet, Wallet},
};

pub fn use_cancel_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        // Push cancel instruction
        ixs.push(ore_api::sdk::automate(
            authority,
            0,
            0,
            Pubkey::default(),
            0,
            0,
            0,
            false,
            AutomationConditions::default(),
        ));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
