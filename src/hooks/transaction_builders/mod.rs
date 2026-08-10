mod use_automate_transaction;
mod use_cancel_transaction;
mod use_claim_ore_transaction;
mod use_claim_sol_transaction;
mod use_pro_deploy_transaction;
mod use_deploy_transaction;
mod use_reset_transaction;
mod use_topup_transaction;

pub use use_automate_transaction::*;
pub use use_cancel_transaction::*;
pub use use_claim_ore_transaction::*;
pub use use_claim_sol_transaction::*;
pub use use_pro_deploy_transaction::*;
pub use use_deploy_transaction::*;
pub use use_reset_transaction::*;
pub use use_topup_transaction::*;

pub const JITO_TIP_AMOUNT: u64 = 2_000;
pub const AUTOMATION_FEE_V2: u64 = 5000 + JITO_TIP_AMOUNT;

use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;

pub fn tip_ix(signer: &Pubkey) -> Instruction {
    let address = get_jito_tip_address();
    solana_sdk::system_instruction::transfer(signer, &address, JITO_TIP_AMOUNT)
}

fn get_jito_tip_address() -> Pubkey {
    let addresses = [
        solana_sdk::pubkey!("96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5"),
        solana_sdk::pubkey!("HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe"),
        solana_sdk::pubkey!("Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY"),
        solana_sdk::pubkey!("ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49"),
        solana_sdk::pubkey!("DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh"),
        solana_sdk::pubkey!("ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt"),
        solana_sdk::pubkey!("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL"),
        solana_sdk::pubkey!("3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT"),
    ];

    let random_index = rand::random::<usize>() % addresses.len();
    addresses[random_index]
}
