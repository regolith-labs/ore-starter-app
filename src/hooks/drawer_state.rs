use dioxus::prelude::*;

// Wrapper type for wallet drawer state
#[derive(Clone, Copy, PartialEq)]
pub struct IsWalletDrawerOpen(pub bool);

// Provider hook for wallet drawer visibility
pub fn use_wallet_drawer_state_provider() {
    use_context_provider::<Signal<IsWalletDrawerOpen>>(|| Signal::new(IsWalletDrawerOpen(false)));
}

// Hook to get or set the wallet drawer state
pub fn use_wallet_drawer_state() -> Signal<IsWalletDrawerOpen> {
    use_context::<Signal<IsWalletDrawerOpen>>()
}
