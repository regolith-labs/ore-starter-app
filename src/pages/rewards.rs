use dioxus::prelude::*;
use ore_api::{
    consts::{DENOMINATOR_BPS, ONE_ORE, TOKEN_DECIMALS},
    state::{Miner, Treasury},
};
use solana_extra_wasm::program::spl_token::amount_to_ui_amount_string_trimmed;
use solana_sdk::native_token::lamports_to_sol;

use crate::{
    components::*,
    gateway::{ore::OreGateway, GatewayResult},
    hooks::{
        on_transaction_done, use_claim_ore_transaction, use_claim_sol_transaction, use_gateway,
        use_miner_wss, use_round_wss, use_treasury_wss,
    },
};

pub fn Rewards() -> Element {
    let miner = use_miner_wss();
    let treasury = use_treasury_wss();
    let round = use_round_wss();

    // Detect if the miner is in the current round
    let is_current_round = use_memo(move || {
        let Ok(miner) = miner() else { return false };
        let Ok(round) = round() else { return false };
        miner.round_id == round.id
    });

    // Optimistic rewards for the current round (same logic as home page)
    let optimistic_rewards = use_memo(move || {
        if !is_current_round() {
            return (0u64, 0u64);
        }
        let Ok(miner) = miner() else { return (0, 0) };
        if miner.round_id == miner.checkpoint_id {
            return (0, 0);
        }
        let Ok(round) = round() else { return (0, 0) };
        let Some(r) = round.rng() else { return (0, 0) };
        let winning_square = round.winning_square(r);
        if miner.deployed[winning_square] == 0 {
            return (0, 0);
        }
        let mut total_winnings = round.calculate_total_winnings(winning_square);
        let total_winnings_admin_fee = total_winnings / 100;
        total_winnings -= total_winnings_admin_fee;
        let vault_amount = total_winnings / 100;
        total_winnings -= vault_amount;
        let top_miner_reward = ONE_ORE;
        let is_split_reward = round.is_split_reward(r);
        let top_miner_sample = round.top_miner_sample(r, winning_square);
        let is_top_miner = top_miner_sample >= miner.cumulative[winning_square]
            && top_miner_sample < miner.cumulative[winning_square] + miner.deployed[winning_square];
        let mut sol_rewards = ((total_winnings as u128 * miner.deployed[winning_square] as u128)
            / round.deployed[winning_square] as u128) as u64;
        sol_rewards += miner.deployed[winning_square] * 99 / 100;
        let mut ore_rewards = 0u64;
        if is_split_reward {
            ore_rewards += ((top_miner_reward as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        } else if is_top_miner {
            ore_rewards += top_miner_reward;
        }
        (sol_rewards, ore_rewards)
    });

    // Past round rewards (async fetch, same logic as home page)
    let past_round_rewards = use_resource(move || async move {
        if is_current_round() {
            return (0u64, 0u64);
        }
        let Ok(miner) = miner() else { return (0, 0) };
        if miner.round_id == 0 || miner.round_id == miner.checkpoint_id {
            return (0, 0);
        }
        let Ok(round) = use_gateway().rpc.get_round(miner.round_id).await else {
            return (0, 0);
        };
        let Some(r) = round.rng() else {
            return (miner.deployed.iter().sum::<u64>(), 0);
        };
        let winning_square = round.winning_square(r);
        if miner.deployed[winning_square] == 0 {
            return (0, 0);
        }
        let total_winnings = round.total_winnings;
        let top_miner_reward = round.top_miner_reward();
        let motherlode = round.motherlode;
        let top_miner_sample = round.top_miner_sample(r, winning_square);
        let is_split_reward = round.is_split_reward_v2(winning_square);
        let is_top_miner = top_miner_sample >= miner.cumulative[winning_square]
            && top_miner_sample < miner.cumulative[winning_square] + miner.deployed[winning_square];
        let mut sol_rewards = ((total_winnings as u128 * miner.deployed[winning_square] as u128)
            / round.deployed[winning_square] as u128) as u64;
        sol_rewards += miner.deployed[winning_square] * 99 / 100;
        let mut ore_rewards = 0u64;
        if is_split_reward {
            ore_rewards += ((top_miner_reward as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        } else if is_top_miner {
            ore_rewards += top_miner_reward;
        }
        if motherlode > 0 {
            ore_rewards += ((motherlode as u128 * miner.deployed[winning_square] as u128)
                / round.deployed[winning_square] as u128) as u64;
        }
        (sol_rewards, ore_rewards)
    });

    // Combine: use optimistic for current round, past_round for settled rounds
    let uncheckpointed_rewards = use_memo(move || {
        if is_current_round() {
            return optimistic_rewards();
        }
        let Some((sol_rewards, ore_rewards)) = past_round_rewards() else {
            return (0, 0);
        };
        (sol_rewards, ore_rewards)
    });

    let rewards_sol = use_memo(move || {
        let Ok(miner) = miner() else { return 0 };
        miner.rewards_sol + uncheckpointed_rewards().0
    });

    let rewards_ore = use_memo(move || {
        let Ok(miner) = miner() else { return 0 };
        miner.rewards_ore + uncheckpointed_rewards().1
    });

    let refined_ore = use_memo(move || {
        let Ok(mut miner) = miner() else { return 0 };
        let Ok(treasury) = treasury() else { return 0 };
        miner.update_rewards(&treasury);
        miner.refined_ore
    });

    let total_ore = use_memo(move || rewards_ore() + refined_ore());

    let has_ore_rewards = use_memo(move || total_ore() > 0);
    let has_sol_rewards = use_memo(move || rewards_sol() > 0);
    let has_rewards = use_memo(move || has_ore_rewards() || has_sol_rewards());

    let claim_sol_tx = use_claim_sol_transaction(uncheckpointed_rewards);

    rsx! {
        Col {
            class: "w-full h-full flex-1 min-h-0 px-4 md:px-8",
            Col {
                gap: 8,
                class: "w-full h-full py-8 max-w-160 mx-auto",
                Heading {
                    class: "w-full h-16 flex-shrink-0",
                    title: "Rewards",
                    subtitle: "Claim your mining rewards.",
                }

                if has_rewards() {
                    Col {
                        class: "w-full gap-8",

                        // ORE claim form
                        if has_ore_rewards() {
                            ClaimOreForm {
                                miner: miner,
                                treasury: treasury,
                                rewards_ore: rewards_ore,
                                refined_ore: refined_ore,
                                uncheckpointed_rewards: uncheckpointed_rewards,
                            }
                        }

                        // Balances section
                        Col {
                            gap: 1,
                            class: "w-full",
                            span {
                                class: "text-elements-midEmphasis font-semibold font-wide text-lg mb-1 px-3",
                                "Balances"
                            }
                            if rewards_ore() > 0 {
                                RewardsInfoRow {
                                    title: "Unrefined ORE".to_string(),
                                    description: "ORE you have mined. Subject to a 10% refining fee when claimed.".to_string(),
                                    value: rsx! {
                                        OreValue {
                                            ui_amount_string: amount_to_ui_amount_string_trimmed(rewards_ore(), TOKEN_DECIMALS),
                                            with_decimal_units: true,
                                            abbreviated: false,
                                            size: Some(TokenValueSize::Medium),
                                            gold: true,
                                        }
                                    }
                                }
                            }
                            if refined_ore() > 0 {
                                RewardsInfoRow {
                                    title: "Refined ORE".to_string(),
                                    description: "ORE earned from refining fees. Not subject to any fee when claimed.".to_string(),
                                    value: rsx! {
                                        OreValue {
                                            ui_amount_string: amount_to_ui_amount_string_trimmed(refined_ore(), TOKEN_DECIMALS),
                                            with_decimal_units: true,
                                            abbreviated: false,
                                            size: Some(TokenValueSize::Medium),
                                            gold: true,
                                        }
                                    }
                                }
                            }
                            if rewards_sol() > 0 {
                                RewardsInfoRow {
                                    title: "SOL".to_string(),
                                    description: "SOL you have earned from mining.".to_string(),
                                    value: rsx! {
                                        SolValue {
                                            ui_amount_string: lamports_to_sol(rewards_sol()).to_string(),
                                            with_decimal_units: true,
                                            abbreviated: false,
                                            size: Some(TokenValueSize::Medium),
                                        }
                                    }
                                }
                                div {
                                    class: "w-full mt-4",
                                    ActionButton {
                                        transaction: claim_sol_tx,
                                        "Claim SOL"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    span {
                        class: "text-elements-lowEmphasis text-sm",
                        "No rewards to claim."
                    }
                }
            }
        }
    }
}

#[component]
fn ClaimOreForm(
    miner: Signal<GatewayResult<Miner>>,
    treasury: Signal<GatewayResult<Treasury>>,
    rewards_ore: Memo<u64>,
    refined_ore: Memo<u64>,
    uncheckpointed_rewards: Memo<(u64, u64)>,
) -> Element {
    let mut pct_input = use_signal(|| "".to_owned());

    let pct = use_memo(move || {
        let val = pct_input().replace('%', "").parse::<u64>().unwrap_or(0);
        val.min(100)
    });

    let bps = use_memo(move || pct() * 100);

    // Preview claim amounts using the same math as Miner::claim_ore
    let claim_preview = use_memo(move || {
        let bps_val = bps();
        if bps_val == 0 {
            return (0u64, 0u64); // (amount_received, fee)
        }

        let Ok(mut m) = miner() else { return (0, 0) };
        let Ok(t) = treasury() else { return (0, 0) };
        m.update_rewards(&t);

        let refined = m.refined_ore;
        let unrefined = m.rewards_ore + uncheckpointed_rewards().1;

        let bps_clamped = bps_val.min(DENOMINATOR_BPS);
        let claim_refined = (refined * bps_clamped) / DENOMINATOR_BPS;
        let claim_rewards = (unrefined * bps_clamped) / DENOMINATOR_BPS;

        let mut fee = 0u64;
        let mut amount = claim_refined + claim_rewards;
        if claim_rewards > 0 {
            fee = 1.max(claim_rewards / 10);
            amount -= fee;
        }

        (amount, fee)
    });

    let claim_ore_tx = use_claim_ore_transaction(bps, uncheckpointed_rewards);

    on_transaction_done(move |_| {
        pct_input.set("".to_owned());
    });

    rsx! {
        Col {
            gap: 0,
            class: "w-full",

            // Large percentage input with dot grid background
            div {
                class: "relative w-full flex flex-col items-center py-4",
                div {
                    class: "absolute inset-0 pointer-events-none",
                    style: "background-image: radial-gradient(circle, rgba(148,163,184,0.2) 1px, transparent 1px); background-size: 12px 12px; background-position: 6px 0; mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%); -webkit-mask-image: radial-gradient(ellipse 65% 65% at 50% 45%, black 0%, transparent 70%);",
                }
                div {
                    class: "relative w-full flex justify-center",
                    input {
                        class: "text-center text-6xl sm:text-7xl font-bold bg-transparent outline-none text-elements-highEmphasis placeholder:text-gray-700 w-full [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                        placeholder: "0%",
                        r#type: "text",
                        inputmode: "numeric",
                        value: pct_input(),
                        oninput: move |e: FormEvent| {
                            let raw = e.value();
                            let had_pct = pct_input().contains('%');
                            let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
                            if had_pct && !raw.contains('%') {
                                // Backspaced over %, also remove last digit
                                let trimmed: String = digits.chars().take(digits.len().saturating_sub(1)).collect();
                                if trimmed.is_empty() {
                                    pct_input.set("".to_owned());
                                } else if let Ok(val) = trimmed.parse::<u64>() {
                                    pct_input.set(format!("{}%", val.min(100)));
                                }
                            } else if digits.is_empty() {
                                pct_input.set("".to_owned());
                            } else if let Ok(val) = digits.parse::<u64>() {
                                pct_input.set(format!("{}%", val.min(100)));
                            }
                        },
                    }
                }
                // Row {
                //     class: "relative text-elements-lowEmphasis gap-1.5 mt-1 border border-transparent rounded-full px-2.5 py-0.5",
                //     OreIcon {
                //         class: "w-4 h-4 my-auto",
                //     }
                //     span {
                //         class: "my-auto text-nowrap text-sm font-semibold",
                //         "{amount_to_ui_amount_string_trimmed(rewards_ore() + refined_ore(), TOKEN_DECIMALS)}"
                //     }
                // }
            }

            // Quick buttons
            div {
                class: "w-full mt-4",
                Row {
                    gap: 2,
                    class: "w-full",
                    button {
                        class: "flex-1 h-12 rounded-full bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis font-semibold transition-all duration-150 cursor-pointer",
                        onclick: move |_| pct_input.set("25%".to_owned()),
                        "25%"
                    }
                    button {
                        class: "flex-1 h-12 rounded-full bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis font-semibold transition-all duration-150 cursor-pointer",
                        onclick: move |_| pct_input.set("50%".to_owned()),
                        "50%"
                    }
                    button {
                        class: "flex-1 h-12 rounded-full bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis font-semibold transition-all duration-150 cursor-pointer",
                        onclick: move |_| pct_input.set("75%".to_owned()),
                        "75%"
                    }
                    button {
                        class: "flex-1 h-12 rounded-full bg-surface-floating hover:bg-surface-floatingControl text-elements-midEmphasis hover:text-elements-highEmphasis font-semibold transition-all duration-150 cursor-pointer",
                        onclick: move |_| pct_input.set("100%".to_owned()),
                        "MAX"
                    }
                }
            }

            // Info rows
            Col {
                gap: 1,
                class: "w-full mt-4",
                RewardsInfoRow {
                    title: "You receive".to_string(),
                    description: "The total ORE you will receive after refining fees.".to_string(),
                    value: rsx! {
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string_trimmed(claim_preview().0, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            abbreviated: false,
                            size: Some(TokenValueSize::Medium),
                            gold: true,
                        }
                    }
                }
                RewardsInfoRow {
                    title: "Refining fee".to_string(),
                    description: "10% fee on unrefined ORE. Refined ORE is not subject to this fee.".to_string(),
                    value: rsx! {
                        OreValue {
                            ui_amount_string: amount_to_ui_amount_string_trimmed(claim_preview().1, TOKEN_DECIMALS),
                            with_decimal_units: true,
                            abbreviated: false,
                            size: Some(TokenValueSize::Medium),
                        }
                    }
                }
            }

            // Claim button
            div {
                class: "w-full mt-4",
                ActionButton {
                    class: "controls-gold",
                    transaction: claim_ore_tx,
                    "Claim ORE"
                }
            }
        }
    }
}

#[component]
fn RewardsInfoRow(title: String, description: String, value: Element) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        Row {
            class: "w-full justify-between items-center h-10 px-3",
            div {
                class: "relative",
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
                span {
                    class: "text-elements-lowEmphasis text-xs font-medium uppercase tracking-wide cursor-help border-b border-dashed border-elements-lowEmphasis hover:text-elements-highEmphasis hover:border-elements-highEmphasis transition-colors duration-200",
                    "{title}"
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
