pub mod entropy;
mod error;
pub mod ore;
pub mod solana;
mod utils;
pub mod wss;

pub use error::*;
use serde_json::{json, Value};
use solana_client_wasm::utils::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client_wasm::utils::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_client_wasm::WasmClient;

use solana_extra_wasm::account_decoder::UiAccountEncoding;
use solana_sdk::{
    account::Account, hash::Hash, pubkey::Pubkey, signature::Signature,
    transaction::VersionedTransaction,
};
use steel::{AccountDeserialize, Discriminator};
pub use utils::*;
pub use wss::*;

pub const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub struct Gateway<R: Rpc> {
    pub rpc: R,
}

impl<R: Rpc> Gateway<R> {
    pub fn new(rpc_url: String) -> Gateway<R> {
        Gateway {
            rpc: R::new(rpc_url),
        }
    }
}

pub trait Rpc {
    fn new(rpc_url: String) -> Self;
    async fn get_account_data(&self, pubkey: &Pubkey) -> GatewayResult<Vec<u8>>;
    async fn get_multiple_account_datas(&self, pubkeys: &[Pubkey]) -> GatewayResult<Vec<Vec<u8>>>;
    async fn get_multiple_accounts_full(
        &self,
        pubkeys: &[Pubkey],
    ) -> GatewayResult<Vec<Option<Account>>>;
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64>;
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash>;
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>>;
    async fn get_token_account(&self, pubkey: &Pubkey) -> GatewayResult<Option<UiTokenAmount>>;
    async fn get_token_supply(&self, mint: &Pubkey) -> GatewayResult<UiTokenAmount>;
    async fn get_program_accounts<T: AccountDeserialize + Discriminator + Clone>(
        &self,
        program_id: Pubkey,
        filters: Vec<RpcFilterType>,
    ) -> GatewayResult<Vec<(Pubkey, T)>>;
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature>;
}

pub struct WebRpc {
    client: WasmClient,
    rpc_url: String,
}

impl Rpc for WebRpc {
    fn new(rpc_url: String) -> Self {
        WebRpc {
            client: WasmClient::new(rpc_url.as_str()),
            rpc_url,
        }
    }

    async fn get_multiple_account_datas(&self, pubkeys: &[Pubkey]) -> GatewayResult<Vec<Vec<u8>>> {
        let account_data = self.client.get_multiple_accounts(pubkeys).await?;
        let account_data = account_data
            .into_iter()
            .map(|opt| opt.map(|acc| acc.data).unwrap_or_default())
            .collect();
        Ok(account_data)
    }

    async fn get_multiple_accounts_full(
        &self,
        pubkeys: &[Pubkey],
    ) -> GatewayResult<Vec<Option<Account>>> {
        // NOTE: WasmClient::get_multiple_accounts() has an upstream bug that
        // filters out None entries, destroying index alignment with the request.
        // We make a direct JSON-RPC call to preserve None for missing accounts.
        use base64::Engine;

        let pubkey_strs: Vec<String> = pubkeys.iter().map(|p| p.to_string()).collect();
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getMultipleAccounts",
            "params": [
                pubkey_strs,
                { "encoding": "base64", "commitment": "confirmed" }
            ]
        });

        let resp = gloo_net::http::Request::post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .body(req.to_string())
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?
            .json::<Value>()
            .await
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let values = resp["result"]["value"]
            .as_array()
            .ok_or(GatewayError::Unknown)?;

        let mut accounts = Vec::with_capacity(values.len());
        for val in values {
            if val.is_null() {
                accounts.push(None);
            } else {
                let lamports = val["lamports"].as_u64().unwrap_or(0);
                let owner_str = val["owner"].as_str().unwrap_or_default();
                let owner = owner_str.parse::<Pubkey>().unwrap_or_default();
                let executable = val["executable"].as_bool().unwrap_or(false);
                let rent_epoch = val["rentEpoch"].as_u64().unwrap_or(0);
                let data_array = val["data"].as_array();
                let data = if let Some(arr) = data_array {
                    let b64 = arr.first().and_then(|v| v.as_str()).unwrap_or_default();
                    base64::engine::general_purpose::STANDARD
                        .decode(b64)
                        .unwrap_or_default()
                } else {
                    vec![]
                };
                accounts.push(Some(Account {
                    lamports,
                    data,
                    owner,
                    executable,
                    rent_epoch,
                }));
            }
        }

        Ok(accounts)
    }

    async fn get_account_data(&self, pubkey: &Pubkey) -> GatewayResult<Vec<u8>> {
        log::info!("Getting account data: {:?}", pubkey);
        self.client
            .get_account_data(pubkey)
            .await
            .map_err(From::from)
    }

    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64> {
        self.client.get_balance(pubkey).await.map_err(From::from)
    }

    async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        self.client.get_latest_blockhash().await.map_err(From::from)
    }

    async fn get_program_accounts<T: AccountDeserialize + Discriminator + Clone>(
        &self,
        program_id: Pubkey,
        filters: Vec<RpcFilterType>,
    ) -> GatewayResult<Vec<(Pubkey, T)>> {
        let mut all_filters = vec![RpcFilterType::Memcmp(Memcmp {
            offset: 0,
            bytes: MemcmpEncodedBytes::Base58(
                bs58::encode(T::discriminator().to_le_bytes()).into_string(),
            ),
            encoding: None,
        })];
        all_filters.extend(filters);
        let result = self
            .client
            .get_program_accounts_with_config(
                &program_id,
                RpcProgramAccountsConfig {
                    filters: Some(all_filters),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .await;

        match result {
            Ok(accounts) => {
                let accounts = accounts
                    .into_iter()
                    .filter_map(|(pubkey, account)| {
                        if let Ok(account) = T::try_from_bytes(&account.data) {
                            Some((pubkey, account.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(accounts)
            }
            Err(err) => {
                log::error!("Failed to get program accounts: {:?}", err);
                Err(From::from(err))
            }
        }
    }

    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>> {
        let vec = self.client.get_signature_statuses(signatures).await?;
        let vec = vec.into_iter().map(|opt| {
            if let Some(status) = opt {
                if let Some(status) = status.confirmation_status {
                    match status {
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Processed =>  Some(TransactionConfirmationStatus::Processed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Confirmed => Some(TransactionConfirmationStatus::Confirmed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Finalized => Some(TransactionConfirmationStatus::Finalized),
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
        Ok(vec)
    }

    async fn get_token_account(&self, pubkey: &Pubkey) -> GatewayResult<Option<UiTokenAmount>> {
        log::info!("Getting token account: {:?}", pubkey);
        let option = self.client.get_token_account(pubkey).await?;
        let option = option.map(|ta| UiTokenAmount {
            ui_amount: ta.token_amount.ui_amount,
            decimals: ta.token_amount.decimals,
            amount: ta.token_amount.amount,
            ui_amount_string: ta.token_amount.ui_amount_string,
        });
        Ok(option)
    }

    async fn get_token_supply(&self, mint: &Pubkey) -> GatewayResult<UiTokenAmount> {
        let ta = self.client.get_token_supply(mint).await?;
        let ta = UiTokenAmount {
            ui_amount: ta.ui_amount,
            decimals: ta.decimals,
            amount: ta.amount,
            ui_amount_string: ta.ui_amount_string,
        };
        Ok(ta)
    }

    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature> {
        self.client
            .send_versioned_transaction(transaction)
            .await
            .map_err(From::from)
    }
}
