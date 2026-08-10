use dioxus::prelude::*;

use crate::{
    components::*,
    hooks::{use_nav_border_on_scroll, use_ore_price, use_sol_price, use_wallet_drawer_state},
    route::Route,
};

#[component]
pub(crate) fn AppNavBar(tabs: bool) -> Element {
    let nav_border = use_nav_border_on_scroll();
    let border_color = if nav_border() {
        "border-gray-800"
    } else {
        "border-transparent"
    };

    rsx! {
        Row {
            class: "w-screen h-16 sm:h-20 px-2 sm:px-6 sm:sticky sm:top-0 bg-black z-50 shrink-0 border-b {border_color} transition-colors duration-200",
            Row {
                class: "items-center w-full my-auto sm:flex sm:justify-between sm:relative",

                // Desktop: logo on the left
                span {
                    class: "hidden lg:flex my-auto",
                    Logo {}
                }

                // Navigation links
                Row {
                    gap: 4,
                    class: "px-8 hidden lg:flex mr-auto text-nowrap items-center",
                    NavLink {
                        title: "Mine".to_string(),
                        route: Route::Mine {},
                    }
                    NavLink {
                        title: "Rewards".to_string(),
                        route: Route::Rewards {},
                    }
                }

                Row {
                    gap: 2,
                    class: "flex-1 px-2 lg:px-0 flex items-center gap-2 lg:gap-3 justify-between lg:justify-end",
                    Row {
                        gap: 2,
                        OrePrice {}
                        span {
                            class: "hidden lg:flex",
                            SolPrice {}
                        }
                    }
                    span {
                        WalletAdapter {}
                    }
                }
            }
        }
    }
}

#[component]
fn SolPrice() -> Element {
    let sol_price = use_sol_price();

    rsx! {
        if let Some(sol_price) = sol_price() {
            Link {
                gap: 1,
                class: "my-auto text-sm flex flex-row rounded-full gap-1 hover:bg-controls-secondaryHover py-2 px-4 transition-colors duration-300 ease-in-out",
                to: "https://jup.ag/tokens/So11111111111111111111111111111111111111112",
                new_tab: true,
                img {
                    src: asset!("/assets/solana.png"),
                    class: "h-4 w-4 my-auto"
                }
                span {
                    class: "text-elements-highEmphasis font-semibold my-auto",
                    "SOL"
                }
                span {
                    class: "text-elements-lowEmphasis font-semibold my-auto",
                    "${sol_price:.2}"
                }
            }
        }
    }
}

#[component]
fn OrePrice() -> Element {
    let ore_price = use_ore_price();

    rsx! {
        if let Some(ore_price) = ore_price() {
            Link {
                gap: 1,
                class: "my-auto text-sm flex flex-row rounded-full gap-2 lg:gap-1 hover:bg-controls-secondaryHover py-2 lg:px-4 px-2 transition-colors duration-300 ease-in-out",
                to: "https://jup.ag/tokens/oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp",
                new_tab: true,
                OreIcon {
                    class: "h-5 w-5 lg:h-4 lg:w-4 my-auto"
                }
                span {
                    class: "text-elements-highEmphasis font-semibold my-auto hidden lg:flex",
                    "ORE"
                }
                span {
                    class: "text-elements-lowEmphasis font-semibold lg:my-auto mt-0.5",
                    "${ore_price:.2}"
                }
            }
        }
    }
}

#[component]
pub(crate) fn NavLink(title: String, route: Route) -> Element {
    let current_route: Route = use_route::<Route>();
    let is_selected = route == current_route;

    let text_class = if is_selected {
        "text-elements-highEmphasis"
    } else {
        "text-elements-lowEmphasis hover:text-elements-highEmphasis"
    };

    let underline_class = if is_selected {
        "border-b-2 border-white"
    } else {
        "border-b-2 border-transparent"
    };

    rsx! {
        Link {
            class: "inline-flex h-10 flex-col items-stretch transition-colors duration-300 ease-in-out",
            to: route,
            div {
                class: "{text_class} flex min-h-0 flex-1 items-center rounded-md px-2 hover:bg-controls-secondaryHover",
                span {
                    class: "font-semibold",
                    "{title}"
                }
            }
            div {
                class: "{underline_class} w-full shrink-0 rounded-none",
            }
        }
    }
}

pub fn Logo() -> Element {
    rsx! {
        Link {
            class: "p-1 my-auto w-min h-min rounded hover:bg-controls-secondaryHover transition-colors duration-300 ease-in-out",
            to: Route::Mine {},
            OreWordmarkIcon {
                class: "h-5"
            }
        }
    }
}

pub(crate) fn MobileTabBar() -> Element {
    let current_route: Route = use_route();
    let drawer_state = use_wallet_drawer_state();

    let hidden = drawer_state.read().0;
    let hidden_class = if hidden { "hidden" } else { "" };

    rsx! {
        Row {
            class: "{hidden_class} lg:hidden fixed bottom-0 w-full elevated z-50 items-end h-16",
            MobileTabButton {
                route: Route::Mine {},
                title: "Mine",
                icon: rsx! { SquaresSolidIcon { class: "h-5 w-5 mx-auto" } },
            }
            MobileTabButton {
                route: Route::Rewards {},
                title: "Rewards",
                icon: rsx! { OreIcon { class: "h-5 w-5 mx-auto" } },
            }
        }
    }
}

#[component]
fn MobileTabButton(route: Route, title: String, icon: Element) -> Element {
    let current_route: Route = use_route();
    let is_selected = route == current_route;

    let color = if !is_selected {
        "text-elements-lowEmphasis hover:text-elements-midEmphasis"
    } else {
        ""
    };

    rsx! {
        Link {
            class: "flex h-16 w-full",
            to: route,
            Col {
                class: "mx-auto my-auto {color}",
                gap: 1,
                {icon}
                span {
                    class: "mx-auto font-medium text-xs",
                    "{title}"
                }
            }
        }
    }
}
