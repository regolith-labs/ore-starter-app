use dioxus::prelude::*;
use solana_sdk::program_pack::Pack;
use spl_associated_token_account::get_associated_token_address;
use steel::*;

use ore_api::state::{Automation, Board, Config, Miner, Treasury};

use crate::gateway::{GatewayError, GatewayResult, Rpc};
use crate::hooks::{use_gateway, use_wallet, OreBalance, SolBalance, Wallet};

fn try_deserialize_account<T: steel::AccountDeserialize>(
    accounts: &[Option<solana_sdk::account::Account>],
    index: usize,
) -> GatewayResult<T>
where
    T: Copy,
{
    let account = accounts
        .get(index)
        .and_then(|opt| opt.as_ref())
        .filter(|a| !a.data.is_empty())
        .ok_or(GatewayError::AccountNotFound)?;
    let val = T::try_from_bytes(&account.data).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(*val)
}

fn try_unpack_token_amount(
    accounts: &[Option<solana_sdk::account::Account>],
    index: usize,
) -> GatewayResult<u64> {
    let account = accounts
        .get(index)
        .and_then(|opt| opt.as_ref())
        .filter(|a| !a.data.is_empty())
        .ok_or(GatewayError::AccountNotFound)?;
    let token_account =
        spl_token::state::Account::unpack(&account.data).map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(token_account.amount)
}

pub fn use_global_batch_init() {
    let mut config_signal: Signal<GatewayResult<Config>> = use_context();
    let mut treasury_signal: Signal<GatewayResult<Treasury>> = use_context();
    let mut board_signal: Signal<GatewayResult<Board>> = use_context();
    let mut clock_signal: Signal<GatewayResult<Clock>> = use_context();

    let _ = use_resource(move || async move {
        let gateway = use_gateway();
        let pubkeys = [
            ore_api::state::config_pda().0,
            ore_api::state::treasury_pda().0,
            ore_api::state::board_pda().0,
            sysvar::clock::ID,
        ];

        log::info!("Batch fetching {} global accounts", pubkeys.len());
        let accounts = match gateway.rpc.get_multiple_accounts_full(&pubkeys).await {
            Ok(accounts) => accounts,
            Err(err) => {
                log::error!("Failed to batch fetch global accounts: {:?}", err);
                return;
            }
        };

        config_signal.set(try_deserialize_account::<Config>(&accounts, 0));
        treasury_signal.set(try_deserialize_account::<Treasury>(&accounts, 1));
        board_signal.set(try_deserialize_account::<Board>(&accounts, 2));

        // Clock uses bincode, not try_from_bytes
        clock_signal.set(
            accounts
                .get(3)
                .and_then(|opt| opt.as_ref())
                .ok_or(GatewayError::AccountNotFound)
                .and_then(|account| {
                    bincode::deserialize::<Clock>(&account.data)
                        .map_err(|e| anyhow::anyhow!("{:?}", e).into())
                }),
        );

        log::info!("Global batch init complete");
    });
}

pub fn use_user_batch_init() {
    let wallet = use_wallet();
    let mut miner_signal: Signal<GatewayResult<Miner>> = use_context();
    let mut automation_signal: Signal<GatewayResult<Automation>> = use_context();
    let mut sol_signal: Signal<GatewayResult<SolBalance>> = use_context();
    let mut ore_signal: Signal<GatewayResult<OreBalance>> = use_context();

    let _ = use_resource(move || {
        let w = wallet();
        async move {
            let Wallet::Connected(wallet_addr) = w else {
                miner_signal.set(Err(GatewayError::AccountNotFound));
                automation_signal.set(Err(GatewayError::AccountNotFound));
                sol_signal.set(Err(GatewayError::AccountNotFound));
                ore_signal.set(Err(GatewayError::AccountNotFound));
                return;
            };

            let gateway = use_gateway();
            let pubkeys = [
                ore_api::state::miner_pda(wallet_addr).0,
                ore_api::state::automation_pda(wallet_addr).0,
                wallet_addr, // SOL balance
                get_associated_token_address(&wallet_addr, &ore_api::consts::MINT_ADDRESS),
            ];

            log::info!("Batch fetching {} user accounts", pubkeys.len());
            let accounts = match gateway.rpc.get_multiple_accounts_full(&pubkeys).await {
                Ok(accounts) => accounts,
                Err(err) => {
                    log::error!("Failed to batch fetch user accounts: {:?}", err);
                    return;
                }
            };

            miner_signal.set(try_deserialize_account::<Miner>(&accounts, 0));
            automation_signal.set(try_deserialize_account::<Automation>(&accounts, 1));

            // SOL balance — extract lamports
            sol_signal.set(
                accounts
                    .get(2)
                    .and_then(|opt| opt.as_ref())
                    .map(|account| SolBalance(account.lamports))
                    .ok_or(GatewayError::AccountNotFound),
            );

            // ORE token balance
            ore_signal.set(try_unpack_token_amount(&accounts, 3).map(OreBalance));

            log::info!("User batch init complete");
        }
    });
}
