use dioxus::prelude::*;

use crate::components::Row;

#[component]
pub fn TitledRow(title: String, description: String, value: Element) -> Element {
    let mut hovered = use_signal(|| false);
    let mut clicked = use_signal(|| false);
    let is_visible = use_memo(move || hovered() || clicked());

    rsx! {
        Row {
            class: "w-full justify-between gap-2 sm:gap-16 items-center py-2.5",
            // Title with tooltip
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
                span {
                    class: "text-elements-lowEmphasis font-medium text-xs uppercase tracking-wide text-nowrap cursor-help border-b border-dashed border-elements-lowEmphasis hover:text-elements-highEmphasis hover:border-elements-highEmphasis transition-colors duration-200",
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
            // Value
            div {
                class: "shrink-0",
                {value}
            }
        }
    }
}

#[component]
pub fn InfoText(class: Option<String>, text: String, hidden: Signal<bool>) -> Element {
    let class = class.unwrap_or("".to_string());
    let max_height = if hidden() {
        "max-h-0"
    } else {
        "max-h-96"
    };
    let opacity = if hidden() {
        "opacity-0"
    } else {
        "opacity-100"
    };
    rsx! {
        div {
            class: "overflow-hidden transition-all duration-300 ease-in-out h-min {max_height} {class}",
            span {
                class: "block w-full transition-opacity duration-300 ease-in-out pt-2 text-wrap {opacity} text-elements-midEmphasis",
                "{text}"
            }
        }
    }
}
