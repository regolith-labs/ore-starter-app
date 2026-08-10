use dioxus::prelude::*;

use crate::{
    components::{format_token_amount, Row},
    gateway::Token,
};

#[component]
pub fn TokenValueSmall(
    class: Option<String>,
    // amount: String,
    token: Token,
    with_decimal_units: Option<bool>,
) -> Element {
    let class = class.unwrap_or("".to_string());

    let ui_amount = token.ui_amount();
    let formatted_amount = format_token_amount(ui_amount, with_decimal_units, Some(false));
    let units: Vec<_> = formatted_amount.split('.').collect();

    rsx! {
        Row {
            class: "gap-1.5 {class}",
            img {
                class: "w-6 h-6 my-auto bg-gray-900 rounded-full border border-gray-800",
                src: "{token.image}"
            }
            span {
                class: "my-auto font-medium",
                "{units[0]}"
                if with_decimal_units.unwrap_or(false) {
                    span {
                        class: "mt-auto font-medium text-elements-lowEmphasis",
                        ".{units[1]}"
                    }
                }
            }
        }
    }
}
