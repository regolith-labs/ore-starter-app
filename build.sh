#!/bin/bash

# Install dx 0.7.3 if not already installed
REQUIRED_DX_VERSION="0.7.3"
INSTALLED_DX_VERSION=$(dx --version 2>/dev/null | awk '{print $2}')
if [ "$INSTALLED_DX_VERSION" != "$REQUIRED_DX_VERSION" ]; then
  echo "dx version $REQUIRED_DX_VERSION required, found '${INSTALLED_DX_VERSION:-not installed}'. Installing..."
  cargo install dioxus-cli --version 0.7.3 --locked --force
fi

# Verify dx version is 0.7.3
INSTALLED_DX_VERSION=$(dx --version 2>/dev/null | awk '{print $2}')
if [ "$INSTALLED_DX_VERSION" != "$REQUIRED_DX_VERSION" ]; then
  echo "Error: dx version $REQUIRED_DX_VERSION is required, but found '${INSTALLED_DX_VERSION:-not installed}'"
  exit 1
fi

# Set target and build
rustup target add wasm32-unknown-unknown
dx build --web --release

# Copy static assets to web/public
cp assets/icon.png target/dx/ore-app/release/web/public/assets
cp assets/metadata.json target/dx/ore-app/release/web/public/assets
cp assets/icon-lst.png target/dx/ore-app/release/web/public/assets
cp assets/metadata-lst.json target/dx/ore-app/release/web/public/assets
cp assets/favicon.png target/dx/ore-app/release/web/public/assets
cp assets/apple-touch-icon.png target/dx/ore-app/release/web/public/assets
cp assets/apple-touch-icon-152x152.png target/dx/ore-app/release/web/public/assets
cp assets/apple-touch-icon-167x167.png target/dx/ore-app/release/web/public/assets
cp assets/apple-touch-icon-180x180.png target/dx/ore-app/release/web/public/assets
cp assets/opengraph.png target/dx/ore-app/release/web/public/assets

# Copy JS bundles (no hash suffix).
cp assets/vendor/spl-memo.global.js target/dx/ore-app/release/web/public/assets
cp assets/vendor/spl-system.global.js target/dx/ore-app/release/web/public/assets
cp assets/vendor/spl-token.global.js target/dx/ore-app/release/web/public/assets
cp assets/wallet.js target/dx/ore-app/release/web/public/assets

# Copy brand assets.
mkdir -p target/dx/ore-app/release/web/public/assets/brand
mkdir -p target/dx/ore-app/release/web/public/assets/brand/colors
mkdir -p target/dx/ore-app/release/web/public/assets/brand/logo
mkdir -p target/dx/ore-app/release/web/public/assets/brand/logo-wordmark
mkdir -p target/dx/ore-app/release/web/public/assets/brand/tokens
mkdir -p target/dx/ore-app/release/web/public/assets/brand/wordmark
cp assets/brand/colors/pallette.png target/dx/ore-app/release/web/public/assets/brand/colors
cp assets/brand/colors/pallette.svg target/dx/ore-app/release/web/public/assets/brand/colors
cp assets/brand/logo/logo-black.png target/dx/ore-app/release/web/public/assets/brand/logo
cp assets/brand/logo/logo-black.svg target/dx/ore-app/release/web/public/assets/brand/logo
cp assets/brand/logo/logo-white.png target/dx/ore-app/release/web/public/assets/brand/logo
cp assets/brand/logo/logo-white.svg target/dx/ore-app/release/web/public/assets/brand/logo
cp assets/brand/logo-wordmark/logo-wordmark-black.png target/dx/ore-app/release/web/public/assets/brand/logo-wordmark
cp assets/brand/logo-wordmark/logo-wordmark-black.svg target/dx/ore-app/release/web/public/assets/brand/logo-wordmark
cp assets/brand/logo-wordmark/logo-wordmark-white.png target/dx/ore-app/release/web/public/assets/brand/logo-wordmark
cp assets/brand/logo-wordmark/logo-wordmark-white.svg target/dx/ore-app/release/web/public/assets/brand/logo-wordmark
cp assets/brand/tokens/ore.png target/dx/ore-app/release/web/public/assets/brand/tokens
cp assets/brand/tokens/ore.svg target/dx/ore-app/release/web/public/assets/brand/tokens
cp assets/brand/wordmark/wordmark-black.png target/dx/ore-app/release/web/public/assets/brand/wordmark
cp assets/brand/wordmark/wordmark-black.svg target/dx/ore-app/release/web/public/assets/brand/wordmark
cp assets/brand/wordmark/wordmark-white.png target/dx/ore-app/release/web/public/assets/brand/wordmark
cp assets/brand/wordmark/wordmark-white.svg target/dx/ore-app/release/web/public/assets/brand/wordmark

# Print assets for debugging
ls -la target/dx/ore-app/release/web/public/assets
