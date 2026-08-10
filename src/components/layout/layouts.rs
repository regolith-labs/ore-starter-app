use dioxus::prelude::*;

use crate::components::*;
use crate::route::Route;

use super::navigation::*;

pub fn AppLayout() -> Element {
    let current_route = use_route();
    let container_class = match current_route {
        Route::Mine {} => "flex-1 h-full w-screen",
        _ => "flex-1 h-full max-w-320 mx-auto pb-48 md:pb-24 lg:pb-16",
    };

    rsx! {
        Col {
            class: "w-screen min-h-dvh h-full flex-1",

            // Navigation
            AppNavBar { tabs: true }
            MobileTabBar {}

            // Content
            Row {
                class: "flex-1 h-full min-h-0",
                div {
                    class: "{container_class} ",
                    Outlet::<Route> {}
                }
            }

            // Notifications
            ToastDisplay {}
        }
    }
}
