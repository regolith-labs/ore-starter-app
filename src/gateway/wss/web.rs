use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message as GlooMessage;
use serde::{Deserialize, Serialize};
use serde_json;

use super::{
    AccountNotificationEnvelope, AccountSubscribe, AccountSubscribeConfig, JsonRpcRequest,
    JsonRpcResponse, JsonRpcResponseWithError, SubscriptionError,
};

/// WebSocket client for account subscriptions
pub struct AccountSubscribeGateway {
    writer: futures_util::stream::SplitSink<WebSocket, GlooMessage>,
    reader: futures_util::stream::SplitStream<WebSocket>,
}

#[derive(Deserialize)]
struct SubscriptionErrorEnvelope {
    params: SubscriptionErrorParams,
}

#[derive(Deserialize)]
struct SubscriptionErrorParams {
    error: SubscriptionErrorBody,
}

#[derive(Deserialize)]
struct SubscriptionErrorBody {
    message: String,
}

fn parse_subscription_error(text: &str) -> Option<String> {
    serde_json::from_str::<SubscriptionErrorEnvelope>(text)
        .ok()
        .map(|envelope| envelope.params.error.message)
}

impl AccountSubscribeGateway {
    async fn send_request<T: Serialize>(
        &mut self,
        request: &JsonRpcRequest<T>,
    ) -> Result<(), SubscriptionError> {
        let req_json = serde_json::to_string(&request)
            .map_err(|e| SubscriptionError::ParseError(e.to_string()))?;
        self.writer
            .send(GlooMessage::Text(req_json))
            .await
            .map_err(|e| SubscriptionError::Other(e.to_string()))
    }

    async fn handle_response<R: serde::de::DeserializeOwned>(
        &mut self,
        request_id: u32,
    ) -> Result<R, SubscriptionError> {
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    if let Ok(resp) = serde_json::from_str::<JsonRpcResponse<R>>(&text) {
                        if resp.id == request_id {
                            return Ok(resp.result);
                        }
                    }
                    if let Ok(resp_err) = serde_json::from_str::<JsonRpcResponseWithError<R>>(&text)
                    {
                        if resp_err.id == request_id {
                            let err_msg = resp_err
                                .error
                                .map(|e| e.message)
                                .unwrap_or_else(|| "Unknown RPC error".to_string());
                            return Err(SubscriptionError::RpcError(err_msg));
                        }
                    }
                }
                Ok(_) => continue,
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended unexpectedly".to_string(),
        ))
    }
}

#[async_trait(?Send)]
impl AccountSubscribe for AccountSubscribeGateway {
    type SubscriptionId = u64;

    async fn connect(url: &str) -> Result<Self, SubscriptionError> {
        let ws = WebSocket::open(url)
            .map_err(|e| SubscriptionError::ConnectionError(format!("{:?}", e)))?;
        let (writer, reader) = ws.split();
        Ok(Self { writer, reader })
    }

    async fn subscribe(
        &mut self,
        account: &str,
        request_id: u32,
    ) -> Result<Self::SubscriptionId, SubscriptionError> {
        let config = AccountSubscribeConfig {
            encoding: "base64".to_string(),
            commitment: "confirmed".to_string(),
        };
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "accountSubscribe".to_string(),
            params: (account.to_string(), config),
        };

        self.send_request(&request).await?;
        self.handle_response(request_id).await
    }

    async fn unsubscribe(
        &mut self,
        subscription: Self::SubscriptionId,
    ) -> Result<(), SubscriptionError> {
        let request_id = fastrand::u32(..);
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "accountUnsubscribe".to_string(),
            params: (subscription,),
        };

        self.send_request(&request).await?;
        let result = self.handle_response::<bool>(request_id).await?;
        if result {
            Ok(())
        } else {
            Err(SubscriptionError::RpcError(
                "Unsubscribe failed".to_string(),
            ))
        }
    }

    async fn next_notification(
        &mut self,
    ) -> Result<AccountNotificationEnvelope, SubscriptionError> {
        while let Some(msg) = self.reader.next().await {
            match msg {
                Ok(GlooMessage::Text(text)) => {
                    match serde_json::from_str::<AccountNotificationEnvelope>(&text) {
                        Ok(notification) => {
                            if notification.method == "accountNotification" {
                                return Ok(notification);
                            } else {
                                log::info!("Ignoring non-account notification: {:?}", notification);
                                continue;
                            }
                        }
                        Err(e) => {
                            if let Some(message) = parse_subscription_error(&text) {
                                return Err(SubscriptionError::RpcError(message));
                            }
                            log::error!("Failed to parse notification: {}, text: {}", e, text);
                            continue;
                        }
                    }
                }
                Ok(msg) => {
                    log::info!("{:?}", msg);
                    continue;
                }
                Err(e) => return Err(SubscriptionError::ConnectionError(e.to_string())),
            }
        }
        Err(SubscriptionError::Other(
            "WebSocket stream ended".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_subscription_error;

    #[test]
    fn parses_provider_subscription_timeout() {
        let payload = r#"{
            "jsonrpc":"2.0",
            "method":"eth_subscription",
            "params":{
                "error":{"code":-32000,"message":"Subscription timed out"},
                "subscription":238755
            },
            "id":9
        }"#;

        assert_eq!(
            parse_subscription_error(payload).as_deref(),
            Some("Subscription timed out")
        );
    }

    #[test]
    fn ignores_unrelated_messages() {
        assert_eq!(parse_subscription_error(r#"{"jsonrpc":"2.0"}"#), None);
    }
}
