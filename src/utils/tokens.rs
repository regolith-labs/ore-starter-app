use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, SOL_MINT};
use solana_sdk::pubkey;
use steel::Pubkey;

#[derive(Clone, Debug, PartialEq)]
pub struct ListedToken {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub icon: Asset,
}

impl ListedToken {
    pub fn ore() -> Self {
        Self {
            mint: MINT_ADDRESS,
            name: "ORE".to_string(),
            symbol: "ORE".to_string(),
            decimals: 11,
            icon: asset!("/assets/ore.svg"),
        }
    }
    pub fn sol() -> Self {
        Self {
            mint: SOL_MINT,
            name: "SOL".to_string(),
            symbol: "SOL".to_string(),
            decimals: 9,
            icon: asset!("/assets/solana.png"),
        }
    }
}
