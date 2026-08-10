use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
use ore_api::consts::MINT_ADDRESS;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;

use crate::gateway::{GatewayError, GatewayResult};

const API_URL: &str = "https://lite-api.jup.ag/price/v3";

pub fn use_ore_price() -> Memo<Option<f64>> {
    let ore_quote = use_ore_quote(MINT_ADDRESS);
    use_memo(move || {
        let Ok(price) = ore_quote.cloned() else {
            return None;
        };
        Some(price)
    })
}

pub fn use_ore_quote(output_token: Pubkey) -> Signal<GatewayResult<f64>> {
    let mut quote = use_signal(|| Err(GatewayError::Unknown));
    let _ = use_resource(move || async move {
        loop {
            let client = reqwest::Client::new();
            let url = format!("{}?ids={}", API_URL, output_token.to_string());
            if let Ok(response) = client.get(url).send().await {
                if let Ok(json) = response.json::<PriceResponse>().await {
                    if let Some(asset_price) = json.0.get(&output_token.to_string()) {
                        quote.set(Ok(asset_price.usd_price));
                    }
                }
            }
            async_std::task::sleep(Duration::from_secs(60)).await;
        }
    });
    quote
}

#[derive(Debug, Deserialize, Clone)]
struct PriceResponse(HashMap<String, AssetPrice>);

#[derive(Debug, Deserialize, Clone)]
struct AssetPrice {
    #[serde(rename = "usdPrice")]
    usd_price: f64,
}
