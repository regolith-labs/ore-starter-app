use std::str::FromStr;
use wasm_bindgen_futures;
use web_sys;

pub struct WebClipboard;

impl WebClipboard {
    pub fn new() -> Self {
        WebClipboard
    }

    pub fn set(&self, text: String) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window available")?;
        let navigator = window.navigator();

        // Navigator.clipboard() returns the clipboard object directly
        let clipboard = navigator.clipboard();

        let promise = clipboard.write_text(&text);
        wasm_bindgen_futures::spawn_local(async {
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        });

        Ok(())
    }
}

#[derive(Clone)]
pub enum Splice {
    Pubkey(String),
    Copied,
}

impl ToString for Splice {
    fn to_string(&self) -> String {
        match self {
            Self::Pubkey(pubkey) => pubkey.to_string(),
            Self::Copied => "Copied!".to_string(),
        }
    }
}

impl FromStr for Splice {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let first_four = &s[0..4];
        let last_four = &s[len - 4..len];
        let splice = format!("{}...{}", first_four, last_four);
        Ok(Splice::Pubkey(splice))
    }
}
