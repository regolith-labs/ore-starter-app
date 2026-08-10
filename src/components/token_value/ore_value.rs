use dioxus::prelude::*;

use super::{format_token_amount, TokenValueSize};
use crate::components::{OreIcon, Row};

#[component]
pub fn OreValue(
    class: Option<String>,
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
    gold: Option<bool>,
    size: Option<TokenValueSize>,
    mono: Option<bool>,
    color_override: Option<String>,
    fixed_decimals: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let formatted_amount = format_token_amount(ui_amount_string, with_decimal_units, abbreviated);
    let units: Vec<_> = formatted_amount.split('.').collect();
    let decimal_units = if units.len() > 1 {
        if fixed_decimals.unwrap_or(false) {
            units[1]
        } else {
            units[1].trim_end_matches('0')
        }
    } else {
        ""
    };

    let is_gold = gold.unwrap_or(false);
    let (whole_units_color, decimal_units_color) = if is_gold {
        ("gold-text animate-gold-sheen-slow", "gold-text animate-gold-sheen-slow")
    } else {
        ("text-elements-highEmphasis", "text-elements-highEmphasis")
    };
    let icon_color = if is_gold { "text-elements-gold gold-icon-shimmer" } else { "text-elements-highEmphasis" };

    let font_style = if mono.unwrap_or(false) {
        "font-mono"
    } else {
        ""
    };

    let icon_color = color_override
        .clone()
        .unwrap_or(icon_color.to_string());
    let whole_units_color = color_override
        .clone()
        .unwrap_or(whole_units_color.to_string());
    let decimal_units_color = color_override
        .clone()
        .unwrap_or(decimal_units_color.to_string());

    let (icon_gap, icon_size, whole_units_size, decimal_units_size, font_weight) =
        match size.unwrap_or(TokenValueSize::Small) {
            TokenValueSize::XSmall => ("gap-1", "h-3 w-3", "text-xs", "text-xs", "font-semibold"),
            TokenValueSize::Small => (
                "gap-1",
                "h-3.5 w-3.5",
                "text-sm",
                "text-sm",
                "font-semibold",
            ),
            TokenValueSize::Medium => (
                "gap-1.5",
                "h-4 w-4",
                "text-base",
                "text-base",
                "font-medium",
            ),
            TokenValueSize::Large => ("gap-1.5", "h-4 w-4", "text-lg", "text-lg", "font-semibold"),
            TokenValueSize::XLarge => ("gap-1.5", "h-4 w-4", "text-xl", "text-xl", "font-semibold"),
        };

    rsx! {
        Row {
            class: "w-min {class} {icon_gap} {font_style}",
            OreIcon {
                class: "my-auto {icon_size} {icon_color}"
            }
            Row {
                class: "my-auto",
                span {
                    class: "mt-auto {whole_units_size} {whole_units_color} {font_weight}",
                    "{units[0]}"
                }
                if decimal_units.len() > 0 {
                    span {
                        class: "mt-auto {decimal_units_size} {decimal_units_color} {font_weight}",
                        ".{decimal_units}"
                    }
                }
            }
        }
    }
}
