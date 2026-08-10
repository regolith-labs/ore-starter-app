use std::rc::Rc;

use dioxus::prelude::*;

use crate::gateway::{Gateway, WebRpc};
use crate::hooks::use_rpc_url::use_rpc_url;

pub fn use_gateway() -> Rc<Gateway<WebRpc>> {
    let rpc_url = use_rpc_url();
    let url = rpc_url.peek().clone();
    Rc::new(Gateway::new(url))
}
