use base64::Engine;
use dioxus::{document::eval, prelude::*};
use solana_sdk::{
    hash::Hash,
    message::VersionedMessage,
    transaction::{Transaction, VersionedTransaction},
};
use steel::Pubkey;

use crate::{
    gateway::{
        solana::SolanaGateway,
        GatewayError, GatewayResult, Rpc,
    },
    hooks::{use_gateway, TransactionStatus},
};

pub async fn _sign_transaction_partial(mut tx: Transaction) -> GatewayResult<(Transaction, Hash)> {
    // set blockhash
    let gateway = use_gateway();
    let hash = gateway.rpc.get_latest_blockhash().await?;
    let message = &mut tx.message;
    message.recent_blockhash = hash;
    // build eval command for wallet signing
    let mut eval = eval(
        r#"
        let msg = await dioxus.recv();
        let signed = await window.WalletSignTransaction({b64: msg});
        dioxus.send(signed);
        "#,
    );
    // serialize transaction to send to wallet
    let vec = bincode::serialize(&tx).map_err(|err| anyhow::anyhow!(err))?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
    let _send = eval
        .send(serde_json::Value::String(b64))
        .map_err(|err| anyhow::anyhow!(err))?;
    // wait on eval
    let res = eval.recv().await;
    // process eval result
    if let Ok(serde_json::Value::String(string)) = res {
        // decode b64 signed transaction
        let buffer = base64::engine::general_purpose::STANDARD
            .decode(string)
            .map_err(|err| anyhow::anyhow!(err))?;
        // deserialize binary to transaction
        let tx =
            bincode::deserialize::<Transaction>(&buffer).map_err(|err| anyhow::anyhow!(err))?;
        Ok((tx, hash))
    } else {
        Err(anyhow::anyhow!("unexpected response format").into())
    }
}

/// signs and submits
// pub fn submit_transaction(mut tx: VersionedTransaction, tx_type: TransactionType) {
pub fn submit_transaction(
    mut tx: VersionedTransaction,
    mut transaction_status: Signal<Option<TransactionStatus>>,
) {
    log::info!("Submitting transaction {:?}", tx);

    spawn(async move {
        // Set blockhash
        let gateway = use_gateway();

        if let Ok(hash) = gateway.rpc.get_latest_blockhash().await {
            match &mut tx.message {
                VersionedMessage::V0(message) => {
                    message.recent_blockhash = hash;
                }
                VersionedMessage::Legacy(message) => {
                    message.recent_blockhash = hash;
                }
            }
        }

        // Build eval command for wallet signing
        let mut eval = eval(
            r#"
            let msg = await dioxus.recv();
            let signed = await window.WalletSignTransaction({b64: msg});
            dioxus.send(signed);
            "#,
        );

        // Serialized the transaction to send to wallet
        match bincode::serialize(&tx) {
            Ok(vec) => {
                transaction_status.set(Some(TransactionStatus::Waiting));
                let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
                let res = eval.send(serde_json::Value::String(b64));
                match res {
                    Ok(()) => {
                        // Execute eval command
                        let res = eval.recv().await;

                        // Process eval result
                        match res {
                            // Process valid signing result
                            Ok(serde_json::Value::String(string)) => {
                                // Decode signed transaction
                                let gateway = use_gateway();
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok();
                                let decode_res = decode_res.and_then(|buffer| {
                                    bincode::deserialize::<VersionedTransaction>(&buffer).ok()
                                });

                                // Simulate transaction to check for insufficient funds
                                // let tx_for_simulation = tx.clone();
                                // if let Ok(simulated_tx) =
                                //     gateway.rpc.simulate_transaction(&tx_for_simulation).await
                                // {
                                //     if let Some(err) = simulated_tx.err {
                                //         if let TransactionError::InstructionError(
                                //             _index,
                                //             instruction_error,
                                //         ) = err
                                //         {
                                //             transaction_status.set(Some(TransactionStatus::Error(
                                //                 GatewayError::Custom(instruction_error.to_string()),
                                //             )));
                                //             return;
                                //             // if matches!(
                                //             //     instruction_error,
                                //             //     InstructionError::Custom(1)
                                //             // ) {
                                //             //     transaction_status.set(Some(
                                //             //         TransactionStatus::Error(
                                //             //             GatewayError::InsufficientSOL,
                                //             //         ),
                                //             //     ));
                                //             //     return;
                                //             // }
                                //         }
                                //     }
                                // }

                                // Send transaction to rpc
                                transaction_status.set(Some(TransactionStatus::Sending(0)));
                                let rpc_res = match decode_res {
                                    Some(tx) => gateway.rpc.send_transaction(&tx).await.ok(),
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };

                                // Confirm transaction
                                match rpc_res {
                                    Some(sig) => {
                                        let confirmed = gateway.rpc.confirm_signature(sig).await;
                                        if confirmed.is_ok() {
                                            transaction_status
                                                .set(Some(TransactionStatus::Done(sig)));
                                        } else {
                                            transaction_status
                                                .set(Some(TransactionStatus::Timeout));
                                        }
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                        transaction_status.set(Some(TransactionStatus::Error(
                                            GatewayError::Unknown,
                                        )))
                                    }
                                }
                            }

                            // Process signing errors
                            Ok(serde_json::Value::Null) => {
                                transaction_status.set(Some(TransactionStatus::Denied))
                            }
                            Err(err) => {
                                log::error!("error signing transaction: {}", err);
                                transaction_status
                                    .set(Some(TransactionStatus::Error(GatewayError::Unknown)))
                            }
                            _ => {
                                log::error!("unrecognized signing response");
                                transaction_status
                                    .set(Some(TransactionStatus::Error(GatewayError::Unknown)))
                            }
                        };
                    }

                    // Process eval errors
                    Err(err) => {
                        log::error!("error executing wallet signing script: {}", err);
                        transaction_status
                            .set(Some(TransactionStatus::Error(GatewayError::Unknown)))
                    }
                }
            }

            // Process serialization errors
            Err(err) => {
                log::error!("err serializing tx: {}", err);
                transaction_status.set(Some(TransactionStatus::Error(GatewayError::Unknown)))
            }
        };
    });
}

