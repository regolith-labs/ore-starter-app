# ORE Starter App

A starter template for building mining clients in the [ORE](https://ore.com) ecosystem. This is a fork of the production app hosted on ore.com, stripped down and cleaned up for builders to use as a starting point.

Built with [Dioxus](https://dioxuslabs.com/) (Rust + WASM) and [Tailwind CSS](https://tailwindcss.com/).

## Prerequisites

- [Rust](https://rustup.rs/) (1.88.0+, set via `rust-toolchain.toml`)
- WASM target: `rustup target add wasm32-unknown-unknown`
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/) v0.7.3: `cargo install dioxus-cli@0.7.3`
- [Node.js](https://nodejs.org/) (for rebuilding the wallet JS bundle, if needed)

## Configuration

Before running, you may want to update these placeholders:

| File | What | Default |
|------|------|---------|
| `src/gateway/mod.rs` | `DEFAULT_RPC_URL` | `https://api.mainnet-beta.solana.com` (rate-limited; swap for your own RPC) |
| `index.html` | Open Graph meta tags | Points to `ore.com` |
| `assets/manifest.json` | PWA manifest | ORE branding |

## Development

Start the dev server with hot-reload:

```sh
dx serve --web
```

This serves the app at `http://localhost:8080`.

### Wallet JS Bundle

The wallet adapter bridge lives in `scripts/wallet/`. If you modify `scripts/wallet/src/main.js`, rebuild it:

```sh
cd scripts/wallet
npm install
npm run build
```

This outputs `assets/wallet.js`.

## Production Build

Build the optimized release bundle:

```sh
./build.sh
```

This compiles the WASM app and copies all static assets to `target/dx/ore-app/release/web/public/`.

### Serving in Production

A minimal [warp](https://github.com/seanmonstar/warp)-based static file server is included in `serve/`:

```sh
cd serve
cargo build --release
./target/release/ore-app-serve
```

This serves the built app on `http://0.0.0.0:8080` with SPA fallback routing.

## Project Structure

```
src/
  main.rs                  # App entry point, providers, router
  route.rs                 # Route definitions
  pages/                   # Page components (Mine, Rewards)
  components/              # Reusable UI components
    common/                # Icons, buttons, clipboard, etc.
    layout/                # Navigation, footer, headings
    token_value/           # Token display components
    wallet/                # Wallet adapter, picker, drawer
  gateway/                 # RPC client, API calls, websockets
    mod.rs                 # Solana RPC trait and implementation
    ore.rs                 # ORE API client (api.ore.com)
    entropy.rs             # Entropy API client
    wss/                   # WebSocket subscription handling
  hooks/                   # Dioxus hooks and state management
    wallet/                # Wallet connection hooks
    transaction_builders/  # Transaction construction
    wss/                   # WebSocket subscription hooks
  utils/                   # Formatting, time, token helpers
scripts/
  wallet/                  # Wallet adapter JS (React + wallet-adapter)
serve/                     # Production static file server
assets/                    # Static assets (images, fonts, JS bundles)
```

## License

Apache-2.0
