use dioxus::prelude::*;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    native_token::sol_to_lamports,
    transaction::{Transaction, VersionedTransaction},
};

use crate::{
    gateway::{GatewayError, GatewayResult},
    hooks::{tip_ix, use_board_wss, use_miner_wss, use_sol_balance_wss, use_wallet, Wallet},
};

pub fn use_deploy_transaction(
    amount: Signal<String>,
    selected_squares: Signal<[bool; 25]>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_sol_balance_wss();
    let board = use_board_wss();
    let miner = use_miner_wss();

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Get amount
        let sol_amount_f64 = amount().parse::<f64>().map_err(|_| GatewayError::Unknown)?;
        let lamports = sol_to_lamports(sol_amount_f64);

        // Return if amount is 0
        if lamports == 0 {
            return Err(GatewayError::AmountTooSmall);
        }

        // Get winning square
        let selected_squares = selected_squares();

        // Returns false if no squares have been selected
        if selected_squares.iter().all(|&x| !x) {
            return Err(GatewayError::NoTerritorySelected);
        }

        // Get sol balance
        let Ok(sol_balance) = sol_balance() else {
            return Err(GatewayError::Unknown);
        };

        // Get board
        let Ok(board) = board() else {
            return Err(GatewayError::Unknown);
        };

        // Gets territory count
        let territory_count = selected_squares.iter().filter(|&&x| x).count();

        // Returns false if sol balance is insufficient
        let total_sum = lamports * territory_count as u64;
        if total_sum > sol_balance.0 {
            return Err(GatewayError::InsufficientSOL);
        }

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        // let budget = 100_000 + (50_000 * territory_count as u32);
        // let budget = budget.min(1_400_000);
        let budget = 750_000;
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(budget));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        // Push swap instruction
        let mut squares = [false; 25];
        for (i, selected) in selected_squares.iter().enumerate() {
            if *selected {
                squares[i] = true;
            }
        }

        // Push checkpoint instruction
        if let Ok(miner) = miner() {
            ixs.push(ore_api::sdk::checkpoint(
                authority,
                authority,
                miner.round_id,
            ));
        }

        // Push deploy instruction
        ixs.push(ore_api::sdk::deploy(
            authority,
            authority,
            lamports,
            board.round_id,
            squares,
        ));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();

        Ok(tx)
    })
}
