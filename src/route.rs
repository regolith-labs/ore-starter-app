use crate::components::AppLayout;
use crate::pages::*;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Mine {},
        #[route("/rewards")]
        Rewards {},
    #[end_layout]

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}
