use dioxus::prelude::*;
use ore_api::state::Automation;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    native_token::sol_to_lamports,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_sol_balance_wss, use_wallet, Wallet},
};

pub fn use_topup_transaction(
    amount: Signal<String>,
    automation: Signal<GatewayResult<Automation>>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_sol_balance_wss();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get the automation
        let Ok(automation) = automation() else {
            return Err(GatewayError::Unknown);
        };

        // Get amount
        let sol_amount_f64 = amount().parse::<f64>().map_err(|_| GatewayError::Unknown)?;
        let deposit_amount = sol_to_lamports(sol_amount_f64);

        // Get sol balance
        let Ok(sol_balance) = sol_balance() else {
            return Err(GatewayError::Unknown);
        };

        // Returns false if sol balance is insufficient
        if deposit_amount > sol_balance.0 {
            return Err(GatewayError::InsufficientSOL);
        }

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        // Push automate instruction
        ixs.push(ore_api::sdk::automate(
            authority,
            automation.amount,
            deposit_amount,
            automation.executor,
            automation.fee,
            automation.mask,
            automation.strategy as u8,
            automation.reload > 0,
            automation.conditions,
        ));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
