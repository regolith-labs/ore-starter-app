use dioxus::prelude::*;
use ore_api::{
    consts::EXECUTOR_ADDRESS,
    state::{AutomationConditions, AutomationStrategy},
};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    native_token::sol_to_lamports,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_sol_balance_wss, use_wallet, Wallet, JITO_TIP_AMOUNT},
};

pub const AUTOMATION_FEE: u64 = 5000 + JITO_TIP_AMOUNT;

pub fn use_automate_transaction(
    amount: Signal<String>,
    num_rounds: Memo<u64>,
    num_squares: Memo<u64>,
    selected_squares: Signal<[bool; 25]>,
    auto_reload: Signal<bool>,
    randomize_tiles: Signal<bool>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_sol_balance_wss();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get amount
        let sol_amount_f64 = amount().parse::<f64>().map_err(|_| GatewayError::Unknown)?;
        let lamports = sol_to_lamports(sol_amount_f64);

        // Count number of squares selected
        let selected_squares_count = selected_squares().iter().filter(|&&x| x).count();

        // Get number of squares
        let num_squares = num_squares() as u64;
        if num_squares == 0 {
            return Err(GatewayError::Unknown);
        }
        if num_squares > 25 {
            return Err(GatewayError::Unknown);
        }

        // Get number of rounds
        let num_rounds = num_rounds();
        if num_rounds == 0 {
            return Err(GatewayError::Unknown);
        }

        // Calculate total deposit amount
        let total_deposit_amount =
            lamports * num_squares * num_rounds + (num_rounds * AUTOMATION_FEE);
        // let total_deposit_amount = lamports * num_squares * num_rounds;

        // Get sol balance
        let Ok(sol_balance) = sol_balance() else {
            return Err(GatewayError::Unknown);
        };

        // Returns false if sol balance is insufficient
        if total_deposit_amount > sol_balance.0 {
            return Err(GatewayError::InsufficientSOL);
        }

        // Calculate strategy
        let strategy = if randomize_tiles() {
            AutomationStrategy::Random
        } else {
            AutomationStrategy::Preferred
        };

        // Calculate mask
        // Convert array of 25 booleans into a 32-bit mask where each bit represents whether
        // that square index is selected (1) or not (0)
        let mut mask: u32 = 0;
        for (i, &square) in selected_squares().iter().enumerate() {
            if square {
                mask |= 1 << i;
            }
        }

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(100_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        log::info!("strategy: {:?}", strategy);

        // Push automate instruction
        ixs.push(ore_api::sdk::automate(
            authority,
            lamports,
            total_deposit_amount,
            EXECUTOR_ADDRESS,
            AUTOMATION_FEE,
            mask as u64,
            strategy.into(),
            auto_reload(),
            AutomationConditions::default(),
        ));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
