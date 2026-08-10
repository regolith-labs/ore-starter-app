use dioxus::prelude::*;
use ore_api::consts::INTERMISSION_SLOTS;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    pubkey,
    transaction::{Transaction, VersionedTransaction},
};
use steel::Pubkey;

use crate::{
    gateway::{entropy::EntropyGateway, GatewayError, GatewayResult},
    hooks::{tip_ix, use_board_wss, use_clock_wss, use_gateway, use_wallet, Wallet},
};

const FEE_COLLECTOR: Pubkey = pubkey!("DyB4Kv6V613gp2LWQTq1dwDYHGKuUEoDHnCouGUtxFiX");
const ENTROPY_VAR: Pubkey = pubkey!("BWCaDY96Xe4WkFq1M7UiCCRcChsJ3p51L5KrGzhxgm2E");

pub fn use_reset_transaction() -> Resource<GatewayResult<VersionedTransaction>> {
    let wallet = use_wallet();

    let board = use_board_wss();
    let clock = use_clock_wss();

    let needs_reset = use_memo(move || {
        let Ok(board) = board() else {
            return false;
        };
        let Ok(clock) = clock() else {
            return false;
        };
        if board.end_slot == u64::MAX {
            return false;
        }
        clock.slot > board.end_slot + INTERMISSION_SLOTS
    });

    use_resource(move || async move {
        // Check if wallet is connected
        let Wallet::Connected(authority) = wallet() else {
            return Err(GatewayError::WalletDisconnected);
        };

        // Exit if board is not initialized
        let Ok(board) = board() else {
            return Err(GatewayError::Unknown);
        };

        if !needs_reset() {
            return Err(GatewayError::Unknown);
        }

        // Fetch entropy seed from the entropy server
        let seed_response = use_gateway()
            .rpc
            .get_seed(ENTROPY_VAR, None)
            .await
            .map_err(|_| GatewayError::Unknown)?;

        // Get winning square
        let top_miner = Pubkey::default(); // TODO

        // Create instructions
        let mut ixs = vec![];

        // Push compute budget
        ixs.push(ComputeBudgetInstruction::set_compute_unit_limit(500_000));
        ixs.push(ComputeBudgetInstruction::set_compute_unit_price(0));

        // Push entropy sample and reveal instructions
        ixs.push(entropy_api::sdk::sample(authority, ENTROPY_VAR));
        ixs.push(entropy_api::sdk::reveal(authority, ENTROPY_VAR, seed_response.seed));

        // Push reset instruction
        ixs.push(ore_api::sdk::reset(
            authority,
            FEE_COLLECTOR,
            board.round_id,
            top_miner,
        ));

        // Push tip instruction
        ixs.push(tip_ix(&authority));

        // Build tx
        let tx = Transaction::new_with_payer(&ixs, Some(&authority)).into();
        Ok(tx)
    })
}
