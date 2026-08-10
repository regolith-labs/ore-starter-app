use dioxus::prelude::*;

use super::{format_token_amount, TokenValueSize};
use crate::components::Row;

#[component]
pub fn StoreValue(
    class: Option<String>,
    ui_amount_string: String,
    with_decimal_units: Option<bool>,
    abbreviated: Option<bool>,
    gold: Option<bool>,
    size: Option<TokenValueSize>,
    mono: Option<bool>,
    color_override: Option<String>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    let formatted_amount = format_token_amount(ui_amount_string, with_decimal_units, abbreviated);
    let units: Vec<_> = formatted_amount.split('.').collect();
    let decimal_units = if units.len() > 1 {
        units[1].trim_end_matches('0')
    } else {
        ""
    };

    let (whole_units_color, decimal_units_color) = if gold.unwrap_or(false) {
        if abbreviated.unwrap_or(false) {
            ("text-elements-gold", "text-elements-gold")
        } else {
            ("text-elements-gold", "text-elements-gold opacity-50")
        }
    } else {
        if abbreviated.unwrap_or(false) {
            ("text-elements-highEmphasis", "text-elements-highEmphasis")
        } else {
            ("text-elements-highEmphasis", "text-elements-highEmphasis")
        }
    };

    let font_style = if mono.unwrap_or(false) {
        "font-mono"
    } else {
        ""
    };

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
            TokenValueSize::XLarge => ("gap-2", "h-5 w-5", "text-xl", "text-xl", "font-bold"),
        };

    rsx! {
        Row {
            class: "w-fit {class} {icon_gap} {font_style}",
            img {
                src: asset!("/assets/icon-lst.png"),
                class: "my-auto {icon_size} {whole_units_color}",
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
