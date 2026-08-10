use dioxus::prelude::*;
use ore_api::consts::{MINT_ADDRESS, SOL_MINT};
use solana_sdk::pubkey;
use steel::Pubkey;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

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
    pub fn usdc() -> Self {
        Self {
            mint: USDC_MINT,
            name: "USDC".to_string(),
            symbol: "USDC".to_string(),
            decimals: 6,
            icon: asset!("/assets/usdc.png"),
        }
    }

}
