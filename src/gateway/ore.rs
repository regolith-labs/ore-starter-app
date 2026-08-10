use cached::proc_macro::cached;
use ore_api::prelude::*;
use ore_types::{
    request::{
        ChatTypingRequest, DiscordAuthRequest, UsernameUpdateRequest, UsernameValidateRequest,
    },
    response::{
        ChatTypingResponse, DailyRevenue, DeployHistoryEvent, DiscordAuthResponse,
        LeaderboardEntry, ResetEventResponse, RoundMinersResponse, User,
        UsernameValidationResponse,
    },
};
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature;
use steel::{AccountDeserialize, Pubkey};

use crate::gateway::GatewayError;
use crate::utils::Duration;

use super::{GatewayResult, Rpc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OreVolumeResponse {
    pub volume_24h_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSnapshot {
    pub ts: String,
    pub price: f64,
    pub volume_24h: f64,
    #[serde(default)]
    pub liquidity: f64,
    pub production_cost: f64,
    pub staking_apy: f64,
    pub staking_balance: u64,
    pub staking_store_balance: u64,
    pub circulating_supply: f64,
    #[serde(default)]
    pub holders: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningSnapshot {
    pub ts: String,
    pub avg_deployed: u64,
    pub avg_miners: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsHistoryResponse {
    pub snapshots: Vec<StatsSnapshot>,
    pub mining: Vec<MiningSnapshot>,
}

const ORE_API_URL: &str = "https://api.ore.com";

pub trait OreGateway {
    async fn get_automation(&self, address: Pubkey) -> GatewayResult<Automation>;
    async fn get_board(&self) -> GatewayResult<Board>;
    async fn get_config(&self) -> GatewayResult<Config>;
    async fn get_treasury(&self) -> GatewayResult<Treasury>;
    async fn get_stake(&self, address: Pubkey) -> GatewayResult<ore_stake_api::prelude::Stake>;
    async fn get_stake_treasury(&self) -> GatewayResult<ore_stake_api::prelude::Treasury>;
    async fn get_stake_vesting(&self) -> GatewayResult<ore_stake_api::state::Vesting>;
    async fn get_miner(&self, address: Pubkey) -> GatewayResult<Miner>;
    async fn get_miners(&self, pubkeys: &[Pubkey]) -> GatewayResult<Vec<Miner>>;
    async fn get_slothash(&self, slot: u64) -> GatewayResult<[u8; 32]>;
    async fn get_round(&self, id: u64) -> GatewayResult<Round>;
    async fn get_user(&self, address: Pubkey) -> GatewayResult<User>;
    async fn update_user_username(
        &self,
        address: Pubkey,
        username: String,
        jwt: String,
    ) -> GatewayResult<User>;
    async fn validate_username(
        &self,
        address: Pubkey,
        username: String,
    ) -> GatewayResult<UsernameValidationResponse>;
    async fn upload_profile_photo(
        &self,
        address: Pubkey,
        file_bytes: Vec<u8>,
        file_type: String,
        jwt: String,
    ) -> GatewayResult<User>;
    async fn get_reset_events(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<(Signature, ResetEventResponse)>>;
    async fn get_motherlode_events(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<(Signature, ResetEventResponse)>>;
    async fn get_bury_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>>;
    async fn get_liq_events(&self, page: u64) -> GatewayResult<Vec<(Signature, LiqEvent)>>;
    async fn get_buywall_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>>;
    async fn get_buyback_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>>;
    async fn get_protocol_rev_7d(&self) -> GatewayResult<u64>;
    async fn get_protocol_rev_30d(&self) -> GatewayResult<u64>;
    async fn get_buried_7d(&self) -> GatewayResult<u64>;
    async fn get_buried_30d(&self) -> GatewayResult<u64>;
    async fn get_shared_7d(&self) -> GatewayResult<u64>;
    async fn get_daily_revenue_21d(&self) -> GatewayResult<Vec<DailyRevenue>>;
    async fn get_cumulative_revenue_history(&self) -> GatewayResult<Vec<DailyRevenue>>;
    async fn get_total_revenue(&self) -> GatewayResult<u64>;
    async fn get_revenue_24h(&self) -> GatewayResult<u64>;
    async fn auth_discord(&self, code: String, jwt: String) -> GatewayResult<DiscordAuthResponse>;
    async fn get_ore_volume_24h(&self) -> GatewayResult<OreVolumeResponse>;
    async fn get_top_miners_unrefined(&self, page: u64) -> GatewayResult<Vec<LeaderboardEntry>>;
    async fn get_top_miners_lifetime_deployed_sol(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<LeaderboardEntry>>;
    async fn get_top_stakers(&self, page: u64) -> GatewayResult<Vec<LeaderboardEntry>>;
    async fn get_deploy_history(
        &self,
        authority: Pubkey,
        page: u64,
    ) -> GatewayResult<Vec<DeployHistoryEvent>>;
    async fn get_round_miners(
        &self,
        round_id: u64,
        authority: Option<Pubkey>,
        limit: u64,
        offset: u64,
    ) -> GatewayResult<RoundMinersResponse>;
    async fn ban_user(&self, authority: Pubkey, jwt: String) -> GatewayResult<User>;
    async fn get_stats_history(&self) -> GatewayResult<StatsHistoryResponse>;
}

impl<R: Rpc> OreGateway for R {
    async fn get_automation(&self, address: Pubkey) -> GatewayResult<Automation> {
        let account_data = self.get_account_data(&address).await?;
        let automation = Automation::try_from_bytes(&account_data)?;
        Ok(*automation)
    }

    async fn get_board(&self) -> GatewayResult<Board> {
        let board_address = board_pda().0;
        let account_data = self.get_account_data(&board_address).await?;
        let board = Board::try_from_bytes(&account_data)?;
        Ok(*board)
    }

    async fn get_config(&self) -> GatewayResult<Config> {
        let config_address = ore_api::state::config_pda().0;
        let account_data = self.get_account_data(&config_address).await?;
        let config = Config::try_from_bytes(&account_data)?;
        Ok(*config)
    }

    async fn get_treasury(&self) -> GatewayResult<Treasury> {
        let treasury_address = ore_api::state::treasury_pda().0;
        let account_data = self.get_account_data(&treasury_address).await?;
        let treasury = Treasury::try_from_bytes(&account_data)?;
        Ok(*treasury)
    }

    async fn get_stake(&self, address: Pubkey) -> GatewayResult<ore_stake_api::prelude::Stake> {
        let account_data = self.get_account_data(&address).await?;
        let stake = ore_stake_api::prelude::Stake::try_from_bytes(&account_data)?;
        Ok(*stake)
    }

    async fn get_stake_treasury(&self) -> GatewayResult<ore_stake_api::prelude::Treasury> {
        let treasury_address = ore_stake_api::state::treasury_pda().0;
        let account_data = self.get_account_data(&treasury_address).await?;
        let treasury = ore_stake_api::prelude::Treasury::try_from_bytes(&account_data)?;
        Ok(*treasury)
    }

    async fn get_stake_vesting(&self) -> GatewayResult<ore_stake_api::state::Vesting> {
        let vesting_address = ore_stake_api::state::vesting_pda().0;
        let account_data = self.get_account_data(&vesting_address).await?;
        let vesting = ore_stake_api::state::Vesting::try_from_bytes(&account_data)?;
        Ok(*vesting)
    }

    async fn get_miner(&self, address: Pubkey) -> GatewayResult<Miner> {
        let account_data = self.get_account_data(&address).await?;
        let miner = Miner::try_from_bytes(&account_data)?;
        Ok(*miner)
    }

    async fn get_miners(&self, pubkeys: &[Pubkey]) -> GatewayResult<Vec<Miner>> {
        let account_datas = self.get_multiple_account_datas(pubkeys).await?;
        let miners = account_datas
            .into_iter()
            .filter_map(|data| {
                if let Ok(miner) = Miner::try_from_bytes(&data) {
                    Some(*miner)
                } else {
                    None
                }
            })
            .collect();
        Ok(miners)
    }

    async fn get_slothash(&self, slot: u64) -> GatewayResult<[u8; 32]> {
        let client = reqwest::Client::new();
        client
            .get(format!("{}/slothash?slot={}", ORE_API_URL, slot))
            .send()
            .await?
            .json::<[u8; 32]>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    }

    async fn get_round(&self, id: u64) -> GatewayResult<Round> {
        let round_address = ore_api::state::round_pda(id).0;
        let account_data = self.get_account_data(&round_address).await?;
        let round = Round::try_from_bytes(&account_data)?;
        Ok(*round)
    }

    async fn get_user(&self, address: Pubkey) -> GatewayResult<User> {
        get_user_cached(address).await
    }

    async fn update_user_username(
        &self,
        address: Pubkey,
        username: String,
        jwt: String,
    ) -> GatewayResult<User> {
        let url = format!("{}/users/{}/username", ORE_API_URL, address);
        log::info!("Gateway: Sending username update request to: {}", url);
        log::info!("Gateway: Username: '{}', Address: {}", username, address);

        let client = reqwest::Client::new();
        let request = UsernameUpdateRequest {
            username: username.clone(),
        };

        log::info!("Gateway: Sending POST request...");
        let response = match client
            .post(&url)
            .json(&request)
            .bearer_auth(&jwt)
            .send()
            .await
        {
            Ok(resp) => {
                log::info!("Gateway: Received response");
                resp
            }
            Err(e) => {
                log::error!("Gateway: Request failed to send: {:?}", e);
                return Err(e.into());
            }
        };

        let status = response.status();
        log::info!("Gateway: Response status: {} ({})", status.as_u16(), status);

        if status.is_success() {
            log::info!("Gateway: Request successful, parsing user response...");
            let user = response.json::<User>().await.map_err(|e| {
                log::error!("Gateway: Failed to parse user response JSON: {:?}", e);
                GatewayError::RequestFailed
            })?;
            log::info!(
                "Gateway: Successfully updated username to '{}' for user {}",
                username,
                address
            );
            Ok(user)
        } else if status.as_u16() == 409 {
            log::warn!(
                "Gateway: Username '{}' already exists (409 Conflict)",
                username
            );
            Err(GatewayError::UsernameExists)
        } else if status.as_u16() == 429 {
            log::warn!("Gateway: Username update rate limited (429 Too Many Requests)");
            Err(GatewayError::UsernameUpdateTimeout)
        } else {
            let body_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            log::error!(
                "Gateway: Request failed with status {} - Response body: {}",
                status,
                body_text
            );
            Err(GatewayError::RequestFailed)
        }
    }

    async fn validate_username(
        &self,
        address: Pubkey,
        username: String,
    ) -> GatewayResult<UsernameValidationResponse> {
        let url = format!("{}/users/validate-username", ORE_API_URL);
        log::info!("Validating username at: {}", url);

        let client = reqwest::Client::new();
        let request_body = UsernameValidateRequest {
            username,
            authority: address.to_string(),
        };

        let response = client.post(&url).json(&request_body).send().await?;

        let status = response.status();
        log::info!("Username validation response status: {}", status);

        if status.is_success() {
            let validation_result = response
                .json::<UsernameValidationResponse>()
                .await
                .map_err(|e| {
                    log::error!("Failed to parse validation JSON: {:?}", e);
                    GatewayError::RequestFailed
                })?;
            log::info!(
                "Username validation result: valid={}, error={:?}",
                validation_result.valid,
                validation_result.error
            );
            Ok(validation_result)
        } else {
            log::error!("Failed to validate username, status: {}", status);
            Err(GatewayError::RequestFailed)
        }
    }

    async fn upload_profile_photo(
        &self,
        address: Pubkey,
        file_bytes: Vec<u8>,
        file_type: String,
        jwt: String,
    ) -> GatewayResult<User> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;

        let url = format!("{}/users/{}/profile-photo", ORE_API_URL, address);

        // Create a Blob from the file bytes
        let uint8_array = js_sys::Uint8Array::from(file_bytes.as_slice());
        let array = js_sys::Array::new();
        array.push(&uint8_array);

        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type(&file_type);
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&array, &blob_options)
            .map_err(|e| {
                log::error!("Failed to create blob: {:?}", e);
                GatewayError::RequestFailed
            })?;

        // Create FormData and append the blob
        let form_data = web_sys::FormData::new().map_err(|e| {
            log::error!("Failed to create FormData: {:?}", e);
            GatewayError::RequestFailed
        })?;
        form_data.append_with_blob("file", &blob).map_err(|e| {
            log::error!("Failed to append blob to FormData: {:?}", e);
            GatewayError::RequestFailed
        })?;

        // Create the request
        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&form_data);

        let request = web_sys::Request::new_with_str_and_init(&url, &opts).map_err(|e| {
            log::error!("Failed to create request: {:?}", e);
            GatewayError::RequestFailed
        })?;

        // Add Authorization header
        let auth_header = format!("Bearer {}", jwt);
        request
            .headers()
            .set("Authorization", &auth_header)
            .map_err(|e| {
                log::error!("Failed to set Authorization header: {:?}", e);
                GatewayError::RequestFailed
            })?;

        // Make the fetch request
        let window = web_sys::window().ok_or_else(|| {
            log::error!("Failed to get window object");
            GatewayError::RequestFailed
        })?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| {
                log::error!("Fetch request failed: {:?}", e);
                GatewayError::RequestFailed
            })?;

        let resp: web_sys::Response = resp_value.dyn_into().map_err(|e| {
            log::error!("Failed to cast response: {:?}", e);
            GatewayError::RequestFailed
        })?;

        let status = resp.status();
        let status_text = resp.status_text();

        if !resp.ok() {
            log::error!("Request failed with status: {} {}", status, status_text);

            // Try to get response body for more details
            if let Ok(text_promise) = resp.text() {
                if let Ok(text_value) = JsFuture::from(text_promise).await {
                    if let Some(text) = text_value.as_string() {
                        log::error!("Response body: {}", text);
                    }
                }
            }

            return Err(GatewayError::RequestFailed);
        }

        // Parse the JSON response
        let json = JsFuture::from(resp.json().map_err(|e| {
            log::error!("Failed to get JSON from response: {:?}", e);
            GatewayError::RequestFailed
        })?)
        .await
        .map_err(|e| {
            log::error!("Failed to parse JSON: {:?}", e);
            GatewayError::RequestFailed
        })?;

        // Deserialize the user response
        let user: User = serde_wasm_bindgen::from_value(json).map_err(|e| {
            log::error!("Failed to deserialize user: {:?}", e);
            GatewayError::RequestFailed
        })?;

        Ok(user)
    }

    async fn get_reset_events(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<(Signature, ResetEventResponse)>> {
        get_reset_events_cached(page).await
    }

    async fn get_motherlode_events(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<(Signature, ResetEventResponse)>> {
        get_motherlode_events_cached(page).await
    }

    async fn get_bury_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
        get_bury_events_cached(page).await
    }

    async fn get_liq_events(&self, page: u64) -> GatewayResult<Vec<(Signature, LiqEvent)>> {
        get_liq_events_cached(page).await
    }

    async fn get_buywall_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
        get_buywall_events_cached(page).await
    }

    async fn get_buyback_events(&self, page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
        get_buyback_events_cached(page).await
    }

    async fn get_protocol_rev_7d(&self) -> GatewayResult<u64> {
        get_protocol_rev_7d_cached().await
    }

    async fn get_protocol_rev_30d(&self) -> GatewayResult<u64> {
        get_protocol_rev_30d_cached().await
    }

    async fn get_buried_7d(&self) -> GatewayResult<u64> {
        get_buried_7d_cached().await
    }

    async fn get_buried_30d(&self) -> GatewayResult<u64> {
        get_buried_30d_cached().await
    }

    async fn get_shared_7d(&self) -> GatewayResult<u64> {
        get_shared_7d_cached().await
    }

    async fn get_daily_revenue_21d(&self) -> GatewayResult<Vec<DailyRevenue>> {
        get_daily_revenue_21d_cached().await
    }

    async fn get_cumulative_revenue_history(&self) -> GatewayResult<Vec<DailyRevenue>> {
        get_cumulative_revenue_history_cached().await
    }

    async fn get_total_revenue(&self) -> GatewayResult<u64> {
        get_total_revenue_cached().await
    }

    async fn get_revenue_24h(&self) -> GatewayResult<u64> {
        get_revenue_24h_cached().await
    }

    async fn auth_discord(&self, code: String, jwt: String) -> GatewayResult<DiscordAuthResponse> {
        let url = format!("{}/auth/discord", ORE_API_URL);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .bearer_auth(&jwt)
            .json(&DiscordAuthRequest {
                code,
                redirect_uri: "https://ore.com/auth/discord".to_string(),
            })
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::CONFLICT {
            return Err(GatewayError::DiscordAccountAlreadyLinked);
        }

        response
            .json::<DiscordAuthResponse>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    }

    async fn get_ore_volume_24h(&self) -> GatewayResult<OreVolumeResponse> {
        get_ore_volume_24h_cached().await
    }

    async fn get_top_miners_unrefined(&self, page: u64) -> GatewayResult<Vec<LeaderboardEntry>> {
        get_top_miners_unrefined_cached(page).await
    }

    async fn get_top_miners_lifetime_deployed_sol(
        &self,
        page: u64,
    ) -> GatewayResult<Vec<LeaderboardEntry>> {
        get_top_miners_lifetime_deployed_sol_cached(page).await
    }

    async fn get_top_stakers(&self, page: u64) -> GatewayResult<Vec<LeaderboardEntry>> {
        get_top_stakers_cached(page).await
    }

    async fn get_deploy_history(
        &self,
        authority: Pubkey,
        page: u64,
    ) -> GatewayResult<Vec<DeployHistoryEvent>> {
        get_deploy_history_impl(authority, page).await
    }

    async fn get_round_miners(
        &self,
        round_id: u64,
        authority: Option<Pubkey>,
        limit: u64,
        offset: u64,
    ) -> GatewayResult<RoundMinersResponse> {
        get_round_miners_cached(round_id, authority.map(|a| a.to_string()), limit, offset).await
    }

    async fn ban_user(&self, authority: Pubkey, jwt: String) -> GatewayResult<User> {
        let url = format!("{}/users/{}/ban", ORE_API_URL, authority);
        let client = reqwest::Client::new();
        let response = client.post(&url).bearer_auth(&jwt).send().await?;
        if response.status().is_success() {
            response
                .json::<User>()
                .await
                .map_err(|_| GatewayError::RequestFailed)
        } else {
            Err(GatewayError::RequestFailed)
        }
    }

    async fn get_stats_history(&self) -> GatewayResult<StatsHistoryResponse> {
        get_stats_history_cached().await
    }
}

/// Send a typing event to the API.
pub async fn send_typing(
    authority: Pubkey,
    jwt: String,
    typing: bool,
) -> GatewayResult<ChatTypingResponse> {
    let url = format!("{}/chat/typing/{}", ORE_API_URL, authority);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&ChatTypingRequest { typing })
        .bearer_auth(&jwt)
        .send()
        .await?;

    if response.status().is_success() {
        response
            .json::<ChatTypingResponse>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    } else {
        Err(GatewayError::RequestFailed)
    }
}

// Cached API calls

#[cached(time = 600)]
async fn get_user_cached(address: Pubkey) -> GatewayResult<User> {
    let url = format!("{}/users/{}", ORE_API_URL, address);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let user = response
            .json::<User>()
            .await
            .map_err(|_| GatewayError::RequestFailed)?;
        Ok(user)
    } else {
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 60, result = true)]
async fn get_reset_events_cached(page: u64) -> GatewayResult<Vec<(Signature, ResetEventResponse)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/reset?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, ResetEventResponse)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 60, result = true)]
async fn get_motherlode_events_cached(
    page: u64,
) -> GatewayResult<Vec<(Signature, ResetEventResponse)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/motherlode?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, ResetEventResponse)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_bury_events_cached(page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/bury?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, BuryEvent)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_liq_events_cached(page: u64) -> GatewayResult<Vec<(Signature, LiqEvent)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/liq?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, LiqEvent)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_buyback_events_cached(page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/buyback?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, BuryEvent)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_buywall_events_cached(page: u64) -> GatewayResult<Vec<(Signature, BuryEvent)>> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/events/buywall?page={}", ORE_API_URL, page))
        .send()
        .await?
        .json::<Vec<(Signature, BuryEvent)>>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_protocol_rev_7d_cached() -> GatewayResult<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/stats/protocol-rev-7d", ORE_API_URL))
        .send()
        .await?
        .json::<u64>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_protocol_rev_30d_cached() -> GatewayResult<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/stats/revenue-30d", ORE_API_URL))
        .send()
        .await?
        .json::<u64>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_buried_7d_cached() -> GatewayResult<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/stats/buried-7d", ORE_API_URL))
        .send()
        .await?
        .json::<u64>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_buried_30d_cached() -> GatewayResult<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/stats/buried-30d", ORE_API_URL))
        .send()
        .await?
        .json::<u64>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_shared_7d_cached() -> GatewayResult<u64> {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/stats/shared-7d", ORE_API_URL))
        .send()
        .await?
        .json::<u64>()
        .await
        .map_err(|_| GatewayError::RequestFailed)
}

#[cached(time = 300, result = true)]
async fn get_daily_revenue_21d_cached() -> GatewayResult<Vec<DailyRevenue>> {
    let url = format!("{}/stats/daily-revenue-21d", ORE_API_URL);
    log::info!("Gateway: Fetching daily revenue from: {}", url);

    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(resp) => {
            log::info!("Gateway: Received response for daily-revenue-21d");
            resp
        }
        Err(e) => {
            log::error!(
                "Gateway: Request to daily-revenue-21d failed to send: {:?}",
                e
            );
            return Err(e.into());
        }
    };

    let status = response.status();
    log::info!(
        "Gateway: daily-revenue-21d response status: {} ({})",
        status.as_u16(),
        status
    );

    if status.is_success() {
        log::info!("Gateway: Parsing daily-revenue-21d JSON response...");
        let daily_revenue = response.json::<Vec<DailyRevenue>>().await.map_err(|e| {
            log::error!("Gateway: Failed to parse daily-revenue-21d JSON: {:?}", e);
            GatewayError::RequestFailed
        })?;
        log::info!(
            "Gateway: Successfully fetched {} daily revenue records",
            daily_revenue.len()
        );
        Ok(daily_revenue)
    } else {
        let body_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        log::error!(
            "Gateway: daily-revenue-21d request failed with status {} - Response body: {}",
            status,
            body_text
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_cumulative_revenue_history_cached() -> GatewayResult<Vec<DailyRevenue>> {
    let url = format!("{}/stats/cumulative-revenue-history", ORE_API_URL);
    log::info!("Gateway: Fetching cumulative revenue history from: {}", url);

    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(resp) => {
            log::info!("Gateway: Received response for cumulative-revenue-history");
            resp
        }
        Err(e) => {
            log::error!(
                "Gateway: Request to cumulative-revenue-history failed to send: {:?}",
                e
            );
            return Err(e.into());
        }
    };

    let status = response.status();
    log::info!(
        "Gateway: cumulative-revenue-history response status: {} ({})",
        status.as_u16(),
        status
    );

    if status.is_success() {
        log::info!("Gateway: Parsing cumulative-revenue-history JSON response...");
        let cumulative_revenue = response.json::<Vec<DailyRevenue>>().await.map_err(|e| {
            log::error!(
                "Gateway: Failed to parse cumulative-revenue-history JSON: {:?}",
                e
            );
            GatewayError::RequestFailed
        })?;
        log::info!(
            "Gateway: Successfully fetched {} cumulative revenue records",
            cumulative_revenue.len()
        );
        Ok(cumulative_revenue)
    } else {
        let body_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to read response body".to_string());
        log::error!(
            "Gateway: cumulative-revenue-history request failed with status {} - Response body: {}",
            status,
            body_text
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_total_revenue_cached() -> GatewayResult<u64> {
    let url = format!("{}/stats/total-revenue", ORE_API_URL);
    log::info!("Gateway: Fetching total revenue from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let total_revenue = response.json::<i64>().await.map_err(|e| {
            log::error!("Gateway: Failed to parse total-revenue JSON: {:?}", e);
            GatewayError::RequestFailed
        })?;
        log::info!(
            "Gateway: Successfully fetched total revenue: {}",
            total_revenue
        );
        Ok(total_revenue as u64)
    } else {
        log::error!(
            "Gateway: total-revenue request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_revenue_24h_cached() -> GatewayResult<u64> {
    let url = format!("{}/stats/revenue-24h", ORE_API_URL);
    log::info!("Gateway: Fetching 24h revenue from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let revenue_24h = response.json::<i64>().await.map_err(|e| {
            log::error!("Gateway: Failed to parse revenue-24h JSON: {:?}", e);
            GatewayError::RequestFailed
        })?;
        log::info!("Gateway: Successfully fetched 24h revenue: {}", revenue_24h);
        Ok(revenue_24h as u64)
    } else {
        log::error!(
            "Gateway: revenue-24h request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_ore_volume_24h_cached() -> GatewayResult<OreVolumeResponse> {
    let url = format!("{}/stats/ore-volume-24h", ORE_API_URL);
    log::info!("Gateway: Fetching 24h ORE volume from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let volume = response.json::<OreVolumeResponse>().await.map_err(|e| {
            log::error!("Gateway: Failed to parse ore-volume-24h JSON: {:?}", e);
            GatewayError::RequestFailed
        })?;
        log::info!(
            "Gateway: Successfully fetched 24h ORE volume: ${}",
            volume.volume_24h_usd
        );
        Ok(volume)
    } else {
        log::error!(
            "Gateway: ore-volume-24h request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_top_miners_unrefined_cached(page: u64) -> GatewayResult<Vec<LeaderboardEntry>> {
    let url = format!("{}/stats/top-miners-unrefined?page={}", ORE_API_URL, page);
    log::info!("Gateway: Fetching top miners unrefined from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let entries = response
            .json::<Vec<LeaderboardEntry>>()
            .await
            .map_err(|e| {
                log::error!(
                    "Gateway: Failed to parse top-miners-unrefined JSON: {:?}",
                    e
                );
                GatewayError::RequestFailed
            })?;
        log::info!(
            "Gateway: Successfully fetched {} top miners unrefined entries",
            entries.len()
        );
        Ok(entries)
    } else {
        log::error!(
            "Gateway: top-miners-unrefined request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_top_miners_lifetime_deployed_sol_cached(
    page: u64,
) -> GatewayResult<Vec<LeaderboardEntry>> {
    let url = format!(
        "{}/stats/top-miners-lifetime-deployed-sol?page={}",
        ORE_API_URL, page
    );
    log::info!(
        "Gateway: Fetching top miners lifetime deployed SOL from: {}",
        url
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let entries = response
            .json::<Vec<LeaderboardEntry>>()
            .await
            .map_err(|e| {
                log::error!(
                    "Gateway: Failed to parse top-miners-lifetime-deployed-sol JSON: {:?}",
                    e
                );
                GatewayError::RequestFailed
            })?;
        log::info!(
            "Gateway: Successfully fetched {} top miners lifetime deployed SOL entries",
            entries.len()
        );
        Ok(entries)
    } else {
        log::error!(
            "Gateway: top-miners-lifetime-deployed-sol request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_top_stakers_cached(page: u64) -> GatewayResult<Vec<LeaderboardEntry>> {
    let url = format!("{}/stats/top-stakers?page={}", ORE_API_URL, page);
    log::info!("Gateway: Fetching top stakers from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let entries = response
            .json::<Vec<LeaderboardEntry>>()
            .await
            .map_err(|e| {
                log::error!("Gateway: Failed to parse top-stakers JSON: {:?}", e);
                GatewayError::RequestFailed
            })?;
        log::info!(
            "Gateway: Successfully fetched {} top stakers entries",
            entries.len()
        );
        Ok(entries)
    } else {
        log::error!(
            "Gateway: top-stakers request failed with status {}",
            response.status()
        );
        Err(GatewayError::RequestFailed)
    }
}

async fn get_deploy_history_impl(
    authority: Pubkey,
    page: u64,
) -> GatewayResult<Vec<DeployHistoryEvent>> {
    let url = format!(
        "{}/events/deploys/{}?page={}&limit=12",
        ORE_API_URL, authority, page
    );
    log::info!("Gateway: Fetching deploy history from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    let status = response.status();
    if status.is_success() {
        let events = response
            .json::<Vec<DeployHistoryEvent>>()
            .await
            .map_err(|e| {
                log::error!("Gateway: Failed to parse deploy history JSON: {:?}", e);
                GatewayError::RequestFailed
            })?;
        log::info!(
            "Gateway: Successfully fetched {} deploy history events",
            events.len()
        );
        Ok(events)
    } else if status.as_u16() == 401 || status.as_u16() == 403 {
        // Silent fail for auth errors - return empty vec
        log::warn!(
            "Gateway: Deploy history auth failed with status {}, returning empty",
            status
        );
        Ok(vec![])
    } else {
        log::error!(
            "Gateway: deploy history request failed with status {}",
            status
        );
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 30, result = true)]
async fn get_round_miners_cached(
    round_id: u64,
    authority: Option<String>,
    limit: u64,
    offset: u64,
) -> GatewayResult<RoundMinersResponse> {
    let mut url = format!(
        "{}/round/{}/miners?limit={}&offset={}",
        ORE_API_URL, round_id, limit, offset
    );
    if let Some(auth) = &authority {
        url.push_str(&format!("&authority={}", auth));
    }
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    if response.status().is_success() {
        response
            .json::<RoundMinersResponse>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    } else {
        Err(GatewayError::RequestFailed)
    }
}

#[cached(time = 300, result = true)]
async fn get_stats_history_cached() -> GatewayResult<StatsHistoryResponse> {
    let url = format!("{}/stats/history", ORE_API_URL);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    if response.status().is_success() {
        response
            .json::<StatsHistoryResponse>()
            .await
            .map_err(|_| GatewayError::RequestFailed)
    } else {
        Err(GatewayError::RequestFailed)
    }
}
