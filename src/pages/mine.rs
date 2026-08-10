use std::str::FromStr;
use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use ore_api::{
    consts::{INTERMISSION_SLOTS, ONE_ORE, SPLIT_ADDRESS, TOKEN_DECIMALS},
    state::{Automation, Board, Miner, Round, Treasury},
};
use ore_types::response::{DeployHistoryEvent, RoundMiner};
use solana_extra_wasm::program::spl_token::{
    amount_to_ui_amount, amount_to_ui_amount_string_trimmed,
};
use solana_sdk::native_token::{lamports_to_sol, sol_to_lamports};
use solana_sdk::transaction::VersionedTransaction;
use steel::{Clock, Pubkey};

use gloo_storage::Storage;

use crate::{
    components::*,
    gateway::{entropy::EntropyGateway, ore::OreGateway, GatewayError, GatewayResult},
    hooks::{
        on_transaction_done, tip_ix, use_automation_wss, use_board_wss,
        use_cancel_transaction, use_clock_wss, use_gateway,
        use_miner_wss, use_ore_price, use_pro_deploy_transaction,
        use_reset_transaction, use_round_wss, use_sol_balance_wss, use_sol_price,
        use_topup_transaction, use_transaction_submitter, use_treasury_wss, use_wallet, GetPubkey,
        SolBalance, Wallet,
    },
    route::Route,
    utils::{format_abbreviated_pubkey, format_whole_number},
};

/// Drag selection state machine
#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum DragState {
    #[default]
    Idle,
    Pending,
    Dragging,
}

/// Whether we're selecting or deselecting during drag
#[derive(Clone, Copy, PartialEq, Default)]
enum DragAction {
    #[default]
    None,
    Selecting,
    Deselecting,
}

#[derive(Clone, Copy)]
struct GridSettings {
    show_tile_number: Signal<bool>,
    show_total_deployed: Signal<bool>,
    show_your_deployed: Signal<bool>,
    show_deploy_gauge: Signal<bool>,
    show_distribution: Signal<bool>,
    use_legacy_form: Signal<bool>,
    auto_reload: Signal<bool>,
    tile_strategy: Signal<bool>,
    randomize_tiles: Signal<bool>,
    min_motherlode_enabled: Signal<bool>,
    max_motherlode_enabled: Signal<bool>,
}

pub fn Mine() -> Element {
    let show_tile_number = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_grid_tile_number").unwrap_or(false)
    });
    let show_total_deployed = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_grid_total_deployed").unwrap_or(true)
    });
    let show_your_deployed = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_grid_your_deployed").unwrap_or(true)
    });
    let show_deploy_gauge = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_grid_deploy_gauge").unwrap_or(false)
    });
    let show_distribution = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_grid_distribution").unwrap_or(true)
    });
    let use_legacy_form = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_use_legacy_form").unwrap_or(false)
    });
    let auto_reload =
        use_signal(|| gloo_storage::LocalStorage::get::<bool>("ore_auto_reload").unwrap_or(true));
    let tile_strategy = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_tile_strategy").unwrap_or(false)
    });
    let randomize_tiles = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_randomize_tiles").unwrap_or(true)
    });
    let min_motherlode_enabled = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_min_motherlode_enabled").unwrap_or(false)
    });
    let max_motherlode_enabled = use_signal(|| {
        gloo_storage::LocalStorage::get::<bool>("ore_max_motherlode_enabled").unwrap_or(false)
    });
    use_context_provider(|| GridSettings {
        show_tile_number,
        show_total_deployed,
        show_your_deployed,
        show_deploy_gauge,
        show_distribution,
        use_legacy_form,
        auto_reload,
        tile_strategy,
        randomize_tiles,
        min_motherlode_enabled,
        max_motherlode_enabled,
    });

    let board = use_board_wss();
    let round = use_round_wss();
    let clock = use_clock_wss();
    let miner = use_miner_wss();
    let automation = use_automation_wss();
    let treasury = use_treasury_wss();
    let sol_balance = use_sol_balance_wss();
    let sol_price = use_sol_price();
    let show_settings = use_signal(|| false);
    let selected_squares = use_signal(|| [false; 25]);
    let mut revealed_squares = use_signal(|| [false; 25]);
    let var_samples = use_signal(|| None);

    // Drag selection state
    let drag_state = use_signal(|| DragState::Idle);
    let drag_action = use_signal(|| DragAction::None);
    let drag_start_time = use_signal(|| None::<f64>);
    let drag_start_square = use_signal(|| None::<u64>);
    let drag_start_pos = use_signal(|| None::<(f64, f64)>);
    let last_drag_square = use_signal(|| None::<u64>);

    let is_round_ended = use_memo(move || {
        let Ok(board) = board() else {
            return false;
        };
        let Ok(clock) = clock() else {
            return false;
        };
        clock.slot > board.end_slot
    });

    let is_seed_available = use_memo(move || {
        let Ok(board) = board() else {
            return false;
        };
        let Ok(clock) = clock() else {
            return false;
        };
        clock.slot > board.end_slot.saturating_add(4)
    });

    use_effect(move || {
        let _ = is_round_ended();
        revealed_squares.set([false; 25]);
    });

    let slot_hash = use_slot_hash(board, is_seed_available, var_samples);

    let r = use_rng(slot_hash);
    let winning_square = use_winning_square(r);
    let is_split_reward = use_is_split_reward(round, winning_square);
    let did_hit_motherlode = use_did_hit_motherlode(r);
    let motherlode_winnings = use_motherlode_winnings(treasury, round);
    let is_all_revealed = use_memo(move || revealed_squares.read().iter().all(|&x| x));
    let uncheckpointed_rewards = use_uncheckpointed_rewards(
        miner,
        round,
        winning_square,
        is_split_reward,
        did_hit_motherlode,
        motherlode_winnings,
        is_all_revealed,
    );

    rsx! {
        Row {
            class: "w-full relative",

            // Main content
            Col {
                class: "max-w-320 mx-auto w-full min-h-[calc(100vh-5rem)]",
                Col {
                    gap: 4,
                    class: "xl:flex-row xl:justify-between gap-4 xl:gap-8 w-full px-4 xl:px-8",
                    Col {
                            gap: 4,
                            class: "max-w-160 mx-auto pt-4 xl:pt-8 w-full",
                            span {
                                class: "block xl:hidden",
                                GameMeta {
                                    board: board,
                                    round: round,
                                    clock: clock,
                                    miner: miner,
                                    treasury: treasury,
                                    did_hit_motherlode: did_hit_motherlode,
                                    selected_squares: selected_squares,
                                    revealed_squares: revealed_squares,
                                    sol_price: sol_price,
                                }
                            }
                            BoardSquares {
                                board: board,
                                round: round,
                                miner: miner,
                                selected_squares: selected_squares,
                                revealed_squares: revealed_squares,
                                winning_square: winning_square,
                                drag_state: drag_state,
                                drag_action: drag_action,
                                drag_start_time: drag_start_time,
                                drag_start_square: drag_start_square,
                                drag_start_pos: drag_start_pos,
                                last_drag_square: last_drag_square,
                            }
                        }
                    Col {
                        gap: 8,
                        class: "max-w-160 xl:max-w-128 w-full mx-auto pb-4 pt-4 xl:pt-8 lg:pb-16",
                        span {
                            class: "hidden xl:block",
                            GameMeta {
                                board: board,
                                round: round,
                                clock: clock,
                                miner: miner,
                                treasury: treasury,
                                did_hit_motherlode: did_hit_motherlode,
                                selected_squares: selected_squares,
                                revealed_squares: revealed_squares,
                                sol_price: sol_price,
                            }
                        }
                        if let Ok(_) = automation() {
                            AutoplayFormV2 {
                                automation: automation,
                                sol_balance: sol_balance,
                                show_settings: show_settings,
                                winning_square: winning_square,
                                round_data: round,
                                board: board,
                                selected_squares: selected_squares,
                            }
                        } else {
                            DeployFormV2 {
                                selected_squares: selected_squares,
                                sol_balance: sol_balance,
                                miner: miner,
                                board: board,
                                clock: clock,
                                treasury: treasury,
                                round_data: round,
                                winning_square: winning_square,
                                uncheckpointed_rewards: uncheckpointed_rewards,
                                show_settings: show_settings,
                            }
                        }
                        ResetButton {
                            board: board,
                            clock: clock,
                        }
                        if !show_settings() {
                            Col {
                                gap: 0,
                                RewardsSummary {
                                    miner: miner,
                                    treasury: treasury,
                                    uncheckpointed_rewards: uncheckpointed_rewards,
                                }
                            }
                        }
                    }
                }
                Footer {}
            }
        }
    }
}

fn use_motherlode_winnings(
    treasury: Signal<GatewayResult<Treasury>>,
    round: Signal<GatewayResult<Round>>,
) -> Memo<u64> {
    use_memo(move || {
        let Ok(treasury) = treasury() else {
            return 0;
        };
        let Ok(round) = round() else {
            return 0;
        };
        treasury.motherlode.max(round.motherlode)
    })
}

fn use_uncheckpointed_rewards(
    miner: Signal<GatewayResult<Miner>>,
    round: Signal<GatewayResult<Round>>,
    winning_square: Memo<Option<u64>>,
    is_split_reward: Memo<bool>,
    did_hit_motherlode: Memo<bool>,
    motherlode_winnings: Memo<u64>,
    is_all_revealed: Memo<bool>,
) -> Memo<(u64, u64)> {
    let is_current_round = use_memo(move || {
        let Ok(miner) = miner() else {
            return false;
        };
        let Ok(round) = round() else {
            return false;
        };
        miner.round_id == round.id
    });

    let optimistic_rewards = use_memo(move || {
        if !is_current_round() {
            return (0, 0);
        }
        let Ok(miner) = miner() else {
            return (0, 0);
        };
        if miner.round_id == miner.checkpoint_id {
            return (0, 0);
        }
        let Ok(round) = round() else {
            return (0, 0);
        };
        // This is the current round (slot hash is not set yet).
        // We must optimistically calculate total winnings, top miner, and motherlode.
        let Some(winning_square) = winning_square() else {
            return (0, 0);
        };
        // Don't display rewards until all squares are revealed.
        if !is_all_revealed() {
            return (0, 0);
        }
        // Get round random number.
        let Some(r) = round.rng() else {
            return (0, 0);
        };

        // Calculate rewards.
        let mut motherlode = 0;
        let winning_square = winning_square as usize;
        let mut total_winnings = round.calculate_total_winnings(winning_square);
        let total_winnings_admin_fee = total_winnings / 100;
        total_winnings -= total_winnings_admin_fee;
        let vault_amount = total_winnings / 100;
        total_winnings -= vault_amount;
        let top_miner_reward = ONE_ORE;
        if did_hit_motherlode() {
            motherlode = motherlode_winnings();
        }
        let is_split_reward = is_split_reward();
        let top_miner_sample = round.top_miner_sample(r, winning_square);
        let is_top_miner = top_miner_sample >= miner.cumulative[winning_square]
            && top_miner_sample < miner.cumulative[winning_square] + miner.deployed[winning_square];

        // If the miner deployed 0 squares, they don't get any rewards.
        if miner.deployed[winning_square] == 0 {
            return (0, 0);
        }

        // Calculate SOL rewards
        let mut sol_rewards = ((total_winnings as u128 * miner.deployed[winning_square] as u128)
            / round.deployed[winning_square] as u128) as u64;
        sol_rewards += miner.deployed[winning_square] * 99 / 100;

        // Calculate ORE rewards
        let mut ore_rewards = 0;
        if is_split_reward {
            ore_rewards += ((top_miner_reward as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        } else if is_top_miner {
            ore_rewards += top_miner_reward;
        }

        // Calculate motherlode rewards
        if motherlode > 0 {
            ore_rewards += ((motherlode as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        }

        (sol_rewards, ore_rewards)
    });

    // Extract deduplicated u64 values from miner/round signals.
    // These memos only notify dependents when the VALUE changes,
    // not on every WSS notification that writes to the signal.
    let miner_round_id = use_memo(move || miner().ok().map(|m| m.round_id).unwrap_or(0));
    let miner_checkpoint_id = use_memo(move || miner().ok().map(|m| m.checkpoint_id).unwrap_or(0));
    let current_round_id = use_memo(move || round().ok().map(|r| r.id).unwrap_or(0));

    let past_round_rewards = use_resource(move || async move {
        // Only read deduplicated memos — these are the resource's sole reactive deps.
        let m_rid = miner_round_id();
        let r_id = current_round_id();
        let m_cid = miner_checkpoint_id();

        if m_rid == 0 || m_rid == r_id {
            return (0, 0);
        }
        if m_rid == m_cid {
            return (0, 0);
        }

        let Ok(round) = use_gateway().rpc.get_round(m_rid).await else {
            return (0, 0);
        };

        // Read miner with peek() to avoid reactive subscription.
        let Ok(miner) = miner.peek().clone() else {
            return (0, 0);
        };

        // Exit early if refunded.
        let Some(r) = round.rng() else {
            return (miner.deployed.iter().sum::<u64>(), 0);
        };

        // Get round data.
        let winning_square = round.winning_square(r);
        let total_winnings = round.total_winnings;
        let top_miner_reward = round.top_miner_reward();
        let motherlode = round.motherlode;
        let top_miner_sample = round.top_miner_sample(r, winning_square);
        let is_split_reward = round.is_split_reward_v2(winning_square);
        let is_top_miner = top_miner_sample >= miner.cumulative[winning_square]
            && top_miner_sample < miner.cumulative[winning_square] + miner.deployed[winning_square];

        // Exit early if deployed 0 squares.
        if miner.deployed[winning_square] == 0 {
            return (0, 0);
        }

        // Calculate SOL rewards
        let mut sol_rewards = ((total_winnings as u128 * miner.deployed[winning_square] as u128)
            / round.deployed[winning_square] as u128) as u64;
        sol_rewards += miner.deployed[winning_square] * 99 / 100;

        // Calculate ORE rewards
        let mut ore_rewards = 0;
        if is_split_reward {
            ore_rewards += ((top_miner_reward as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        } else if is_top_miner {
            ore_rewards += top_miner_reward;
        }

        // Calculate motherlode rewards
        if motherlode > 0 {
            ore_rewards += ((motherlode as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        }

        (sol_rewards, ore_rewards)
    });

    use_memo(move || {
        if is_current_round() {
            return optimistic_rewards();
        }
        let Some((sol_rewards, ore_rewards)) = past_round_rewards() else {
            return (0, 0);
        };
        (sol_rewards, ore_rewards)
    })
}

#[component]
pub fn GameMeta(
    board: Signal<GatewayResult<Board>>,
    round: Signal<GatewayResult<Round>>,
    clock: Signal<GatewayResult<Clock>>,
    miner: Signal<GatewayResult<Miner>>,
    treasury: Signal<GatewayResult<Treasury>>,
    did_hit_motherlode: Memo<bool>,
    selected_squares: Signal<[bool; 25]>,
    revealed_squares: Signal<[bool; 25]>,
    sol_price: Memo<Option<f64>>,
) -> Element {
    rsx! {
        Col {
            gap: 2,
            class: "w-full",

            div {
                class: "relative w-full rounded-md overflow-hidden",
                Row {
                    gap: 0,
                    class: "w-full items-center relative z-10",

                    div {
                        class: "flex-1 transition-opacity duration-200 opacity-100",
                        TotalDeployed {
                            board: board,
                            round: round,
                            sol_price: sol_price,
                        }
                    }

                    div {
                        class: "w-px h-8 bg-gray-800 flex-shrink-0"
                    }

                    div {
                        class: "flex-1 transition-opacity duration-200 opacity-100",
                        MotherlodeSize {
                            treasury: treasury,
                            did_hit_motherlode: did_hit_motherlode,
                        }
                    }

                    div {
                        class: "w-px h-8 bg-gray-800 flex-shrink-0"
                    }

                    div {
                        class: "flex-1 transition-opacity duration-200 opacity-100",
                        TimeRemaining {
                            board: board,
                            clock: clock,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RewardsSummary(
    miner: Signal<GatewayResult<Miner>>,
    treasury: Signal<GatewayResult<Treasury>>,
    uncheckpointed_rewards: Memo<(u64, u64)>,
) -> Element {
    let total_ore_rewards = use_memo(move || {
        let Ok(miner) = miner() else {
            return 0u64;
        };
        miner.rewards_ore + uncheckpointed_rewards().1
    });

    let sol_rewards = use_memo(move || {
        let Ok(miner) = miner() else {
            return 0u64;
        };
        miner.rewards_sol + uncheckpointed_rewards().0
    });

    // Don't show if no rewards
    if total_ore_rewards() == 0 && sol_rewards() == 0 {
        return rsx! {};
    }

    let rewards_str = amount_to_ui_amount_string_trimmed(total_ore_rewards(), TOKEN_DECIMALS);

    rsx! {
        Link {
            class: "relative w-full flex flex-row items-center justify-between rounded-md px-3 py-3 hover:bg-surface-elevatedHover active:ring-2 active:ring-white transition-all duration-150 cursor-pointer",
            to: Route::Rewards {},
            // Gold gradient background
            // div {
            //     class: "absolute inset-0 pointer-events-none",
            //     style: "background: linear-gradient(to right, rgba(234,179,8,0.03) 0%, rgba(234,179,8,0.08) 50%, rgba(234,179,8,0.12) 100%);",
            // }
            // Left side
            span {
                class: "relative z-10 text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide cursor-help border-b border-dashed border-elements-lowEmphasis hover:text-elements-highEmphasis hover:border-elements-highEmphasis transition-colors duration-200",
                "Rewards"
            }
            // Right side: SOL value + ORE value + chevron
            Row {
                class: "relative z-10 items-center gap-2",
                if sol_rewards() > 0 {
                    SolValue {
                        ui_amount_string: lamports_to_sol(sol_rewards()).to_string(),
                        size: Some(TokenValueSize::Medium),
                        with_decimal_units: Some(true),
                        abbreviated: Some(true),
                    }
                }
                if sol_rewards() > 0 && total_ore_rewards() > 0 {
                    span {
                        class: "text-elements-lowEmphasis text-base font-medium",
                        "+"
                    }
                }
                if total_ore_rewards() > 0 {
                    OreValue {
                        ui_amount_string: rewards_str,
                        size: Some(TokenValueSize::Medium),
                        with_decimal_units: Some(true),
                        abbreviated: Some(true),
                        gold: Some(true),
                    }
                }
                ChevronRightIcon {
                    class: "w-4 h-4 text-elements-lowEmphasis",
                }
            }
        }
    }
}

#[component]
fn RecentRoundRow(event: DeployHistoryEvent) -> Element {
    let wallet = use_wallet();

    let winning_square_display = if event.winning_square >= 0 && event.winning_square < 25 {
        format!("{}", event.winning_square + 1)
    } else {
        "–".to_string()
    };

    let is_winner = (event.mask & (1i64 << event.winning_square)) != 0;

    // Resolve top_miner: fetch from round if event has default pubkey
    let top_miner_str_clone = event.top_miner.clone();
    let round_id = event.round_id;
    let top_miner = use_resource(move || {
        let top_miner_str = top_miner_str_clone.clone();
        async move {
            if let Ok(pubkey) = top_miner_str.parse::<Pubkey>() {
                if pubkey != Pubkey::default() {
                    return pubkey;
                }
            }
            let Ok(round) = use_gateway().rpc.get_round(round_id as u64).await else {
                return Pubkey::default();
            };
            round.top_miner
        }
    });

    // Calculate ORE rewards
    let ore_rewards = use_memo(move || {
        if !is_winner || event.deployed_winning_square == 0 {
            return None;
        }

        let Some(top_miner_pubkey) = top_miner() else {
            return None;
        };

        let one_ore = 10u64.pow(TOKEN_DECIMALS as u32);
        let user_proportion = event.amount as f64 / event.deployed_winning_square as f64;

        let is_split = top_miner_pubkey == SPLIT_ADDRESS;
        let base_ore = if is_split {
            (user_proportion * one_ore as f64) as u64
        } else {
            let user_pubkey = wallet.pubkey().ok();
            if user_pubkey == Some(top_miner_pubkey) {
                one_ore
            } else {
                0u64
            }
        };

        let motherlode_share = if event.motherlode > 0 {
            (user_proportion * event.motherlode as f64) as u64
        } else {
            0u64
        };

        let total = base_ore + motherlode_share;
        if total > 0 {
            Some(total)
        } else {
            None
        }
    });

    rsx! {
        Link {
            class: "w-full flex flex-row items-center justify-between rounded-md px-3 py-3 hover:bg-surface-elevatedHover active:ring-2 active:ring-white transition-all duration-150 cursor-pointer",
            to: Route::Mine {},
            // Left side: round number
            span {
                class: "text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide",
                "#{format_whole_number(event.round_id.to_string())}"
            }
            // Right side: tile + ore rewards + chevron
            Row {
                class: "items-center gap-3",
                if event.motherlode > 0 {
                    SparklesSolidIcon {
                        class: "w-4 h-4 text-elements-gold gold-icon-shimmer",
                    }
                }
                Row {
                    class: "items-center gap-1 text-elements-lowEmphasis",
                    SquaresWinnerIcon {
                        class: "w-3.5 h-3.5",
                    }
                    span {
                        class: "text-sm font-medium",
                        "{winning_square_display}"
                    }
                }
                if let Some(ore) = ore_rewards() {
                    OreValue {
                        ui_amount_string: amount_to_ui_amount_string_trimmed(ore, TOKEN_DECIMALS),
                        with_decimal_units: Some(true),
                        abbreviated: Some(true),
                        size: Some(TokenValueSize::Medium),
                        gold: Some(true),
                    }
                } else {
                    span {
                        class: "text-elements-lowEmphasis text-sm font-medium",
                        "–"
                    }
                }
                ChevronRightIcon {
                    class: "w-4 h-4 text-elements-lowEmphasis",
                }
            }
        }
    }
}

fn use_slot_hash(
    board: Signal<GatewayResult<Board>>,
    is_seed_available: Memo<bool>,
    mut var_samples: Signal<Option<u64>>,
) -> Resource<Option<[u8; 32]>> {
    let samples = if let Some(samples) = var_samples.cloned() {
        Some(samples - 1)
    } else {
        None
    };

    let end_slot = use_memo(move || {
        let Ok(board) = board() else {
            return None;
        };
        Some(board.end_slot)
    });

    use_resource(move || async move {
        log::info!("use_slot_hash");
        if !is_seed_available() {
            return None;
        }
        log::info!("use_slot_hash: is_seed_available");
        let Some(end_slot) = end_slot() else {
            return None;
        };
        log::info!("use_slot_hash: board");
        let Ok(slot_hash) = use_gateway().rpc.get_slothash(end_slot).await else {
            log::info!("use_slot_hash: get_slothash: none");
            return None;
        };

        // get seed
        log::info!("use_slot_hash: get_seed: {:?}", slot_hash);
        let entropy_var = solana_sdk::pubkey!("BWCaDY96Xe4WkFq1M7UiCCRcChsJ3p51L5KrGzhxgm2E");
        let Ok(seed_response) = use_gateway().rpc.get_seed(entropy_var, samples).await else {
            log::info!("seed_response: none");
            return None;
        };
        log::info!("seed_response: {:?}", seed_response);
        let seed = seed_response.seed;

        // set the var samples
        var_samples.set(Some(seed_response.samples));

        // hash the slot hash, seed, and samples
        log::info!("use_slot_hash: hashv");
        let value =
            solana_sdk::keccak::hashv(&[&slot_hash, &seed, &seed_response.samples.to_le_bytes()])
                .to_bytes();

        // Hash the final sample.
        Some(value)
    })
}

fn use_did_hit_motherlode(r: Memo<Option<u64>>) -> Memo<bool> {
    use_memo(move || {
        let Some(r) = r() else {
            return false;
        };
        did_hit_motherlode(r)
    })
}

fn use_is_split_reward(
    round: Signal<GatewayResult<Round>>,
    winning_square: Memo<Option<u64>>,
) -> Memo<bool> {
    use_memo(move || {
        let Ok(round) = round() else {
            return false;
        };
        let Some(winning_square) = winning_square() else {
            return false;
        };
        round.is_split_reward_v2(winning_square as usize)
    })
}

fn use_winning_square(r: Memo<Option<u64>>) -> Memo<Option<u64>> {
    use_memo(move || {
        let Some(r) = r() else {
            return None;
        };
        Some(r % 25)
    })
}

#[component]
fn ClaimConfirmationModal(
    mut show_modal: Signal<bool>,
    claim_all_tx: Resource<GatewayResult<VersionedTransaction>>,
    rewards_sol: Memo<u64>,
    rewards_ore: Memo<u64>,
    refined_ore: Memo<u64>,
) -> Element {
    let mut is_visible = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            sleep(Duration::from_millis(10)).await;
            is_visible.set(true);
        });
    });

    let overlay_class = if is_visible() {
        "opacity-100"
    } else {
        "opacity-0"
    };

    let modal_class = if is_visible() {
        "opacity-100 scale-100"
    } else {
        "opacity-0 scale-95"
    };

    rsx! {
        Fragment {
            div {
                class: "fixed inset-0 transition-all duration-300 ease-out bg-black/50 backdrop-blur-sm z-[1000] {overlay_class}",
                style: "height: 100vh; width: 100vw;",
                onclick: move |_| {
                    show_modal.set(false);
                }
            }

            div {
                class: "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-11/12 sm:w-96 max-w-md z-[1001] transition-all duration-300 ease-out {modal_class}",
                div {
                    class: "bg-surface-elevated elevated-border rounded-md pt-6 pb-6 px-6",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    Col {
                        class: "w-full gap-2",
                        Row {
                            class: "justify-between items-center",
                            span {
                                class: "text-elements-highEmphasis font-semibold text-2xl",
                                "Claim rewards"
                            }
                        }

                        span {
                            class: "text-elements-midEmphasis font-medium",
                            "Are you sure you want to claim all your mining rewards, including refined and unrefined ORE?"
                        }

                        Col {
                            class: "w-full gap-2 py-6",
                            if rewards_sol() > 0 {
                                Row {
                                    class: "justify-between items-center",
                                    span {
                                        class: "text-elements-lowEmphasis font-medium",
                                        "SOL"
                                    }
                                    SolValue {
                                        ui_amount_string: lamports_to_sol(rewards_sol()).to_string(),
                                        with_decimal_units: true,
                                        abbreviated: false,
                                        size: Some(TokenValueSize::Medium),
                                    }
                                }
                                Row {
                                    class: "justify-between items-center",
                                    span {
                                        class: "text-elements-lowEmphasis font-medium",
                                        "Unrefined ORE"
                                    }
                                    OreValue {
                                        ui_amount_string: amount_to_ui_amount_string_trimmed(rewards_ore(), TOKEN_DECIMALS),
                                        with_decimal_units: true,
                                        abbreviated: false,
                                        size: Some(TokenValueSize::Medium),
                                        gold: false,
                                    }
                                }
                                Row {
                                    class: "justify-between items-center",
                                    span {
                                        class: "text-elements-lowEmphasis font-medium",
                                        "Refined ORE"
                                    }
                                    OreValue {
                                        ui_amount_string: amount_to_ui_amount_string_trimmed(refined_ore(), TOKEN_DECIMALS),
                                        with_decimal_units: true,
                                        abbreviated: false,
                                        size: Some(TokenValueSize::Medium),
                                        gold: false,
                                    }
                                }
                            }
                        }

                        Col {
                            class: "w-full gap-2",
                            ActionButton {
                                transaction: claim_all_tx,
                                "Claim all"
                            }
                            button {
                                class: "flex h-12 w-full rounded-full text-elements-lowEmphasis cursor-pointer hover:text-elements-highEmphasis hover:bg-controls-secondaryHover transition",
                                onclick: move |_| {
                                    show_modal.set(false);
                                },
                                span {
                                    class: "mx-auto my-auto font-semibold",
                                    "Cancel"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimeRemaining(
    board: Signal<GatewayResult<Board>>,
    clock: Signal<GatewayResult<Clock>>,
) -> Element {
    let time_remaining = use_memo(move || {
        let Ok(board) = board() else {
            return None;
        };
        let Ok(clock) = clock() else {
            return None;
        };
        if board.end_slot == u64::MAX {
            return None;
        }
        let slots_remaining = board.end_slot.saturating_sub(clock.slot);
        let secs_remaining = (slots_remaining as f64 * 0.4) as u64;
        let minutes = secs_remaining / 60;
        let seconds = secs_remaining % 60;
        Some((minutes, seconds))
    });

    let round_number = use_memo(move || {
        let Ok(board) = board() else {
            return "".to_string();
        };
        format!("#{}", format_whole_number(board.round_id.to_string()))
    });

    let is_low_time = use_memo(move || {
        time_remaining()
            .map(|(m, s)| m == 0 && s <= 10)
            .unwrap_or(false)
    });

    rsx! {
        QuickStat {
            title: "Time".to_string(),
            hover_title: round_number(),
            value: rsx! {
                span {
                    class: if is_low_time() { "text-red-400" } else { "" },
                    if let Some((minutes, seconds)) = time_remaining() {
                        "{minutes:02}:{seconds:02}"
                    } else {
                        "1:00"
                    }
                }
            },
        }
    }
}

#[component]
pub fn QuickStat(
    class: Option<String>,
    title: String,
    hover_title: Option<String>,
    value: Element,
    gold: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let _gold = gold.unwrap_or(false);
    let mut click_hover = use_signal(|| false);

    let _ = use_resource(move || async move {
        if click_hover() {
            sleep(Duration::from_secs(3)).await;
            click_hover.set(false);
        }
    });

    let border_color = "";

    let (title_class, hover_title_class) = if hover_title.is_some() {
        if click_hover() {
            ("hidden", "inline")
        } else {
            ("group-hover:hidden", "hidden group-hover:inline")
        }
    } else {
        ("", "hidden")
    };

    rsx! {
        button {
            class: "flex flex-col justify-center {border_color} {class} rounded-md w-full py-2 group",
            onclick: move |_| {
                let current = click_hover();
                click_hover.set(!current);
            },
            span {
                class: "text-elements-highEmphasis font-semibold text-xl mx-auto lg:scale-125 origin-center",
                {value}
            }
            span {
                class: "text-elements-lowEmphasis font-medium text-xs uppercase tracking-wide mx-auto text-nowrap mt-1 {title_class}",
                "{title}"
            }
            if let Some(hover_title) = hover_title {
                span {
                    class: "text-elements-lowEmphasis font-medium text-xs uppercase tracking-wide mx-auto text-nowrap mt-1 {hover_title_class}",
                    "{hover_title}"
                }
            }
        }
    }
}

#[component]
fn TotalDeployed(
    board: Signal<GatewayResult<Board>>,
    round: Signal<GatewayResult<Round>>,
    sol_price: Memo<Option<f64>>,
) -> Element {
    let total_deployed = use_memo(move || {
        let Ok(round) = round() else {
            return 0;
        };
        round.total_deployed()
    });

    let usd_value = use_memo(move || {
        let Some(sol_price) = sol_price() else {
            return 0.0;
        };
        lamports_to_sol(total_deployed()) * sol_price
    });

    let usd_title = use_memo(move || {
        let usd_value_str = format_token_amount(usd_value().to_string(), Some(false), Some(false));
        format!("≈${usd_value_str}")
    });

    rsx! {
        QuickStat {
            title: "Deployed".to_string(),
            hover_title: usd_title(),
            value: rsx! {
                SolValue {
                    ui_amount_string: format!("{:.2}", lamports_to_sol(total_deployed())),
                    with_decimal_units: true,
                    abbreviated: true,
                    size: Some(TokenValueSize::XLarge),
                }
            },
        }
    }
}

#[component]
fn MotherlodeSize(
    treasury: Signal<GatewayResult<Treasury>>,
    did_hit_motherlode: Memo<bool>,
) -> Element {
    let ore_price = use_ore_price();

    let motherlode = use_memo(move || {
        let Ok(treasury) = treasury() else {
            return 0;
        };
        treasury.motherlode
    });

    let usd_value = use_memo(move || {
        let Some(ore_price) = ore_price() else {
            return 0.0;
        };
        amount_to_ui_amount(motherlode(), TOKEN_DECIMALS) * ore_price
    });

    if did_hit_motherlode() {
        return rsx! {
            Col {
                gap: 0,
                class: "rounded-md w-full py-2 animate-pulse",
                span {
                    class: "text-elements-gold font-semibold text-lg mx-auto",
                    "Hit!"
                }
                span {
                    class: "text-elements-gold font-medium text-xs uppercase tracking-wide mx-auto text-nowrap",
                    "Motherlode"
                }
            }
        };
    }

    let usd_title = use_memo(move || {
        let usd_value_str = format_token_amount(usd_value().to_string(), Some(false), Some(false));
        format!("≈${usd_value_str}")
    });

    rsx! {
        QuickStat {
            title: "Motherlode",
            hover_title: usd_title(),
            value: rsx! {
                OreValue {
                    class: "text-elements-highEmphasis",
                    ui_amount_string: amount_to_ui_amount_string_trimmed(motherlode(), TOKEN_DECIMALS),
                    with_decimal_units: true,
                    abbreviated: true,
                    size: Some(TokenValueSize::XLarge),
                    gold: true,
                }
            },
            gold: true,
        }
    }
}

#[component]
pub fn GameStatsRow(title: String, value: String, gold: Option<bool>) -> Element {
    let gold = gold.unwrap_or(false);

    let value_color = if gold {
        "text-elements-gold"
    } else {
        "text-elements-highEmphasis"
    };

    rsx! {
        Row {
            gap: 8,
            class: "w-full justify-between",
            span {
                class: "text-elements-lowEmphasis h-min font-medium text-left",
                "{title}"
            }
            span {
                class: "{value_color} text-right font-medium",
                "{value}"
            }
        }
    }
}


#[component]
fn SelectAllButton(
    selected_squares: Signal<[bool; 25]>,
    num_selected_squares: Memo<usize>,
) -> Element {
    let highlight_class = use_memo(move || {
        if num_selected_squares() == 25 {
            "text-elements-highEmphasis border-elements-highEmphasis"
        } else {
            "border-transparent"
        }
    });

    rsx! {
        button {
            class: "flex shrink-0 py-0.5 px-1 w-14 rounded controls-secondary border-2 {highlight_class} hover:border-elements-midEmphasis hover:text-elements-highEmphasis my-auto text-xs font-semibold font-sans",
            onclick: move |_| {
                if num_selected_squares() == 25 {
                    selected_squares.set([false; 25]);
                } else {
                    selected_squares.set([true; 25]);
                }
            },
            span {
                class: "text-center mx-auto",
                "All"
            }
        }
    }
}

#[component]
fn AutoplayTabs() -> Element {
    let common_class =
        "flex-1 py-1 h-min transition-colors rounded-lg font-semibold hover:cursor-pointer text-sm";

    rsx! {
        Row {
            class: "px-2 py-2 border-b border-gray-800",
            gap: 2,
            button {
                class: "{common_class} text-elements-highEmphasis bg-controls-secondary",
                onclick: move |_|  {
                    // Noop
                },
                "Autominer"
            }
        }
    }
}

#[component]
fn QuickAddOptions(amount: Signal<String>) -> Element {
    rsx! {
        Row {
            class: "w-full justify-end ml-auto py-2",
            gap: 2,
            QuickAddButton {
                amount: 1.0,
                title: "+1".to_string(),
                value: amount,
            }
            QuickAddButton {
                amount: 0.1,
                title: "+0.1".to_string(),
                value: amount,
            }
            QuickAddButton {
                amount: 0.01,
                title: "+0.01".to_string(),
                value: amount,
            }
        }
    }
}

#[component]
fn QuickAddButton(amount: f64, title: String, mut value: Signal<String>) -> Element {
    rsx! {
        button {
            class: "flex py-0.5 px-1 w-14 rounded controls-secondary border-2 border-transparent hover:border-elements-midEmphasis hover:text-elements-highEmphasis my-auto text-xs font-semibold font-sans",
            onclick: move |_| {
                let current_value = value.read().parse::<f64>().unwrap_or(0.0);
                let new_value_f64 = (current_value * 100.0 + amount * 100.0) / 100.0; // Avoid float precision issues
                let new_value_str = format!("{:.2}", new_value_f64).trim_end_matches('0').trim_end_matches('.').to_string(); // Format to 2 decimal places
                value.set(new_value_str);
            },
            span {
                class: "text-center mx-auto",
                "{title}"
            }
        }
    }
}

#[component]
fn ResetButton(
    board: Signal<GatewayResult<Board>>,
    clock: Signal<GatewayResult<Clock>>,
) -> Element {
    let tx = use_reset_transaction();

    let needs_reset = use_memo(move || {
        let Ok(board) = board() else {
            return false;
        };
        let Ok(clock) = clock() else {
            return false;
        };
        if board.end_slot == u64::MAX {
            return false;
        }
        clock.slot >= board.end_slot + INTERMISSION_SLOTS + 25 // buffer of 10 seconds
    });

    rsx! {
        if needs_reset() {
            ActionButton {
                transaction: tx,
                "Reset"
            }
        }
    }
}

#[component]
fn BoardSquares(
    board: Signal<GatewayResult<Board>>,
    round: Signal<GatewayResult<Round>>,
    miner: Signal<GatewayResult<Miner>>,
    mut selected_squares: Signal<[bool; 25]>,
    revealed_squares: Signal<[bool; 25]>,
    winning_square: Memo<Option<u64>>,
    mut drag_state: Signal<DragState>,
    mut drag_action: Signal<DragAction>,
    mut drag_start_time: Signal<Option<f64>>,
    mut drag_start_square: Signal<Option<u64>>,
    mut drag_start_pos: Signal<Option<(f64, f64)>>,
    mut last_drag_square: Signal<Option<u64>>,
) -> Element {
    // Diagonal wave reveal: all tiles revealed at once, CSS transition-delay creates the wave
    let _ = use_resource(move || async move {
        let Some(winning_square) = winning_square() else {
            return;
        };
        // Reveal all non-winning tiles simultaneously; per-tile CSS transition-delay handles the wave
        {
            let mut squares = revealed_squares.write();
            for i in 0..25 {
                if i != winning_square as usize {
                    squares[i] = true;
                }
            }
        }
        // Wait for wave to complete (8 diagonals * 30ms + 300ms transition), then reveal winner
        sleep(Duration::from_millis(550)).await;
        revealed_squares.write()[winning_square as usize] = true;
    });

    // Constants for drag detection
    const DRAG_DELAY_MS: f64 = 150.0;
    const SCROLL_THRESHOLD_PX: f64 = 20.0;

    // Helper to get square ID from coordinates using elementFromPoint.
    // Walks up the DOM tree to find the nearest ancestor with data-square-id,
    // since elementFromPoint may return a child element inside the button.
    let get_square_at_point = |x: f64, y: f64| -> Option<u64> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let mut el: Option<web_sys::Element> = document.element_from_point(x as f32, y as f32);
        while let Some(element) = el {
            if let Some(id_str) = element.get_attribute("data-square-id") {
                return id_str.parse().ok();
            }
            el = element.parent_element();
        }
        None
    };

    // Helper to apply drag action to a square
    let mut apply_action = move |square_id: u64| {
        let mut squares = selected_squares.cloned();
        match drag_action() {
            DragAction::Selecting => {
                if !squares[square_id as usize] {
                    squares[square_id as usize] = true;
                    selected_squares.set(squares);
                }
            }
            DragAction::Deselecting => {
                if squares[square_id as usize] {
                    squares[square_id as usize] = false;
                    selected_squares.set(squares);
                }
            }
            DragAction::None => {}
        }
    };

    // Helper to reset drag state
    let mut reset_drag_state = move || {
        drag_state.set(DragState::Idle);
        drag_action.set(DragAction::None);
        drag_start_time.set(None);
        drag_start_square.set(None);
        drag_start_pos.set(None);
        last_drag_square.set(None);
    };

    // Handle touch move on the board container (for cross-square detection)
    let handle_touch_move = move |e: Event<TouchData>| {
        let touches = e.touches();
        let Some(touch) = touches.first() else { return };
        let coords = touch.client_coordinates();
        let current_pos = (coords.x, coords.y);

        match drag_state() {
            DragState::Pending => {
                let elapsed = drag_start_time()
                    .map(|t| js_sys::Date::now() - t)
                    .unwrap_or(0.0);

                if elapsed < DRAG_DELAY_MS {
                    // Check if user moved significantly (scrolling)
                    if let Some((start_x, start_y)) = drag_start_pos() {
                        let dx = (current_pos.0 - start_x).abs();
                        let dy = (current_pos.1 - start_y).abs();
                        if dx > SCROLL_THRESHOLD_PX || dy > SCROLL_THRESHOLD_PX {
                            // User is scrolling - cancel and reset
                            reset_drag_state();
                            return;
                        }
                    }
                    return;
                }

                // 150ms elapsed - prevent scroll from this point on
                e.prevent_default();

                // Check if moved to different square
                let current_square = get_square_at_point(current_pos.0, current_pos.1);
                if current_square != drag_start_square() {
                    if let (Some(start_sq), Some(curr_sq)) = (drag_start_square(), current_square) {
                        // Transition: PENDING → DRAGGING
                        drag_state.set(DragState::Dragging);

                        // Apply action to both initial and current square
                        apply_action(start_sq);
                        apply_action(curr_sq);
                        last_drag_square.set(Some(curr_sq));
                    }
                }
            }
            DragState::Dragging => {
                e.prevent_default();
                if let Some(square_id) = get_square_at_point(current_pos.0, current_pos.1) {
                    // Skip if still on the same square
                    if last_drag_square() == Some(square_id) {
                        return;
                    }
                    last_drag_square.set(Some(square_id));
                    apply_action(square_id);
                }
            }
            DragState::Idle => {}
        }
    };

    // Handle touch end - toggle on tap, or end drag
    let handle_touch_end = move |e: Event<TouchData>| {
        match drag_state() {
            DragState::Pending => {
                // User tapped and released - check if in same square
                // Use changedTouches for touchend since touches is empty
                if let Some(touch) = e.touches().first().or_else(|| {
                    // Fallback: use drag_start_square since we can't get end position easily
                    None
                }) {
                    let coords = touch.client_coordinates();
                    let end_square = get_square_at_point(coords.x, coords.y);
                    if end_square == drag_start_square() {
                        // Normal tap-to-toggle
                        if let Some(sq) = drag_start_square() {
                            let is_any_revealed = revealed_squares.read().iter().any(|&x| x);
                            if !is_any_revealed {
                                let mut squares = selected_squares.cloned();
                                squares[sq as usize] = !squares[sq as usize];
                                selected_squares.set(squares);
                            }
                        }
                    }
                } else {
                    // Fallback: just toggle the start square (tap behavior)
                    if let Some(sq) = drag_start_square() {
                        let is_any_revealed = revealed_squares.read().iter().any(|&x| x);
                        if !is_any_revealed {
                            let mut squares = selected_squares.cloned();
                            squares[sq as usize] = !squares[sq as usize];
                            selected_squares.set(squares);
                        }
                    }
                }
            }
            DragState::Dragging => {
                // Drag complete, nothing more to do
            }
            DragState::Idle => {}
        }
        reset_drag_state();
    };

    // Handle touch cancel - just reset
    let handle_touch_cancel = move |_: Event<TouchData>| {
        reset_drag_state();
    };

    let distribution_mask = use_memo(move || {
        let Ok(round) = round() else {
            return 0u32;
        };
        round.distribution_mask()
    });

    rsx! {
        Col {
            gap: 4,
            class: "w-full tabular-numbers",
            div {
                class: "mx-auto w-full grid grid-cols-5 grid-rows-5 gap-1 sm:gap-2 xl:max-h-[calc(100vh-10rem)] xl:max-w-[calc(100vh-10rem)] xl:min-h-80 xl:min-w-80",
                ontouchmove: handle_touch_move,
                ontouchend: handle_touch_end,
                ontouchcancel: handle_touch_cancel,
                for i in 0..5 {
                    for j in 0..5 {
                        BoardSquare {
                            id: i * 5 + j,
                            board: board,
                            miner: miner,
                            round: round,
                            selected_squares: selected_squares,
                            revealed_squares: revealed_squares,
                            winning_square: winning_square,
                            distribution_mask: distribution_mask,
                            drag_state: drag_state,
                            drag_action: drag_action,
                            drag_start_time: drag_start_time,
                            drag_start_square: drag_start_square,
                            drag_start_pos: drag_start_pos,
                            last_drag_square: last_drag_square,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BoardSquare(
    id: u64,
    board: Signal<GatewayResult<Board>>,
    miner: Signal<GatewayResult<Miner>>,
    round: Signal<GatewayResult<Round>>,
    mut selected_squares: Signal<[bool; 25]>,
    revealed_squares: Signal<[bool; 25]>,
    winning_square: Memo<Option<u64>>,
    distribution_mask: Memo<u32>,
    mut drag_state: Signal<DragState>,
    mut drag_action: Signal<DragAction>,
    mut drag_start_time: Signal<Option<f64>>,
    mut drag_start_square: Signal<Option<u64>>,
    mut drag_start_pos: Signal<Option<(f64, f64)>>,
    mut last_drag_square: Signal<Option<u64>>,
) -> Element {
    let grid_settings: GridSettings = use_context();
    let show_tile_number = grid_settings.show_tile_number;
    let show_total_deployed = grid_settings.show_total_deployed;
    let show_your_deployed = grid_settings.show_your_deployed;
    let show_deploy_gauge = grid_settings.show_deploy_gauge;
    let show_distribution = grid_settings.show_distribution;

    let is_selected = use_memo(move || selected_squares.read()[id as usize]);

    let is_revealed = use_memo(move || revealed_squares.read()[id as usize]);

    let is_any_revealed = use_memo(move || revealed_squares.read().iter().any(|&x| x));

    let is_revealed_and_winning = use_memo(move || is_revealed() && winning_square() == Some(id));

    let miner_deployed = use_memo(move || {
        let Ok(miner) = miner() else {
            log::warn!("Failed to get miner");
            return 0;
        };
        let Ok(board) = board() else {
            log::warn!("Failed to get board");
            return 0;
        };
        if miner.round_id != board.round_id {
            return 0;
        }
        miner.deployed[id as usize]
    });

    let square_deployed = use_memo(move || {
        let Ok(board) = board() else {
            return 0;
        };
        let Ok(round) = round() else {
            return 0;
        };
        if board.round_id != round.id {
            return 0;
        }
        round.deployed[id as usize]
    });

    let is_winner_take_all = use_memo(move || (distribution_mask() & (1u32 << id)) != 0);

    let border_color = use_memo(move || {
        if winning_square().is_none() {
            if miner_deployed() > 0 {
                if is_selected() {
                    "enabled:hover:bg-surface-floatingHover enabled:hover:cursor-pointer border-elements-highEmphasis inset-ring lg:inset-ring-2 inset-ring-blue-500 bg-blue-500/10"
                } else {
                    "enabled:hover:bg-surface-floatingHover enabled:hover:cursor-pointer border-blue-500 bg-blue-500/10"
                }
            } else {
                if is_selected() {
                    "enabled:hover:bg-surface-floatingHover enabled:hover:cursor-pointer border-elements-highEmphasis inset-ring lg:inset-ring-2 inset-ring-elements-highEmphasis"
                } else {
                    "enabled:hover:bg-surface-floatingHover enabled:hover:cursor-pointer border-gray-700 enabled:hover:border-elements-highEmphasis/50"
                }
            }
        } else if is_revealed_and_winning() {
            "border-elements-gold inset-ring lg:inset-ring-2 inset-ring-elements-gold"
        } else if is_revealed() {
            "border-transparent"
        } else if miner_deployed() > 0 {
            "border-blue-500"
        } else {
            "border-gray-700"
        }
    });

    let opacity = use_memo(move || {
        if is_revealed_and_winning() {
            "opacity-100"
        } else if is_revealed() {
            "opacity-10"
        } else {
            "opacity-100"
        }
    });

    let id_color = use_memo(move || {
        if is_revealed_and_winning() {
            "text-elements-gold"
        } else {
            "text-elements-lowEmphasis"
        }
    });

    let wta_color = use_memo(move || {
        if is_revealed_and_winning() {
            "text-elements-gold"
        } else {
            "text-elements-midEmphasis"
        }
    });

    // Constants for drag detection
    const DRAG_DELAY_MS: f64 = 150.0;

    // Helper to reset drag state
    let mut reset_drag_state = move || {
        drag_state.set(DragState::Idle);
        drag_action.set(DragAction::None);
        drag_start_time.set(None);
        drag_start_square.set(None);
        drag_start_pos.set(None);
        last_drag_square.set(None);
    };

    // Helper to apply drag action
    let mut apply_action = move |square_id: u64| {
        let mut squares = selected_squares.cloned();
        match drag_action() {
            DragAction::Selecting => {
                if !squares[square_id as usize] {
                    squares[square_id as usize] = true;
                    selected_squares.set(squares);
                }
            }
            DragAction::Deselecting => {
                if squares[square_id as usize] {
                    squares[square_id as usize] = false;
                    selected_squares.set(squares);
                }
            }
            DragAction::None => {}
        }
    };

    // Handle touch start - transition to PENDING
    let handle_touch_start = move |e: Event<TouchData>| {
        if is_any_revealed() || drag_state() != DragState::Idle {
            return;
        }

        if let Some(touch) = e.touches().first() {
            let coords = touch.client_coordinates();
            drag_state.set(DragState::Pending);
            drag_start_time.set(Some(js_sys::Date::now()));
            drag_start_square.set(Some(id));
            drag_start_pos.set(Some((coords.x, coords.y)));

            // Determine action based on current square state
            let current_selected = selected_squares.read()[id as usize];
            drag_action.set(if current_selected {
                DragAction::Deselecting
            } else {
                DragAction::Selecting
            });
        }
        // DON'T prevent default - allow scroll to work initially
    };

    // Handle mouse down - transition to PENDING (desktop)
    let handle_mouse_down = move |e: Event<MouseData>| {
        if is_any_revealed() || drag_state() != DragState::Idle {
            return;
        }

        let coords = e.client_coordinates();
        drag_state.set(DragState::Pending);
        drag_start_time.set(Some(js_sys::Date::now()));
        drag_start_square.set(Some(id));
        drag_start_pos.set(Some((coords.x, coords.y)));

        // Determine action based on current square state
        let current_selected = selected_squares.read()[id as usize];
        drag_action.set(if current_selected {
            DragAction::Deselecting
        } else {
            DragAction::Selecting
        });
    };

    // Handle mouse enter - for mouse drag detection
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if drag_state() != DragState::Pending && drag_state() != DragState::Dragging {
            return;
        }

        let elapsed = drag_start_time()
            .map(|t| js_sys::Date::now() - t)
            .unwrap_or(0.0);

        if drag_state() == DragState::Pending && elapsed >= DRAG_DELAY_MS {
            // Transition to DRAGGING
            drag_state.set(DragState::Dragging);

            // Apply action to start square first
            if let Some(start_sq) = drag_start_square() {
                apply_action(start_sq);
            }
            // Apply action to this square
            apply_action(id);
        } else if drag_state() == DragState::Dragging {
            apply_action(id);
        }
    };

    // Handle mouse up - toggle on tap or end drag (desktop)
    let handle_mouse_up = move |_: Event<MouseData>| {
        match drag_state() {
            DragState::Pending => {
                // This was a tap (no drag) - toggle the square
                if !is_any_revealed() {
                    let mut squares = selected_squares.cloned();
                    squares[id as usize] = !squares[id as usize];
                    selected_squares.set(squares);
                }
            }
            DragState::Dragging => {}
            DragState::Idle => {}
        }
        reset_drag_state();
    };

    // Handle mouse leave the board area - end drag if needed
    let handle_mouse_leave = move |_: Event<MouseData>| {
        // Don't reset here - allow drag to continue when re-entering
    };

    // Handle touch end - toggle on tap or end drag (mobile)
    let handle_touch_end = move |e: Event<TouchData>| {
        // Stop propagation so container's handler doesn't also fire
        e.stop_propagation();
        // Prevent synthetic mouse events from firing after touch
        e.prevent_default();

        match drag_state() {
            DragState::Pending => {
                // This was a tap (no drag) - toggle the square
                // Only toggle if this is the square where touch started
                if drag_start_square() == Some(id) && !is_any_revealed() {
                    let mut squares = selected_squares.cloned();
                    squares[id as usize] = !squares[id as usize];
                    selected_squares.set(squares);
                }
            }
            DragState::Dragging => {}
            DragState::Idle => {}
        }
        reset_drag_state();
    };

    let gauge_pct = use_memo(move || {
        let Ok(r) = round() else { return 0.0f64 };
        let Ok(b) = board() else { return 0.0 };
        if r.id != b.round_id {
            return 0.0;
        }
        let max = r.deployed.iter().copied().max().unwrap_or(0) as f64;
        let min = r.deployed.iter().copied().min().unwrap_or(0) as f64;
        if max == min {
            return 0.0;
        }
        let tile = r.deployed[id as usize] as f64;
        ((tile - min) / (max - min)) * 100.0
    });

    // Diagonal-based transition delay for smooth wave reveal
    let reveal_delay = use_memo(move || {
        if winning_square().is_some() {
            let row = id as usize / 5;
            let col = id as usize % 5;
            let diag = row + col; // 0 (top-left) to 8 (bottom-right)
            let delay_ms = diag * 30;
            format!("transition-delay: {delay_ms}ms;")
        } else {
            String::new()
        }
    });

    let gauge_style = use_memo(move || {
        let pct = gauge_pct();
        format!("position:absolute;bottom:0;left:0;right:0;height:{pct}%;background:linear-gradient(to top, rgba(59,130,246,0.03) 0%, rgba(59,130,246,0.12) 100%);border-radius:0 0 0.375rem 0.375rem;pointer-events:none;transition:height 0.5s ease-in-out;")
    });

    rsx! {
        button {
            class: "relative flex flex-col box-content aspect-square text-xs md:text-sm lg:text-sm xl:text-base size-auto rounded-md border-2 {border_color} p-1 lg:p-2 disabled:hover:cursor-not-allowed disabled:opacity-50 overflow-hidden enabled:hover:scale-[1.05] enabled:hover:z-10",
            style: "{reveal_delay} transition: scale 100ms ease-out, transform 100ms ease-out, background-color 300ms, border-color 300ms;",
            "data-square-id": "{id}",
            ontouchstart: handle_touch_start,
            ontouchend: handle_touch_end,
            onmousedown: handle_mouse_down,
            onmouseup: handle_mouse_up,
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            if show_deploy_gauge() {
                div {
                    class: "{opacity} transition-opacity duration-300",
                    style: "{gauge_style()} {reveal_delay}",
                }
            }
            div {
                class: "flex flex-col flex-1 w-full {opacity} transition-opacity duration-300",
                style: "{reveal_delay}",
                Row {
                    class: "relative z-10 w-full justify-between",
                    if show_tile_number() {
                        span {
                            class: "mb-auto text-nowrap text-xs font-medium {id_color} transition-colors duration-200",
                            "#{id + 1}"
                        }
                    } else {
                        span { class: "mb-auto" }
                    }
                    if show_distribution() && is_winner_take_all() {
                        StarFourIcon {
                            class: "w-2.5 h-2.5 sm:w-3 sm:h-3 mb-auto mt-0.5 mr-0.5 {wta_color}",
                        }
                    }
                }
                Col {
                    class: "relative z-10 mt-auto ml-auto",
                    if show_your_deployed() && miner_deployed() > 0 {
                        Row {
                            class: "ml-auto items-center gap-1 text-blue-500 font-medium",
                            img {
                                src: asset!("/assets/solana.png"),
                                class: "w-2.5 h-2.5 md:w-3 md:h-3",
                            }
                            span {
                                "{display_sol_3dp(miner_deployed())}"
                            }
                        }
                    }
                    if show_total_deployed() && square_deployed() > 0 {
                        Row {
                            class: "ml-auto items-center gap-1 text-elements-highEmphasis font-medium",
                            img {
                                src: asset!("/assets/solana.png"),
                                class: "w-2.5 h-2.5 md:w-3 md:h-3",
                            }
                            span {
                                "{display_sol_3dp(square_deployed())}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn display_sol(lamports: u64) -> String {
    let sol = lamports_to_sol(lamports);

    // Count the number of zeros after the decimal point
    let mut num_zeros = 0;
    let decimal_str = format!("{:.10}", sol);
    let after_decimal = decimal_str.split('.').nth(1).unwrap_or("");
    for c in after_decimal.chars() {
        if c == '0' {
            num_zeros += 1;
        } else {
            break;
        }
    }

    // Get the significant figures after the decimal point
    let big_units = decimal_str.split('.').nth(0).unwrap_or("0");
    let sig_figs = decimal_str
        .split('.')
        .nth(1)
        .unwrap_or("")
        .trim_start_matches('0');

    // Format the balance with subscripts for zeros
    let mut display_sol = if num_zeros == 9 {
        format!("{}.0₈{}", big_units, sig_figs)
    } else if num_zeros == 8 {
        format!("{}.0₇{}", big_units, sig_figs)
    } else if num_zeros == 7 {
        format!("{}.0₆{}", big_units, sig_figs)
    } else if num_zeros == 6 {
        format!("{}.0₅{}", big_units, sig_figs)
    } else if num_zeros == 5 {
        format!("{}.0₄{}", big_units, sig_figs)
    } else if num_zeros == 4 {
        format!("{}.0₃{}", big_units, sig_figs)
    } else if num_zeros == 3 {
        format!("{}.000{}", big_units, &sig_figs[..(1.min(sig_figs.len()))])
    } else if num_zeros == 2 {
        format!("{}.00{}", big_units, &sig_figs[..(2.min(sig_figs.len()))])
    } else if num_zeros == 1 {
        format!("{}.0{}", big_units, &sig_figs[..(3.min(sig_figs.len()))])
    } else {
        sol.to_string()
    };

    // Trim to only 4 characters after the decimal point
    if let Some(_) = display_sol.find('.') {
        let units: Vec<_> = display_sol.split('.').collect();
        let big_units = units[0].to_string();
        let mut small_units = units[1].to_string();
        let truc = if num_zeros > 3 { 6 } else { 4 };
        small_units.truncate(truc);
        small_units = small_units.trim_end_matches('0').to_string();
        display_sol = format!("{}.{}", big_units, small_units);
    }

    // Trim trailing zeros and decimal point
    display_sol = display_sol.trim_end_matches('.').to_string();

    // If the display sol is empty, return 0
    if display_sol.is_empty() {
        "0".to_string()
    } else {
        display_sol
    }
}

/// Format SOL for miners table: 3dp, subscript at 3+ leading zeros
fn display_sol_miner(lamports: u64) -> String {
    let sol = lamports_to_sol(lamports);
    if sol == 0.0 {
        return "0".to_string();
    }

    let decimal_str = format!("{:.10}", sol);
    let big = decimal_str.split('.').next().unwrap_or("0");
    let after = decimal_str.split('.').nth(1).unwrap_or("");

    let mut num_zeros = 0;
    for c in after.chars() {
        if c == '0' {
            num_zeros += 1;
        } else {
            break;
        }
    }

    let sig_figs: String = after.trim_start_matches('0').chars().take(1).collect();

    if num_zeros >= 3 && big == "0" {
        let subscript = match num_zeros {
            3 => "₂",
            4 => "₃",
            5 => "₄",
            6 => "₅",
            7 => "₆",
            8 => "₇",
            _ => "₈",
        };
        format!("{}.0{}{}", big, subscript, sig_figs)
    } else {
        format!("{:.3}", sol)
    }
}

fn display_sol_3dp(lamports: u64) -> String {
    let sol = lamports_to_sol(lamports);
    // If 3 decimal places would show 0.000, use subscript notation
    if sol > 0.0 && sol < 0.001 {
        return display_sol(lamports);
    }
    format!("{:.3}", sol)
}

#[test]
fn test_display_sol() {
    assert_eq!(display_sol(sol_to_lamports(0.000001)), "0.0₄1");
    assert_eq!(display_sol(sol_to_lamports(0.0000012)), "0.0₄12");
    assert_eq!(display_sol(sol_to_lamports(0.00000123)), "0.0₄12");
    assert_eq!(display_sol(sol_to_lamports(0.00001)), "0.0₃1");
    assert_eq!(display_sol(sol_to_lamports(0.000012)), "0.0₃12");
    assert_eq!(display_sol(sol_to_lamports(0.0000123)), "0.0₃12");
    assert_eq!(display_sol(sol_to_lamports(0.0001)), "0.0001");
    assert_eq!(display_sol(sol_to_lamports(0.00012)), "0.0001");
    assert_eq!(display_sol(sol_to_lamports(0.001)), "0.001");
    assert_eq!(display_sol(sol_to_lamports(0.0012)), "0.0012");
    assert_eq!(display_sol(sol_to_lamports(0.00123)), "0.0012");
    assert_eq!(display_sol(sol_to_lamports(0.01)), "0.01");
    assert_eq!(display_sol(sol_to_lamports(0.1)), "0.1");
    assert_eq!(display_sol(sol_to_lamports(1.0)), "1");
    assert_eq!(display_sol(sol_to_lamports(1.1)), "1.1");
    assert_eq!(display_sol(sol_to_lamports(1.01)), "1.01");
    assert_eq!(display_sol(sol_to_lamports(10.0)), "10");
    assert_eq!(display_sol(sol_to_lamports(100.0)), "100");
    assert_eq!(display_sol(sol_to_lamports(1000.0)), "1000");
    assert_eq!(display_sol(sol_to_lamports(10000.0)), "10000");
    assert_eq!(display_sol(sol_to_lamports(10000.01)), "10000.01");
    assert_eq!(display_sol(sol_to_lamports(10000.000001)), "10000.0₄1");
    assert_eq!(display_sol(sol_to_lamports(10000.0000012)), "10000.0₄12");
    assert_eq!(display_sol(sol_to_lamports(10000.00000123)), "10000.0₄12");
}

fn use_rng(slot_hash: Resource<Option<[u8; 32]>>) -> Memo<Option<u64>> {
    use_memo(move || {
        log::info!("use_rng");
        let Some(Some(slot_hash)) = slot_hash() else {
            log::info!("use_rng: NONE");
            return None;
        };
        log::info!("use_rng: SOME");
        let r1 = u64::from_le_bytes(slot_hash[0..8].try_into().unwrap());
        let r2 = u64::from_le_bytes(slot_hash[8..16].try_into().unwrap());
        let r3 = u64::from_le_bytes(slot_hash[16..24].try_into().unwrap());
        let r4 = u64::from_le_bytes(slot_hash[24..32].try_into().unwrap());
        Some(r1 ^ r2 ^ r3 ^ r4)
    })
}

fn did_hit_motherlode(r: u64) -> bool {
    r.reverse_bits() % 500 == 0
}

// ---------------------------------------------------------------------------
// DeployFormV2 – total-SOL-first deploy form
// ---------------------------------------------------------------------------

#[component]
fn DeployFormV2Tabs() -> Element {
    rsx! {
        Row {
            class: "justify-center",
            gap: 2,
            span {
                class: "px-4 py-1.5 text-sm font-semibold text-elements-highEmphasis",
                "Mine"
            }
        }
    }
}

#[component]
fn DeployFormV2QuickButtons(
    mut amount: Signal<String>,
    sol_balance: Signal<GatewayResult<SolBalance>>,
) -> Element {
    let btn_class = "flex-1 py-2 text-sm font-semibold rounded-full cursor-pointer text-elements-midEmphasis hover:text-elements-highEmphasis bg-surface-floating hover:bg-surface-floatingControl border-2 border-transparent active:border-white transition-all duration-150";

    rsx! {
        Row {
            class: "w-full justify-center",
            gap: 2,
            button {
                class: "{btn_class}",
                onclick: move |_| {
                    let current = amount().parse::<f64>().unwrap_or(0.0);
                    let new_val = (current * 100.0 + 1.0) / 100.0;
                    let s = format!("{:.2}", new_val).trim_end_matches('0').trim_end_matches('.').to_string();
                    amount.set(s);
                },
                "+0.01"
            }
            button {
                class: "{btn_class}",
                onclick: move |_| {
                    let current = amount().parse::<f64>().unwrap_or(0.0);
                    let new_val = (current * 10.0 + 1.0) / 10.0;
                    let s = format!("{:.2}", new_val).trim_end_matches('0').trim_end_matches('.').to_string();
                    amount.set(s);
                },
                "+0.1"
            }
            button {
                class: "{btn_class}",
                onclick: move |_| {
                    let current = amount().parse::<f64>().unwrap_or(0.0);
                    let new_val = current + 1.0;
                    let s = format!("{:.2}", new_val).trim_end_matches('0').trim_end_matches('.').to_string();
                    amount.set(s);
                },
                "+1"
            }
            button {
                class: "{btn_class}",
                onclick: move |_| {
                    let balance = sol_balance().unwrap_or_default().0;
                    let max_sol = lamports_to_sol(balance.saturating_sub(5000));
                    if max_sol > 0.0 {
                        let s = format!("{:.4}", max_sol).trim_end_matches('0').trim_end_matches('.').to_string();
                        amount.set(s);
                    }
                },
                "MAX"
            }
        }
    }
}

#[component]
fn DeployFormV2(
    selected_squares: Signal<[bool; 25]>,
    sol_balance: Signal<GatewayResult<SolBalance>>,
    miner: Signal<GatewayResult<Miner>>,
    board: Signal<GatewayResult<Board>>,
    clock: Signal<GatewayResult<Clock>>,
    treasury: Signal<GatewayResult<Treasury>>,
    round_data: Signal<GatewayResult<Round>>,
    winning_square: Memo<Option<u64>>,
    uncheckpointed_rewards: Memo<(u64, u64)>,
    mut show_settings: Signal<bool>,
) -> Element {
    let wallet = use_wallet();
    let grid_settings: GridSettings = use_context();
    let mut amount = use_signal(|| "".to_owned());
    let mut rounds_input = use_signal(|| "".to_owned());
    let mut territories = use_signal(|| "".to_owned());

    let num_selected_squares =
        use_memo(move || selected_squares.read().iter().filter(|&&x| x).count());

    let mut solo_tiles_count = use_signal(|| 0u64);
    let mut split_tiles_count = use_signal(|| 0u64);
    let tile_strategy_enabled = grid_settings.tile_strategy;
    let effective_solo_tiles = use_memo(move || {
        if tile_strategy_enabled() {
            solo_tiles_count() as u16
        } else {
            0u16
        }
    });
    let effective_split_tiles = use_memo(move || {
        if tile_strategy_enabled() {
            split_tiles_count() as u16
        } else {
            0u16
        }
    });

    // Motherlode condition signals
    let min_motherlode_enabled = grid_settings.min_motherlode_enabled;
    let max_motherlode_enabled = grid_settings.max_motherlode_enabled;
    let mut min_motherlode_value = use_signal(|| 0u16);
    let mut max_motherlode_value = use_signal(|| 0u16);
    let effective_min_motherlode = use_memo(move || {
        if min_motherlode_enabled() {
            min_motherlode_value()
        } else {
            0u16
        }
    });
    let effective_max_motherlode = use_memo(move || {
        if max_motherlode_enabled() {
            max_motherlode_value()
        } else {
            u16::MAX
        }
    });

    // Distribution mask and tile lists (needed for tile strategy sync)
    let distribution_mask = use_memo(move || {
        let Ok(round) = round_data() else {
            return 0u32;
        };
        round.distribution_mask()
    });

    let split_tile_indices = use_memo(move || {
        let mask = distribution_mask();
        (0..25u64)
            .filter(|&i| (mask & (1u32 << i)) == 0)
            .collect::<Vec<u64>>()
    });

    let solo_tile_indices = use_memo(move || {
        let mask = distribution_mask();
        (0..25u64)
            .filter(|&i| (mask & (1u32 << i)) != 0)
            .collect::<Vec<u64>>()
    });

    // When the round changes (distribution_mask changes), re-select tiles to match form counts.
    // Uses peek() for counts so this only re-runs on mask changes, not count changes.
    let mut prev_mask = use_signal(|| 0u32);
    use_effect(move || {
        let mask = distribution_mask(); // subscribe to mask changes only
        if mask == prev_mask() {
            return;
        }
        prev_mask.set(mask);
        if !tile_strategy_enabled() {
            return;
        }
        let target_solo = solo_tiles_count.peek().clone() as usize;
        let target_split = split_tiles_count.peek().clone() as usize;
        if target_solo == 0 && target_split == 0 {
            return;
        }
        let solos = solo_tile_indices();
        let splits = split_tile_indices();
        let mut squares = [false; 25];
        // Select random solo tiles up to the target count
        let mut solo_pool = solos.clone();
        let mut count = 0;
        while count < target_solo && !solo_pool.is_empty() {
            let idx_pos = rand::random::<usize>() % solo_pool.len();
            squares[solo_pool.remove(idx_pos) as usize] = true;
            count += 1;
        }
        // Select random split tiles up to the target count
        let mut split_pool = splits.clone();
        count = 0;
        while count < target_split && !split_pool.is_empty() {
            let idx_pos = rand::random::<usize>() % split_pool.len();
            squares[split_pool.remove(idx_pos) as usize] = true;
            count += 1;
        }
        selected_squares.set(squares);
    });

    // When user taps a tile on the board, sync solo/split counts back to form
    use_effect(move || {
        if !tile_strategy_enabled() {
            return;
        }
        let squares = selected_squares();
        let mask = distribution_mask();
        let mut solo = 0u64;
        let mut split = 0u64;
        for i in 0..25 {
            if squares[i] {
                if (mask & (1u32 << i)) != 0 {
                    solo += 1;
                } else {
                    split += 1;
                }
            }
        }
        if solo != solo_tiles_count() {
            solo_tiles_count.set(solo);
        }
        if split != split_tiles_count() {
            split_tiles_count.set(split);
        }
    });

    use_effect(move || {
        if num_selected_squares() > 0 {
            territories.set(num_selected_squares().to_string());
        } else {
            territories.set("".to_owned());
        }
    });

    let num_squares = use_memo(move || territories().parse::<u64>().unwrap_or(0));

    // Rounds stepper steps: 1, 2, 5, 10, 15, 20
    let rounds_steps: Vec<u64> = vec![1, 2, 5, 10, 15, 20, 25, 30, 40, 50, 75, 100];
    let rounds_steps_clone = rounds_steps.clone();

    let mut rounds_step_up = move || {
        let current = rounds_input().parse::<u64>().unwrap_or(0);
        let next = rounds_steps
            .iter()
            .find(|&&s| s > current)
            .copied()
            .unwrap_or(current + 10);
        rounds_input.set(next.to_string());
    };

    let mut rounds_step_down = move || {
        let current = rounds_input().parse::<u64>().unwrap_or(1);
        let prev = rounds_steps_clone
            .iter()
            .rev()
            .find(|&&s| s < current)
            .copied()
            .unwrap_or(1)
            .max(1);
        rounds_input.set(prev.to_string());
    };

    let is_per_tile_mode = grid_settings.use_legacy_form;

    let pro_num_rounds = use_memo(move || rounds_input().parse::<u64>().unwrap_or(1).max(1));

    // Total bet amount: mode-aware
    let total_bet_amount = use_memo(move || {
        let input_f64 = amount().parse::<f64>().unwrap_or(0.0);
        let input_lamports = sol_to_lamports(input_f64);
        if is_per_tile_mode() {
            // Per-tile mode: input is SOL per tile per round
            input_lamports * num_squares() * pro_num_rounds()
        } else {
            // Total mode: input is total SOL
            input_lamports
        }
    });

    // Per-round lamports: mode-aware
    let per_round_lamports = use_memo(move || {
        let input_f64 = amount().parse::<f64>().unwrap_or(0.0);
        let input_lamports = sol_to_lamports(input_f64);
        if is_per_tile_mode() {
            // Per-tile mode: per round = input * tiles
            input_lamports * num_squares()
        } else {
            // Total mode: per round = total / rounds
            let rounds = pro_num_rounds();
            if rounds > 0 {
                input_lamports / rounds
            } else {
                0
            }
        }
    });

    let active_balance_str = use_memo(move || {
        if let Wallet::Connected(_) = wallet() {
            if let Ok(sol_balance) = sol_balance() {
                format!("{}", lamports_to_sol(sol_balance.0))
            } else {
                "0".to_string()
            }
        } else {
            "".to_string()
        }
    });

    let _total_ore_balance = use_memo(move || {
        let Ok(mut miner) = miner() else { return 0u64 };
        let Ok(treasury) = treasury() else {
            return miner.rewards_ore + uncheckpointed_rewards().1;
        };
        miner.update_rewards(&treasury);
        miner.rewards_ore + miner.refined_ore + uncheckpointed_rewards().1
    });

    // Bridge signal: always passes total amount to tx builder regardless of input mode
    let mut total_amount_for_tx = use_signal(|| "".to_owned());
    use_effect(move || {
        let total = total_bet_amount();
        if total > 0 {
            total_amount_for_tx.set(format!("{}", lamports_to_sol(total)));
        } else {
            total_amount_for_tx.set("".to_owned());
        }
    });

    let pro_tx = use_pro_deploy_transaction(
        total_amount_for_tx,
        num_squares,
        selected_squares,
        pro_num_rounds,
        grid_settings.auto_reload,
        grid_settings.randomize_tiles,
        effective_solo_tiles,
        effective_split_tiles,
        effective_min_motherlode,
        effective_max_motherlode,
    );
    on_transaction_done(move |_| {
        amount.set("".to_owned());
        rounds_input.set("".to_owned());
        territories.set("".to_owned());
        selected_squares.set([false; 25]);
    });

    let is_insufficient_sol = use_memo(move || {
        matches!(pro_tx.value()(), Some(Err(GatewayError::InsufficientSOL)))
    });

    let deploy_amount_str = use_memo(move || {
        if total_bet_amount() > 0 {
            Some(format!("{}", lamports_to_sol(total_bet_amount())))
        } else {
            None
        }
    });

    rsx! {
        Col {
            gap: 0,
            class: "w-full items-center",

            // Tabs + settings
            div {
                class: "w-full relative",
                // User's deployed SOL (left-aligned pill with tooltip)
                {
                    let user_deployed = match (miner(), board()) {
                        (Ok(m), Ok(b)) if m.round_id == b.round_id => m.deployed.iter().sum::<u64>(),
                        _ => 0,
                    };
                    if user_deployed > 0 && !show_settings() {
                        rsx! {
                            DeployedPill {
                                amount: user_deployed,
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                if show_settings() {
                    // Centered title — matches tab row height (px-4 py-1.5 text-sm + border-2)
                    Row {
                        class: "justify-center",
                        gap: 2,
                        span {
                            class: "px-4 py-1.5 text-sm font-semibold text-elements-highEmphasis border-2 border-transparent",
                            "Settings"
                        }
                    }
                    // Close button
                    button {
                        class: "absolute right-0 top-1/2 -translate-y-1/2 p-2 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover transition-colors duration-200 cursor-pointer",
                        onclick: move |_| show_settings.set(false),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "2.5",
                            stroke: "currentColor",
                            class: "w-5 h-5",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }
                    }
                } else {
                    DeployFormV2Tabs {}
                    button {
                        class: "absolute right-0 top-1/2 -translate-y-1/2 p-2 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover transition-colors duration-200 cursor-pointer",
                        onclick: move |_| show_settings.set(true),
                        CogIcon {
                            class: "w-5 h-5",
                        }
                    }
                }
            }

            if show_settings() {
                // Settings menu
                DeploySettingsMenu {}
            } else {

            // Large amount input with dot grid background
            div {
                class: "relative w-full flex flex-col items-center mt-4 py-4",
                // Dot grid with breathing animation
                div {
                    class: "absolute inset-0 pointer-events-none",
                    style: "background-image: radial-gradient(circle, rgba(148,163,184,0.2) 1px, transparent 1px); background-size: 12px 12px; background-position: 6px 0; mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%); -webkit-mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%);",
                }
                div {
                    class: "relative w-full flex justify-center",
                    input {
                        class: "text-center text-6xl sm:text-7xl font-bold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 w-full [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0",
                        r#type: "number",
                        step: "0.1",
                        inputmode: "decimal",
                        min: "0",
                        value: amount(),
                        oninput: move |e: FormEvent| amount.set(e.value()),
                    }
                }
                // Active balance
                Row {
                    class: if is_insufficient_sol() {
                        "relative gap-1.5 mt-1 text-red-400 border border-red-500 rounded-full px-2.5 py-0.5 transition-colors duration-200"
                    } else {
                        "relative text-elements-lowEmphasis gap-1.5 mt-1 border border-transparent rounded-full px-2.5 py-0.5 transition-colors duration-200"
                    },
                    img {
                        src: asset!("/assets/solana.png"),
                        class: "w-4 h-4 my-auto",
                    }
                    span {
                        class: "my-auto text-nowrap text-sm font-semibold",
                        "{active_balance_str()}"
                    }
                }
            }

            // Quick add buttons
            div {
                class: "w-full mt-4",
                DeployFormV2QuickButtons {
                    amount: amount,
                    sol_balance: sol_balance,
                }
            }

            // Info rows
            Col {
                gap: 1,
                class: "w-full mt-4",
                // Tile selection rows
                if (grid_settings.tile_strategy)() {
                        // Tile strategy: Solo + Split rows
                        {
                            let mut split_count = split_tiles_count;
                            let mut solo_count = solo_tiles_count;

                            let split_tiles = split_tile_indices;
                            let solo_tiles = solo_tile_indices;

                            let max_split: u64 = 15;
                            let max_solo: u64 = 10;

                            let mut add_tile_from = move |tile_indices: &[u64]| {
                                let mut squares = selected_squares();
                                let unselected: Vec<u64> = tile_indices.iter().copied()
                                    .filter(|&i| !squares[i as usize]).collect();
                                if !unselected.is_empty() {
                                    let idx = unselected[rand::random::<usize>() % unselected.len()];
                                    squares[idx as usize] = true;
                                    selected_squares.set(squares);
                                }
                            };

                            let mut remove_tile_from = move |tile_indices: &[u64]| {
                                let mut squares = selected_squares();
                                let selected: Vec<u64> = tile_indices.iter().copied()
                                    .filter(|&i| squares[i as usize]).collect();
                                if !selected.is_empty() {
                                    let idx = selected[rand::random::<usize>() % selected.len()];
                                    squares[idx as usize] = false;
                                    selected_squares.set(squares);
                                }
                            };

                            let mut set_all_from = move |tile_indices: &[u64], select: bool| {
                                let mut squares = selected_squares();
                                for &i in tile_indices {
                                    squares[i as usize] = select;
                                }
                                selected_squares.set(squares);
                            };

                            let mut set_count_from = move |tile_indices: &[u64], target: usize| {
                                let mut squares = selected_squares();
                                let current_selected: Vec<u64> = tile_indices.iter().copied()
                                    .filter(|&i| squares[i as usize]).collect();
                                let current = current_selected.len();
                                if target > current {
                                    let mut unselected: Vec<u64> = tile_indices.iter().copied()
                                        .filter(|&i| !squares[i as usize]).collect();
                                    let mut to_add = target - current;
                                    while to_add > 0 && !unselected.is_empty() {
                                        let idx_pos = rand::random::<usize>() % unselected.len();
                                        squares[unselected.remove(idx_pos) as usize] = true;
                                        to_add -= 1;
                                    }
                                } else if target < current {
                                    let mut selected_idxs = current_selected;
                                    let mut to_remove = current - target;
                                    while to_remove > 0 && !selected_idxs.is_empty() {
                                        let idx_pos = rand::random::<usize>() % selected_idxs.len();
                                        squares[selected_idxs.remove(idx_pos) as usize] = false;
                                        to_remove -= 1;
                                    }
                                }
                                selected_squares.set(squares);
                            };

                            rsx! {
                                DeployInfoRow {
                                    title: "Solo tiles".to_string(),
                                    description: "The number of solo tiles to deploy on.".to_string(),
                                    value: rsx! {
                                        Row {
                                            gap: 1,
                                            class: "items-center",
                                            button {
                                                class: "flex items-center justify-center w-16 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-xs font-semibold uppercase border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    let solos = solo_tiles();
                                                    let max_available = (solos.len() as u64).min(max_solo);
                                                    if solo_count() == max_available {
                                                        solo_count.set(0);
                                                        set_all_from(&solos, false);
                                                    } else {
                                                        solo_count.set(max_available);
                                                        set_all_from(&solos, false);
                                                        set_count_from(&solos, max_available as usize);
                                                    }
                                                },
                                                if solo_count() == (solo_tiles().len() as u64).min(max_solo) { "None" } else { "All" }
                                            }
                                            button {
                                                class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    if solo_count() > 0 {
                                                        solo_count.set(solo_count() - 1);
                                                        remove_tile_from(&solo_tiles());
                                                    }
                                                },
                                                "−"
                                            }
                                            input {
                                                class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                                placeholder: "0",
                                                r#type: "number",
                                                step: "1",
                                                inputmode: "numeric",
                                                min: "0",
                                                max: "{max_solo}",
                                                value: if solo_count() > 0 { format!("{}", solo_count()) } else { "".to_string() },
                                                oninput: move |e: FormEvent| {
                                                    let max_available = (solo_tiles().len() as u64).min(max_solo);
                                                    let val = e.value().parse::<u64>().unwrap_or(0).min(max_available);
                                                    solo_count.set(val);
                                                    set_count_from(&solo_tiles(), val as usize);
                                                },
                                            }
                                            button {
                                                class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    let max_available = (solo_tiles().len() as u64).min(max_solo);
                                                    if solo_count() < max_available {
                                                        solo_count.set(solo_count() + 1);
                                                        add_tile_from(&solo_tiles());
                                                    }
                                                },
                                                "+"
                                            }
                                        }
                                    },
                                }
                                DeployInfoRow {
                                    title: "Split tiles".to_string(),
                                    description: "The number of split tiles to deploy on.".to_string(),
                                    value: rsx! {
                                        Row {
                                            gap: 1,
                                            class: "items-center",
                                            button {
                                                class: "flex items-center justify-center w-16 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-xs font-semibold uppercase border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    let splits = split_tiles();
                                                    let max_available = (splits.len() as u64).min(max_split);
                                                    if split_count() == max_available {
                                                        split_count.set(0);
                                                        set_all_from(&splits, false);
                                                    } else {
                                                        split_count.set(max_available);
                                                        set_all_from(&splits, false);
                                                        set_count_from(&splits, max_available as usize);
                                                    }
                                                },
                                                if split_count() == (split_tiles().len() as u64).min(max_split) { "None" } else { "All" }
                                            }
                                            button {
                                                class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    if split_count() > 0 {
                                                        split_count.set(split_count() - 1);
                                                        remove_tile_from(&split_tiles());
                                                    }
                                                },
                                                "−"
                                            }
                                            input {
                                                class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                                placeholder: "0",
                                                r#type: "number",
                                                step: "1",
                                                inputmode: "numeric",
                                                min: "0",
                                                max: "{max_split}",
                                                value: if split_count() > 0 { format!("{}", split_count()) } else { "".to_string() },
                                                oninput: move |e: FormEvent| {
                                                    let max_available = (split_tiles().len() as u64).min(max_split);
                                                    let val = e.value().parse::<u64>().unwrap_or(0).min(max_available);
                                                    split_count.set(val);
                                                    set_count_from(&split_tiles(), val as usize);
                                                },
                                            }
                                            button {
                                                class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                                onclick: move |_| {
                                                    let max_available = (split_tiles().len() as u64).min(max_split);
                                                    if split_count() < max_available {
                                                        split_count.set(split_count() + 1);
                                                        add_tile_from(&split_tiles());
                                                    }
                                                },
                                                "+"
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    } else {
                        // Standard Tiles row
                        DeployInfoRow {
                            title: "Tiles".to_string(),
                            description: "The number of tiles to deploy SOL on.".to_string(),
                            value: rsx! {
                                Row {
                                    gap: 1,
                                    class: "items-center",
                                    button {
                                        class: "flex items-center justify-center w-16 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-xs font-semibold uppercase border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            if num_selected_squares() == 25 {
                                                selected_squares.set([false; 25]);
                                            } else {
                                                selected_squares.set([true; 25]);
                                            }
                                        },
                                        if num_selected_squares() == 25 { "None" } else { "All" }
                                    }
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let mut squares = selected_squares();
                                            let selected: Vec<usize> = squares.iter().enumerate().filter(|(_, &s)| s).map(|(i, _)| i).collect();
                                            if !selected.is_empty() {
                                                let idx = selected[rand::random::<usize>() % selected.len()];
                                                squares[idx] = false;
                                                selected_squares.set(squares);
                                            }
                                        },
                                        "−"
                                    }
                                    input {
                                        class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                        placeholder: "0",
                                        r#type: "number",
                                        step: "1",
                                        inputmode: "numeric",
                                        min: "0",
                                        max: "25",
                                        value: if num_squares() > 0 { format!("{}", num_squares()) } else { "".to_string() },
                                        oninput: move |e: FormEvent| {
                                            let val = e.value().parse::<usize>().unwrap_or(0).min(25);
                                            let current_count = selected_squares().iter().filter(|&&s| s).count();
                                            if val > current_count {
                                                let mut squares = selected_squares();
                                                let mut to_add = val - current_count;
                                                let mut unselected: Vec<usize> = squares.iter().enumerate().filter(|(_, &s)| !s).map(|(i, _)| i).collect();
                                                while to_add > 0 && !unselected.is_empty() {
                                                    let idx_pos = rand::random::<usize>() % unselected.len();
                                                    squares[unselected.remove(idx_pos)] = true;
                                                    to_add -= 1;
                                                }
                                                selected_squares.set(squares);
                                            } else if val < current_count {
                                                let mut squares = selected_squares();
                                                let mut to_remove = current_count - val;
                                                let mut selected_idxs: Vec<usize> = squares.iter().enumerate().filter(|(_, &s)| s).map(|(i, _)| i).collect();
                                                while to_remove > 0 && !selected_idxs.is_empty() {
                                                    let idx_pos = rand::random::<usize>() % selected_idxs.len();
                                                    squares[selected_idxs.remove(idx_pos)] = false;
                                                    to_remove -= 1;
                                                }
                                                selected_squares.set(squares);
                                            }
                                        },
                                    }
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let mut squares = selected_squares();
                                            let unselected: Vec<usize> = squares.iter().enumerate().filter(|(_, &s)| !s).map(|(i, _)| i).collect();
                                            if !unselected.is_empty() {
                                                let idx = unselected[rand::random::<usize>() % unselected.len()];
                                                squares[idx] = true;
                                                selected_squares.set(squares);
                                            }
                                        },
                                        "+"
                                    }
                                }
                            },
                        }
                    }
                // Rounds row (shared across all Pro tile modes)
                DeployInfoRow {
                        title: "Rounds".to_string(),
                        description: "The number of rounds to deploy across.".to_string(),
                        value: rsx! {
                            Row {
                                class: "items-center gap-1",
                                button {
                                    class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                    onclick: move |_| {
                                        if pro_num_rounds() > 1 {
                                            rounds_step_down();
                                        }
                                    },
                                    "−"
                                }
                                input {
                                    class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                    placeholder: "1",
                                    r#type: "number",
                                    step: "1",
                                    inputmode: "numeric",
                                    min: "1",
                                    value: rounds_input(),
                                    oninput: move |e: FormEvent| rounds_input.set(e.value()),
                                }
                                button {
                                    class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                    onclick: move |_| rounds_step_up(),
                                    "+"
                                }
                            }
                        },
                    }
                    // Per round (hidden in per-tile mode since user already types it)
                    if !is_per_tile_mode() {
                        DeployInfoRow {
                            title: "Per round".to_string(),
                            description: "The calculated SOL deployed per round.".to_string(),
                            value: rsx! {
                                Row {
                                    class: "items-center gap-1.5",
                                    img {
                                        src: asset!("/assets/solana.png"),
                                        class: "h-3.5 w-3.5",
                                    }
                                    span {
                                        class: "text-elements-highEmphasis text-xl font-semibold",
                                        "{lamports_to_sol(per_round_lamports())}"
                                    }
                                }
                            },
                        }
                    }
                    // Min motherlode (conditional)
                    if min_motherlode_enabled() {
                        DeployInfoRow {
                            title: "Min motherlode".to_string(),
                            description: "Only deploy when the motherlode is above this threshold (in ORE).".to_string(),
                            value: rsx! {
                                Row {
                                    class: "items-center gap-1",
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let val = min_motherlode_value();
                                            if val >= 10 {
                                                min_motherlode_value.set(val - 10);
                                            } else {
                                                min_motherlode_value.set(0);
                                            }
                                        },
                                        "−"
                                    }
                                    input {
                                        class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                        placeholder: "0",
                                        r#type: "number",
                                        step: "10",
                                        inputmode: "numeric",
                                        min: "0",
                                        value: if min_motherlode_value() > 0 { format!("{}", min_motherlode_value()) } else { "".to_string() },
                                        oninput: move |e: FormEvent| {
                                            let val = e.value().parse::<u16>().unwrap_or(0);
                                            min_motherlode_value.set(val);
                                        },
                                    }
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let val = min_motherlode_value();
                                            min_motherlode_value.set(val.saturating_add(10));
                                        },
                                        "+"
                                    }
                                }
                            },
                        }
                    }
                    // Max motherlode (conditional)
                    if max_motherlode_enabled() {
                        DeployInfoRow {
                            title: "Max motherlode".to_string(),
                            description: "Only deploy when the motherlode is below this threshold (in ORE).".to_string(),
                            value: rsx! {
                                Row {
                                    class: "items-center gap-1",
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let val = max_motherlode_value();
                                            if val >= 10 {
                                                max_motherlode_value.set(val - 10);
                                            } else {
                                                max_motherlode_value.set(0);
                                            }
                                        },
                                        "−"
                                    }
                                    input {
                                        class: "w-20 text-center text-xl font-semibold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                                        placeholder: "∞",
                                        r#type: "number",
                                        step: "10",
                                        inputmode: "numeric",
                                        min: "0",
                                        value: if max_motherlode_value() > 0 { format!("{}", max_motherlode_value()) } else { "".to_string() },
                                        oninput: move |e: FormEvent| {
                                            let val = e.value().parse::<u16>().unwrap_or(0);
                                            max_motherlode_value.set(val);
                                        },
                                    }
                                    button {
                                        class: "flex items-center justify-center w-9 h-9 rounded-md bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis cursor-pointer transition-all duration-150 text-lg font-semibold border-2 border-transparent active:border-white",
                                        onclick: move |_| {
                                            let val = max_motherlode_value();
                                            max_motherlode_value.set(val.saturating_add(10));
                                        },
                                        "+"
                                    }
                                }
                            },
                        }
                }
            }

            // Action button
            div {
                class: "w-full mt-4",
                ActionButton {
                    transaction: pro_tx,
                    if let Some(amount_str) = deploy_amount_str() {
                        "Deploy"
                        img {
                            src: asset!("/assets/solana.png"),
                            class: "h-3.5 w-3.5",
                        }
                        "{amount_str}"
                    } else {
                        "Deploy"
                    }
                }
            }

            // // ORE balance label
            // if total_ore_balance() > 0 {
            //     div {
            //         class: "w-full flex justify-center mt-3",
            //         Link {
            //             class: "flex flex-row gap-1.5 text-elements-gold rounded-full px-3 py-1 hover:bg-controls-secondaryHover transition-colors duration-200",
            //             to: Route::Rewards {},
            //             OreIcon {
            //                 class: "w-3.5 h-3.5 my-auto",
            //             }
            //             span {
            //                 class: "my-auto text-sm font-semibold",
            //                 "{amount_to_ui_amount(total_ore_balance(), TOKEN_DECIMALS)}"
            //             }
            //         }
            //     }
            // }
            } // end else (deploy form)
        }
    }
}

/// Deploy form info row with tooltip popup.
/// Desktop: hover to show, mouseleave to hide.
/// Mobile: tap to show, tap anywhere else to dismiss.
#[component]
fn DeployInfoRow(title: String, description: String, value: Element) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);

    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        Row {
            class: "w-full justify-between items-center h-10 px-3",
            div {
                class: "relative",
                // Desktop hover (only when not in clicked/pinned state)
                onmouseenter: move |_| {
                    if !clicked() {
                        hovered.set(true);
                    }
                },
                onmouseleave: move |_| {
                    hovered.set(false);
                },
                // Click/tap toggles pinned state
                onclick: move |e| {
                    e.stop_propagation();
                    let new_state = !clicked();
                    clicked.set(new_state);
                    // If closing via click, also clear hover
                    if !new_state {
                        hovered.set(false);
                    }
                },
                span {
                    class: "text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide cursor-help border-b border-dashed border-elements-lowEmphasis hover:text-elements-highEmphasis hover:border-elements-highEmphasis transition-colors duration-200",
                    "{title}"
                }
                if is_visible() {
                    // Invisible backdrop to catch taps/clicks elsewhere and close tooltip
                    if clicked() {
                        div {
                            class: "fixed inset-0 z-40",
                            onclick: move |e| {
                                e.stop_propagation();
                                clicked.set(false);
                                hovered.set(false);
                            },
                        }
                    }
                    // Tooltip popup
                    div {
                        class: "absolute bottom-full left-0 mb-2 z-50 w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                        onclick: move |e| e.stop_propagation(),
                        "{description}"
                    }
                }
            }
            div {
                class: "shrink-0",
                {value}
            }
        }
    }
}

#[component]
fn AutoReloadTooltip() -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        div {
            class: "relative mr-1",
            onmouseenter: move |_| {
                if !clicked() { hovered.set(true); }
            },
            onmouseleave: move |_| { hovered.set(false); },
            onclick: move |e| {
                e.stop_propagation();
                let new_state = !clicked();
                clicked.set(new_state);
                if !new_state { hovered.set(false); }
            },
            AutoReloadIcon {
                class: if is_visible() {
                    "h-4 w-4 text-elements-highEmphasis cursor-help transition-colors"
                } else {
                    "h-4 w-4 text-elements-lowEmphasis hover:text-elements-highEmphasis cursor-help transition-colors"
                }
            }
            if is_visible() {
                if clicked() {
                    div {
                        class: "fixed inset-0 z-40",
                        onclick: move |e| {
                            e.stop_propagation();
                            clicked.set(false);
                            hovered.set(false);
                        },
                    }
                }
                div {
                    class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                    onclick: move |e| e.stop_propagation(),
                    "SOL rewards auto-reload back into your autominer balance."
                }
            }
        }
    }
}

#[component]
fn PreferredTilesTooltip() -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        div {
            class: "relative mr-1",
            onmouseenter: move |_| {
                if !clicked() { hovered.set(true); }
            },
            onmouseleave: move |_| { hovered.set(false); },
            onclick: move |e| {
                e.stop_propagation();
                let new_state = !clicked();
                clicked.set(new_state);
                if !new_state { hovered.set(false); }
            },
            BookmarkIcon {
                class: if is_visible() {
                    "h-4 w-4 text-elements-highEmphasis cursor-help transition-colors"
                } else {
                    "h-4 w-4 text-elements-lowEmphasis hover:text-elements-highEmphasis cursor-help transition-colors"
                }
            }
            if is_visible() {
                if clicked() {
                    div {
                        class: "fixed inset-0 z-40",
                        onclick: move |e| {
                            e.stop_propagation();
                            clicked.set(false);
                            hovered.set(false);
                        },
                    }
                }
                div {
                    class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                    onclick: move |e| e.stop_propagation(),
                    "Tile selection is fixed and not randomized."
                }
            }
        }
    }
}

#[component]
fn ConditionBlockedTooltip(message: String) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        div {
            class: "relative",
            onmouseenter: move |_| {
                if !clicked() { hovered.set(true); }
            },
            onmouseleave: move |_| { hovered.set(false); },
            onclick: move |e| {
                e.stop_propagation();
                let new_state = !clicked();
                clicked.set(new_state);
                if !new_state { hovered.set(false); }
            },
            WarningIcon {
                class: if is_visible() {
                    "h-4 w-4 text-yellow-400 cursor-help transition-colors"
                } else {
                    "h-4 w-4 text-yellow-500 hover:text-yellow-500 cursor-help transition-colors"
                }
            }
            if is_visible() {
                if clicked() {
                    div {
                        class: "fixed inset-0 z-40",
                        onclick: move |e| {
                            e.stop_propagation();
                            clicked.set(false);
                            hovered.set(false);
                        },
                    }
                }
                div {
                    class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-2 z-50 w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                    onclick: move |e| e.stop_propagation(),
                    "{message}"
                }
            }
        }
    }
}

#[component]
fn DeployedPill(amount: u64) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        div {
            class: "absolute left-0 top-1/2 -translate-y-1/2",
            onmouseenter: move |_| {
                if !clicked() {
                    hovered.set(true);
                }
            },
            onmouseleave: move |_| {
                hovered.set(false);
            },
            onclick: move |e| {
                e.stop_propagation();
                let new_state = !clicked();
                clicked.set(new_state);
                if !new_state {
                    hovered.set(false);
                }
            },
            div {
                class: "px-2.5 py-0.5 rounded-full bg-blue-500/10 border border-blue-500/30 cursor-help",
                Row {
                    class: "items-center gap-1 text-blue-400",
                    img {
                        src: asset!("/assets/solana.png"),
                        class: "w-3 h-3",
                    }
                    span {
                        class: "text-xs font-semibold",
                        "{display_sol_3dp(amount)}"
                    }
                }
            }
            if is_visible() {
                if clicked() {
                    div {
                        class: "fixed inset-0 z-40",
                        onclick: move |e| {
                            e.stop_propagation();
                            clicked.set(false);
                            hovered.set(false);
                        },
                    }
                }
                div {
                    class: "absolute top-full left-0 mt-2 z-[100] w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                    onclick: move |e| e.stop_propagation(),
                    "Total SOL you have deployed in the current round."
                }
            }
        }
    }
}


#[component]
fn DeploySettingsMenu(automation: Option<Signal<GatewayResult<Automation>>>) -> Element {
    let grid_settings: GridSettings = use_context();
    let show_tile_number = grid_settings.show_tile_number;
    let show_sol_deployed = grid_settings.show_total_deployed;
    let show_your_deployed = grid_settings.show_your_deployed;
    let show_deploy_gauge = grid_settings.show_deploy_gauge;
    let show_distribution = grid_settings.show_distribution;
    let use_legacy_form = grid_settings.use_legacy_form;
    let auto_reload = grid_settings.auto_reload;
    let tile_strategy = grid_settings.tile_strategy;
    let randomize_tiles = grid_settings.randomize_tiles;
    let min_motherlode_enabled = grid_settings.min_motherlode_enabled;
    let max_motherlode_enabled = grid_settings.max_motherlode_enabled;
    let wallet = use_wallet();
    let submitter = use_transaction_submitter();

    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_grid_tile_number", show_tile_number());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_grid_total_deployed", show_sol_deployed());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_grid_your_deployed", show_your_deployed());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_grid_deploy_gauge", show_deploy_gauge());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_grid_distribution", show_distribution());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_use_legacy_form", use_legacy_form());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_auto_reload", auto_reload());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_tile_strategy", tile_strategy());
    });
    use_effect(move || {
        let _ = gloo_storage::LocalStorage::set("ore_randomize_tiles", randomize_tiles());
    });
    use_effect(move || {
        let _ =
            gloo_storage::LocalStorage::set("ore_min_motherlode_enabled", min_motherlode_enabled());
    });
    use_effect(move || {
        let _ =
            gloo_storage::LocalStorage::set("ore_max_motherlode_enabled", max_motherlode_enabled());
    });

    rsx! {
        Col {
            gap: 4,
            class: "w-full mt-4 px-3",
            // Grid section
            Col {
                gap: 0,
                class: "w-full",
                span {
                    class: "text-elements-highEmphasis text-xs font-semibold uppercase tracking-wider mb-1",
                    "Grid"
                }
                SettingsToggle {
                    label: "Tile number".to_string(),
                    enabled: show_tile_number,
                }
                SettingsToggle {
                    label: "Total deployed".to_string(),
                    enabled: show_sol_deployed,
                }
                SettingsToggle {
                    label: "Your deployed".to_string(),
                    enabled: show_your_deployed,
                }
                SettingsToggle {
                    label: "Deploy gauge".to_string(),
                    enabled: show_deploy_gauge,
                }
                SettingsToggle {
                    label: "Solos".to_string(),
                    enabled: show_distribution,
                }
            }
            // Miner section
            Col {
                gap: 0,
                class: "w-full",
                span {
                    class: "text-elements-highEmphasis text-xs font-semibold uppercase tracking-wider mb-1",
                    "Miner"
                }
                SettingsToggle {
                    label: "Auto reload".to_string(),
                    description: "Automatically reload SOL rewards back into the autominer balance.".to_string(),
                    enabled: auto_reload,
                    on_change: move |new_val: bool| {
                        if let Some(automation) = automation {
                            if let Ok(a) = automation() {
                                if let Ok(authority) = wallet.pubkey() {
                                    let mut ixs = vec![];
                                    ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(100_000));
                                    ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(0));
                                    ixs.push(ore_api::sdk::automate(
                                        authority,
                                        a.amount,
                                        0,
                                        a.executor,
                                        a.fee,
                                        a.mask,
                                        a.strategy as u8,
                                        new_val,
                                        a.conditions,
                                    ));
                                    ixs.push(tip_ix(&authority));
                                    let tx: VersionedTransaction = solana_sdk::transaction::Transaction::new_with_payer(&ixs, Some(&authority)).into();
                                    submitter.send(tx);
                                }
                            }
                        }
                    },
                }
                SettingsToggle {
                    label: "Per tile control".to_string(),
                    description: "If enabled, the input form will expect deployment amount per tile, rather than total deployment amount.".to_string(),
                    enabled: use_legacy_form,
                }
                SettingsToggle {
                    label: "SPLIT/SOLO CONTROL".to_string(),
                    description: "Choose precisely how many split and solo tiles to deploy on.".to_string(),
                    enabled: tile_strategy,
                }
                if !tile_strategy() {
                SettingsToggle {
                    label: "Randomize tiles".to_string(),
                    description: "Randomize autominer tile selection.".to_string(),
                    enabled: randomize_tiles,
                    on_change: move |new_val: bool| {
                        if let Some(automation) = automation {
                            if let Ok(a) = automation() {
                                if let Ok(authority) = wallet.pubkey() {
                                    let new_strategy: u8 = if new_val { 0 } else { 1 };
                                    let mut ixs = vec![];
                                    ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(100_000));
                                    ixs.push(solana_sdk::compute_budget::ComputeBudgetInstruction::set_compute_unit_price(0));
                                    ixs.push(ore_api::sdk::automate(
                                        authority,
                                        a.amount,
                                        0,
                                        a.executor,
                                        a.fee,
                                        a.mask,
                                        new_strategy,
                                        a.reload > 0,
                                        a.conditions
                                    ));
                                    ixs.push(tip_ix(&authority));
                                    let tx: VersionedTransaction = solana_sdk::transaction::Transaction::new_with_payer(&ixs, Some(&authority)).into();
                                    submitter.send(tx);
                                }
                            }
                        }
                    },
                }
                }
                SettingsToggle {
                    label: "Min motherlode".to_string(),
                    description: "Only deploy when the motherlode is above a minimum threshold.".to_string(),
                    enabled: min_motherlode_enabled,
                }
                SettingsToggle {
                    label: "Max motherlode".to_string(),
                    description: "Only deploy when the motherlode is below a maximum threshold.".to_string(),
                    enabled: max_motherlode_enabled,
                }
            }
        }
    }
}

#[component]
fn SettingsToggle(
    label: String,
    description: Option<String>,
    mut enabled: Signal<bool>,
    on_change: Option<EventHandler<bool>>,
) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let has_description = description.is_some();
    let is_visible = use_memo(move || has_description && (hovered() || clicked()));

    rsx! {
        Row {
            class: "w-full justify-between items-center py-1.5",
            if let Some(desc) = &description {
                div {
                    class: "relative",
                    onmouseenter: move |_| { if !clicked() { hovered.set(true); } },
                    onmouseleave: move |_| { hovered.set(false); },
                    onclick: move |e| {
                        e.stop_propagation();
                        let new_state = !clicked();
                        clicked.set(new_state);
                        if !new_state { hovered.set(false); }
                    },
                    span {
                        class: "text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide cursor-help border-b border-dashed border-elements-lowEmphasis hover:text-elements-highEmphasis hover:border-elements-highEmphasis transition-colors duration-200",
                        "{label}"
                    }
                    if is_visible() {
                        if clicked() {
                            div {
                                class: "fixed inset-0 z-40",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    clicked.set(false);
                                    hovered.set(false);
                                },
                            }
                        }
                        div {
                            class: "absolute bottom-full left-0 mb-2 z-50 w-max max-w-72 px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis shadow-xl text-elements-highEmphasis text-sm text-left",
                            onclick: move |e| e.stop_propagation(),
                            "{desc}"
                        }
                    }
                }
            } else {
                span {
                    class: "text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide",
                    "{label}"
                }
            }
            button {
                class: "cursor-pointer",
                onclick: move |_| {
                    let new_val = !enabled();
                    enabled.set(new_val);
                    if let Some(handler) = &on_change {
                        handler.call(new_val);
                    }
                },
                div {
                    class: if enabled() {
                        "relative w-12 h-7 rounded-full bg-white border border-white transition-colors duration-200"
                    } else {
                        "relative w-12 h-7 rounded-full bg-transparent border border-white transition-colors duration-200"
                    },
                    div {
                        class: if enabled() {
                            "absolute top-0.5 left-5.5 w-5.5 h-5.5 rounded-full bg-black shadow-md transition-all duration-200"
                        } else {
                            "absolute top-1 left-1 w-4.5 h-4.5 rounded-full bg-white shadow-md transition-all duration-200"
                        },
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AutoplayFormV2 – redesigned autominer summary + top-up form
// ---------------------------------------------------------------------------

#[component]
fn AutoplayFormV2(
    automation: Signal<GatewayResult<Automation>>,
    sol_balance: Signal<GatewayResult<SolBalance>>,
    mut show_settings: Signal<bool>,
    winning_square: Memo<Option<u64>>,
    round_data: Signal<GatewayResult<Round>>,
    board: Signal<GatewayResult<Board>>,
    selected_squares: Signal<[bool; 25]>,
) -> Element {
    let mut show_topup = use_signal(|| false);
    let mut topup_amount = use_signal(|| "".to_owned());
    let topup_tx = use_topup_transaction(topup_amount, automation);
    let cancel_tx = use_cancel_transaction();
    let _submitter = use_transaction_submitter();

    // Sync auto_reload toggle with on-chain state (read-only sync)
    let grid_settings: GridSettings = use_context();
    let mut auto_reload = grid_settings.auto_reload;
    use_effect(move || {
        if let Ok(a) = automation() {
            auto_reload.set(a.reload > 0);
        }
    });

    // Sync randomize_tiles toggle with on-chain state (read-only sync)
    let mut randomize_tiles = grid_settings.randomize_tiles;
    use_effect(move || {
        if let Ok(a) = automation() {
            randomize_tiles.set(a.strategy == 0);
        }
    });

    on_transaction_done(move |_| {
        topup_amount.set("".to_owned());
        show_topup.set(false);
        selected_squares.set([false; 25]);
    });

    let balance = use_memo(move || automation().map(|a| a.balance).unwrap_or(0));

    let num_squares = use_memo(move || {
        if let Ok(a) = automation() {
            (0..25u64).filter(|i| (a.mask & (1 << i)) != 0).count() as u64
        } else {
            0
        }
    });

    let per_round_lamports =
        use_memo(move || automation().map(|a| a.amount * num_squares()).unwrap_or(0));

    let fee_per_round = use_memo(move || automation().map(|a| a.fee).unwrap_or(0));

    let net_per_round = use_memo(move || per_round_lamports() + fee_per_round());

    let est_rounds = use_memo(move || {
        if net_per_round() > 0 {
            balance() / net_per_round()
        } else {
            0
        }
    });

    let topup_lamports =
        use_memo(move || sol_to_lamports(topup_amount().parse::<f64>().unwrap_or(0.0)));

    let new_balance = use_memo(move || balance() + topup_lamports());

    let _new_est_rounds = use_memo(move || {
        if net_per_round() > 0 {
            new_balance() / net_per_round()
        } else {
            0
        }
    });

    let active_wallet_balance =
        use_memo(move || sol_balance().map(|b| lamports_to_sol(b.0)).unwrap_or(0.0));

    let is_insufficient_sol =
        use_memo(move || matches!(topup_tx.value()(), Some(Err(GatewayError::InsufficientSOL))));

    let miner = use_miner_wss();
    let board = use_board_wss();
    let user_deployed_auto = use_memo(move || match (miner(), board()) {
        (Ok(m), Ok(b)) if m.round_id == b.round_id => m.deployed.iter().sum::<u64>(),
        _ => 0,
    });

    rsx! {
        Col {
            gap: 0,
            class: "w-full items-center",

            // Header row
            div {
                class: "w-full relative",
                // User's deployed SOL (left-aligned pill with tooltip)
                if user_deployed_auto() > 0 && !show_settings() {
                    DeployedPill {
                        amount: user_deployed_auto(),
                    }
                }
                if show_settings() {
                    Row {
                        class: "justify-center",
                        span {
                            class: "px-4 py-1.5 text-sm font-semibold text-elements-highEmphasis",
                            "Settings"
                        }
                    }
                    button {
                        class: "absolute right-0 top-1/2 -translate-y-1/2 p-2 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover transition-colors duration-200 cursor-pointer",
                        onclick: move |_| show_settings.set(false),
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "2.5",
                            stroke: "currentColor",
                            class: "w-5 h-5",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }
                    }
                } else {
                    {
                        let treasury_for_indicator = use_treasury_wss();
                        let is_conditions_blocked = use_memo(move || {
                            let Ok(a) = automation() else { return false };
                            let motherlode_ore = treasury_for_indicator().map(|t| t.motherlode / ONE_ORE).unwrap_or(0);
                            let min = a.conditions.min_motherlode;
                            let max = a.conditions.max_motherlode;
                            (min > 0 && motherlode_ore < min as u64) || (max < u16::MAX && motherlode_ore > max as u64)
                        });
                        rsx! {
                            Row {
                                class: "justify-center items-center gap-2 py-1.5",
                                if is_conditions_blocked() {
                                    PauseCircleIcon {
                                        class: "h-4 w-4 text-yellow-500",
                                    }
                                } else {
                                    div {
                                        class: "relative flex h-2 w-2",
                                        span {
                                            class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-blue-400 opacity-75",
                                        }
                                        span {
                                            class: "relative inline-flex rounded-full h-2 w-2 bg-blue-500",
                                        }
                                    }
                                }
                                span {
                                    class: "text-sm font-semibold text-elements-highEmphasis",
                                    "Autominer"
                                }
                            }
                        }
                    }
                    button {
                        class: "absolute right-0 top-1/2 -translate-y-1/2 p-2 rounded-full text-elements-lowEmphasis hover:text-elements-highEmphasis hover:bg-controls-secondaryHover transition-colors duration-200 cursor-pointer",
                        onclick: move |_| show_settings.set(true),
                        CogIcon {
                            class: "w-5 h-5",
                        }
                    }
                }
            }

            if show_settings() {
                DeploySettingsMenu {
                    automation: Some(automation),
                }
            } else {
                // Big input
                div {
                    class: "relative w-full flex flex-col items-center mt-4 py-4",
                    div {
                        class: "absolute inset-0 pointer-events-none",
                        style: "background-image: radial-gradient(circle, rgba(148,163,184,0.2) 1px, transparent 1px); background-size: 12px 12px; background-position: 6px 0; mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%); -webkit-mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%);",
                    }
                    div {
                        class: "relative w-full flex justify-center",
                        input {
                            class: "text-center text-6xl sm:text-7xl font-bold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 w-full [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                            placeholder: "0",
                            r#type: "number",
                            step: "0.1",
                            inputmode: "decimal",
                            min: "0",
                            value: topup_amount(),
                            oninput: move |e: FormEvent| topup_amount.set(e.value()),
                        }
                    }
                    Row {
                        class: if is_insufficient_sol() {
                            "relative gap-1.5 mt-1 text-red-400 border border-red-500 rounded-full px-2.5 py-0.5 transition-colors duration-200"
                        } else {
                            "relative text-elements-lowEmphasis gap-1.5 mt-1 border border-transparent rounded-full px-2.5 py-0.5 transition-colors duration-200"
                        },
                        img {
                            src: asset!("/assets/solana.png"),
                            class: "w-4 h-4 my-auto",
                        }
                        span {
                            class: "my-auto text-nowrap text-sm font-semibold",
                            "{active_wallet_balance()}"
                        }
                    }
                }

                // Quick buttons
                div {
                    class: "w-full mt-4",
                    DeployFormV2QuickButtons {
                        amount: topup_amount,
                        sol_balance: sol_balance,
                    }
                }

                // Info rows
                Col {
                    gap: 1,
                    class: "w-full mt-4",
                    DeployInfoRow {
                        title: "Balance".to_string(),
                        description: "The remaining SOL balance in your autominer.".to_string(),
                        value: rsx! {
                            Row {
                                class: "items-center gap-1.5",
                                if auto_reload() {
                                    AutoReloadTooltip {}
                                }
                                img {
                                    src: asset!("/assets/solana.png"),
                                    class: "h-3.5 w-3.5",
                                }
                                span {
                                    class: "text-elements-highEmphasis text-base font-medium",
                                    "{lamports_to_sol(balance())}"
                                }
                            }
                        },
                    }
                    DeployInfoRow {
                        title: "Per round".to_string(),
                        description: "The amount of SOL deployed per round.".to_string(),
                        value: rsx! {
                            Row {
                                class: "items-center gap-1.5",
                                img {
                                    src: asset!("/assets/solana.png"),
                                    class: "h-3.5 w-3.5",
                                }
                                span {
                                    class: "text-elements-highEmphasis text-base font-medium",
                                    "{lamports_to_sol(per_round_lamports())}"
                                }
                            }
                        },
                    }
                    {
                        let tiles_display = use_memo(move || {
                            if let Ok(a) = automation() {
                                let solo = a.conditions.solo_tiles;
                                let split = a.conditions.split_tiles;
                                if solo > 0 && split > 0 {
                                    return format!("{solo} SOLO + {split} SPLIT");
                                } else if solo > 0 {
                                    return format!("{solo} SOLO");
                                } else if split > 0 {
                                    return format!("{split} SPLIT");
                                }
                            }
                            format!("{}", num_squares())
                        });
                        rsx! {
                            DeployInfoRow {
                                title: "Tiles".to_string(),
                                description: "The number of tiles your autominer deploys on each round.".to_string(),
                                value: rsx! {
                                    Row {
                                        class: "items-center gap-1.5",
                                        span {
                                            class: "text-elements-highEmphasis text-base font-medium",
                                            "{tiles_display()}"
                                        }
                                    }
                                },
                            }
                        }
                    }
                    DeployInfoRow {
                        title: "Est. rounds".to_string(),
                        description: "The estimated number of rounds your autominer will run for.".to_string(),
                        value: rsx! {
                            span {
                                class: "text-elements-highEmphasis text-base font-medium",
                                "{est_rounds()}"
                            }
                        },
                    }
                    {
                        let min_ml = use_memo(move || automation().map(|a| a.conditions.min_motherlode).unwrap_or(0));
                        let max_ml = use_memo(move || automation().map(|a| a.conditions.max_motherlode).unwrap_or(u16::MAX));
                        let treasury = use_treasury_wss();
                        let current_motherlode_ore = use_memo(move || {
                            treasury().map(|t| t.motherlode / ONE_ORE).unwrap_or(0)
                        });
                        let min_blocked = use_memo(move || {
                            min_ml() > 0 && current_motherlode_ore() < min_ml() as u64
                        });
                        let max_blocked = use_memo(move || {
                            max_ml() < u16::MAX && current_motherlode_ore() > max_ml() as u64
                        });
                        rsx! {
                            if min_ml() > 0 {
                                DeployInfoRow {
                                    title: "Min motherlode".to_string(),
                                    description: "Autominer only deploys when the motherlode is above this threshold (in ORE).".to_string(),
                                    value: rsx! {
                                        Row {
                                            class: "items-center gap-1.5",
                                            if min_blocked() {
                                                ConditionBlockedTooltip {
                                                    message: "Deployment paused. Motherlode is below the minimum threshold.".to_string(),
                                                }
                                            }
                                            span {
                                                class: "text-elements-highEmphasis text-base font-medium",
                                                "{min_ml()}"
                                            }
                                        }
                                    },
                                }
                            }
                            if max_ml() < u16::MAX {
                                DeployInfoRow {
                                    title: "Max motherlode".to_string(),
                                    description: "Autominer only deploys when the motherlode is below this threshold (in ORE).".to_string(),
                                    value: rsx! {
                                        Row {
                                            class: "items-center gap-1.5",
                                            if max_blocked() {
                                                ConditionBlockedTooltip {
                                                    message: "Deployment paused. Motherlode is above the maximum threshold.".to_string(),
                                                }
                                            }
                                            span {
                                                class: "text-elements-highEmphasis text-base font-medium",
                                                "{max_ml()}"
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                // Top up button
                div {
                    class: "w-full mt-4",
                    ActionButton {
                        transaction: topup_tx,
                        if topup_lamports() > 0 {
                            "Top up"
                            img {
                                src: asset!("/assets/solana.png"),
                                class: "h-3.5 w-3.5",
                            }
                            "{lamports_to_sol(topup_lamports())}"
                        } else {
                            "Top up"
                        }
                    }
                }

                // Stop button
                div {
                    class: "w-full mt-2",
                    ActionButton {
                        class: "text-elements-lowEmphasis cursor-pointer hover:text-elements-highEmphasis hover:bg-controls-secondaryHover".to_string(),
                        transaction: cancel_tx,
                        "Stop"
                    }
                }
            }
        }
    }
}
