use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn Heading(
    class: Option<String>,
    tip: Option<String>,
    title: String,
    subtitle: Option<String>,
    tools: Option<Element>,
) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        Row {
            class: "{class} justify-between",
            Col {
                gap: 2,
                class: "w-full",
                Row {
                    gap: 2,
                    span {
                        class: "font-wide text-3xl font-bold",
                        "{title}"
                    }
                    if let Some(tip) = tip {
                        span {
                            class: "border border-blue-500 bg-blue-500/10 rounded-lg px-2 py-0.5 text-sm my-auto font-semibold",
                            "{tip}"
                        }
                    }
                }
                if let Some(subtitle) = subtitle {
                    span {
                        class: "text-elements-lowEmphasis font-medium",
                        "{subtitle}"
                    }
                }
            }
            if let Some(tools) = tools {
                {tools}
            }
        }
    }
}

#[component]
pub fn Subheading(class: Option<String>, title: String) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        span {
            class: "text-elements-highEmphasis font-semibold text-2xl {class}",
            "{title}"
        }
    }
}
