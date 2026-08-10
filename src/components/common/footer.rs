use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "w-full mt-auto pb-24 lg:pb-8 px-4 pt-24",
        }
    }
}
