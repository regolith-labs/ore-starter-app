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
    hooks::{
        tip_ix, use_board_wss, use_miner_wss, use_sol_balance_wss, use_wallet, Wallet,
        AUTOMATION_FEE_V2,
    },
};

/// Pro mode unified deploy transaction builder.
///
/// - When `num_rounds == 1`: builds a manual deploy transaction.
/// - When `num_rounds > 1`: builds an automate transaction.
///
/// `total_amount` is the big SOL input (total collateral).
/// `num_squares` is the number of tiles to deploy on.
/// `selected_squares` is which specific squares are selected.
/// `num_rounds` is the number of rounds.
/// `auto_reload` controls whether the auto miner loops indefinitely (only used when rounds > 1).
pub fn use_pro_deploy_transaction(
    total_amount: Signal<String>,
    num_squares: Memo<u64>,
    selected_squares: Signal<[bool; 25]>,
    num_rounds: Memo<u64>,
    auto_reload: Signal<bool>,
    randomize_tiles: Signal<bool>,
    solo_tiles: Memo<u16>,
    split_tiles: Memo<u16>,
    min_motherlode: Memo<u16>,
    max_motherlode: Memo<u16>,
) -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();
    let sol_balance = use_sol_balance_wss();
    let board = use_board_wss();
    let miner = use_miner_wss();

    use_resource(move || async move {
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        let total_sol = total_amount()
            .parse::<f64>()
            .map_err(|_| GatewayError::Unknown)?;
        let total_lamports = sol_to_lamports(total_sol);
        if total_lamports == 0 {
            return Err(GatewayError::AmountTooSmall);
        }

        let Ok(sol_balance) = sol_balance() else {
            return Err(GatewayError::Unknown);
        };
        if total_lamports > sol_balance.0 {
            return Err(GatewayError::InsufficientSOL);
        }

        let num_squares = num_squares();
        if num_squares == 0 {
            return Err(GatewayError::NoTerritorySelected);
        }
        if num_squares > 25 {
            return Err(GatewayError::Unknown);
        }

        let num_rounds = num_rounds();
        if num_rounds == 0 {
            return Err(GatewayError::Unknown);
        }

        let lamports_per_square = total_lamports / (num_squares * num_rounds);
        if lamports_per_square == 0 {
            return Err(GatewayError::AmountTooSmall);
        }

        if num_rounds == 1 {
            // Manual deploy for a single round
            let selected = selected_squares();
            if selected.iter().all(|&x| !x) {
                return Err(GatewayError::NoTerritorySelected);
            }

            let Ok(board) = board() else {
                return Err(GatewayError::Unknown);
            };

            let mut ixs = vec![];
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(750_000));
            ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

            if let Ok(miner) = miner() {
                ixs.push(ore_api::sdk::checkpoint(
                    authority,
                    authority,
                    miner.round_id,
                ));
            }

            ixs.push(ore_api::sdk::deploy(
                authority,
                authority,
                lamports_per_square,
                board.round_id,
                selected,
            ));

            ixs.push(tip_ix(&authority));
            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        } else {
            // Automate transaction for multiple rounds
            let strategy = if solo_tiles() > 0 || split_tiles() > 0 || randomize_tiles() {
                AutomationStrategy::Random
            } else {
                AutomationStrategy::Preferred
            };

            let mut mask: u32 = 0;
            for (i, &square) in selected_squares().iter().enumerate() {
                if square {
                    mask |= 1 << i;
                }
            }

            let deploy_per_round = lamports_per_square * num_squares;
            let total_fees = num_rounds * AUTOMATION_FEE_V2;
            let total_deposit_amount = (num_rounds * deploy_per_round) + total_fees;

            let Ok(board) = board() else {
                return Err(GatewayError::Unknown);
            };

            let mut ixs = vec![];
            ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(850_000));
            ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));
            let conditions = AutomationConditions {
                solo_tiles: solo_tiles(),
                split_tiles: split_tiles(),
                min_motherlode: min_motherlode(),
                max_motherlode: max_motherlode(),
                ..AutomationConditions::default()
            };
            ixs.push(ore_api::sdk::automate(
                authority,
                lamports_per_square,
                total_deposit_amount,
                EXECUTOR_ADDRESS,
                AUTOMATION_FEE_V2,
                mask as u64,
                strategy.into(),
                auto_reload(),
                conditions,
            ));

            // Crank the first round immediately so the user doesn't wait for offchain infra
            let checkpoint_round_id = miner().map(|m| m.round_id).unwrap_or(0);
            ixs.push(ore_api::sdk::checkpoint(
                authority,
                authority,
                checkpoint_round_id,
            ));
            ixs.push(ore_api::sdk::deploy(
                authority,
                authority,
                lamports_per_square,
                board.round_id,
                selected_squares(),
            ));

            ixs.push(tip_ix(&authority));

            let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
            Ok(tx)
        }
    })
}
