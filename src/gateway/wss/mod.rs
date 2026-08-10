mod web;

pub use web::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;

////////////////////////////////////////////////////////////////////////////
// JSON‑RPC Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JsonRpcResponseWithError<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

////////////////////////////////////////////////////////////////////////////
// Account Subscription Request/Response Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct AccountSubscribeConfig {
    pub encoding: String,
    pub commitment: String,
}

////////////////////////////////////////////////////////////////////////////
// Account Notification Types
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountContext {
    pub slot: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotification {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    // Some RPC gateways pass u64::MAX through JavaScript and round it beyond Rust's u64 range.
    // This metadata is not used by account subscribers, so preserve its wire value as JSON.
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: serde_json::Value,
    pub space: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotificationResult {
    pub context: AccountContext,
    pub value: AccountNotification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotificationEnvelope {
    pub jsonrpc: String,
    pub method: String,
    pub params: AccountNotificationParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountNotificationParams {
    pub result: AccountNotificationResult,
    pub subscription: u64,
}

////////////////////////////////////////////////////////////////////////////
// Subscription Trait
////////////////////////////////////////////////////////////////////////////

// TODO: reconnect logic
#[async_trait(?Send)]
pub trait AccountSubscribe: Sized {
    type SubscriptionId: Copy + Debug;
    async fn connect(url: &str) -> Result<Self, SubscriptionError>;
    async fn subscribe(
        &mut self,
        account: &str,
        request_id: u32,
    ) -> Result<Self::SubscriptionId, SubscriptionError>;
    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError>;
    async fn next_notification(&mut self)
        -> Result<AccountNotificationEnvelope, SubscriptionError>;
}

#[derive(Debug)]
pub enum SubscriptionError {
    ConnectionError(String),
    ParseError(String),
    RpcError(String),
    Other(String),
}

impl std::fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::RpcError(msg) => write!(f, "RPC error: {}", msg),
            Self::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for SubscriptionError {}

#[cfg(test)]
mod tests {
    use super::AccountNotificationEnvelope;

    #[test]
    fn parses_javascript_rounded_rent_epoch_notifications() {
        let payload = r#"{
            "jsonrpc":"2.0",
            "method":"accountNotification",
            "params":{
                "result":{
                    "context":{"slot":438461458},
                    "value":{
                        "data":["","base64"],
                        "executable":false,
                        "lamports":1169280,
                        "owner":"Sysvar1111111111111111111111111111111111111",
                        "rentEpoch":18446744073709552000,
                        "space":40
                    }
                },
                "subscription":574975
            }
        }"#;

        let notification = serde_json::from_str::<AccountNotificationEnvelope>(payload).unwrap();
        assert_eq!(notification.params.subscription, 574975);
    }
}
