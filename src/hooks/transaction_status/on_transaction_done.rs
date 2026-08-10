use dioxus::prelude::*;

use solana_sdk::signature::Signature;

use crate::hooks::TransactionStatus;

use super::use_transaction_status;

pub fn on_transaction_done(mut callback: impl FnMut(Signature) + 'static) {
    // Get transaction status
    let transaction_status = use_transaction_status();

    // Callback if transaction success
    use_effect(move || {
        if let Some(TransactionStatus::Done(signature)) = transaction_status() {
            callback(signature);
        }
    });
}
