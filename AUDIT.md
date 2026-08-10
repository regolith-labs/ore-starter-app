# Pre-Publication Audit

Review of the codebase for secrets, sensitive data, and items to clean up before making this repo public.

---

## CRITICAL: Secrets & API Keys

### 1. Ironforge RPC API Key (hardcoded in 2 places)
- **`src/gateway/mod.rs:23`** — `apiKey=01J4NJDYJXSGJYE3AN6VXEB5VR` in `DEFAULT_RPC_URL`
- **`scripts/wallet/src/main.js:72-73`** — Same API key used in Privy's Solana RPC config (both HTTP and WSS URLs)

**Action:** Replace with a placeholder like `YOUR_RPC_API_KEY` or read from an environment variable. Rotate the existing key since it's already in git history.

### 2. Privy App ID (hardcoded)
- **`scripts/wallet/src/main.js:31`** — `PRIVY_APP_ID = 'cmjbrsqs102pela0cdvv9tpj9'`

**Action:** This is your production Privy app ID. Replace with a placeholder or env var. While Privy app IDs are semi-public (they appear in client-side code), exposing your production one in a starter template means other people's apps will authenticate against *your* Privy project.

### 3. Android App Signing Certificate Fingerprint
- **`assets/.well-known/assetlinks.json`** — Contains `sha256_cert_fingerprints` for `supply.ore.app`

**Action:** Remove or replace. This is specific to your production Android TWA app and should not be in a starter template.

---

## HIGH: Production-Specific Configuration

### 4. Hardcoded `ore.com` URLs
These tie the starter app to your production infrastructure:
- **`src/gateway/ore.rs:56`** — `ORE_API_URL = "https://api.ore.com"`
- **`src/hooks/use_ore_price.rs:10`** — `"https://api.ore.com/jupiter/price"`
- **`src/hooks/use_sol_price.rs:9`** — `"https://api.ore.com/jupiter/price"`
- **`src/gateway/ore.rs:519`** — `"https://ore.com/auth/discord"` redirect URI
- **`src/components/common/footer.rs:12,17,22`** — Links to `ore.com/about`, `/privacy`, `/terms`
- **`index.html:13,16`** — Open Graph URLs pointing to `ore.com`
- **`assets/manifest.json:13`** — Icon URL pointing to `ore.com`

**Action:** Replace with placeholders or make configurable. The `api.ore.com` endpoints won't work for third-party builders anyway.

### 5. Hardcoded Entropy API URL
- **`src/gateway/entropy.rs:8`** — `"https://entropy-api.onrender.com"`

**Action:** Replace with placeholder or env var.

### 6. Hardcoded Privacy/Relayer API URL
- **`scripts/privacy/src/utils/constants.ts:20`** — `"https://api3.privacycash.org"` (default for `RELAYER_API_URL`)

**Action:** Replace with placeholder.

### 7. Discord Invite Link
- **`src/components/layout/navigation.rs:60`** — `"https://discord.com/invite/4TQfshAAsT"`

**Action:** Decide if you want your Discord invite link in the starter app.

---

## MEDIUM: Files to Remove

### 8. `changelog/` directory (19 files)
Internal development changelogs with details about Privy wallet migration, TEE wallet signing investigations, Apple Pay onramp work, chat features, etc. These expose internal development history and are not relevant for a starter app.

**Action:** Remove the entire `changelog/` directory.

### 9. `DEAD_CODE.md`
Internal dead code tracking document referencing components, pages, and gateway modules from the full ore.com app.

**Action:** Remove.

### 10. `.claude/worktrees/v4-demo` (tracked by git)
A Claude Code worktree directory is tracked in git. Contains a variant of the codebase with additional features (Discord gateway, terms page, shielded balances, etc.).

**Action:** Remove from git tracking. Add `.claude/` to `.gitignore`.

### 11. `screenshots/` directory
Empty directory (just a `.DS_Store`). Tracked by git.

**Action:** Remove.

### 12. `.DS_Store` files
Multiple `.DS_Store` files are tracked in git:
- Root `.DS_Store`
- `screenshots/.DS_Store`
- `assets/.DS_Store`
- `assets/fonts/.DS_Store`
- `scripts/wallet/.DS_Store`

**Action:** Remove from git and add `.DS_Store` pattern to `.gitignore` (it's already in `.gitignore` at root level, but the nested ones are tracked).

---

## MEDIUM: Licensed / Commercial Fonts

### 13. Commercial font files tracked in git
- `assets/fonts/Copernicus/CopernicusTrial-Book.otf` — **Trial** font (note "Trial" in filename)
- `assets/fonts/Pilat/` — 5 weights
- `assets/fonts/PilatExtended/` — 5 weights
- `assets/fonts/PilatWide/` — 5 weights
- `assets/fonts/TiemposText/TestTiemposText-Regular.otf` — **Test** font (note "Test" in filename)

**Action:** These are likely commercially licensed fonts. Distributing them in a public repo may violate license terms. Remove and replace with open-source alternatives, or confirm licensing allows redistribution.

---

## LOW: Code Cleanup

### 14. Commented-out localhost/dev URLs
- **`src/gateway/ore.rs:520`** — `// redirect_uri: "http://localhost:8080/auth/discord".to_string(), // dev`

**Action:** Remove commented-out dev URLs.

### 15. Sticker assets
`assets/stickers/` contains 30+ meme/chat sticker images. These are specific to the ore.com chat feature and add unnecessary bloat to a starter app.

**Action:** Consider removing if chat isn't part of the starter app.

### 16. Privacy/circuit assets
Large binary assets for zero-knowledge proofs:
- `assets/circuit/`, `assets/circuit2/` — WASM and zkey files
- `assets/privacy.js`, `assets/privacy-*.js` — Privacy JS bundles
- `scripts/privacy/` — Full privacy SDK source
- `assets/*.wasm`, `assets/*.zkey` — Additional circuit files at root

**Action:** Remove if privacy features aren't part of the starter app.

### 17. `build.sh` references ore.com-specific assets
The build script copies stickers, brand assets, privacy bundles, and `.well-known` files that are specific to the production ore.com deployment.

**Action:** Simplify for the starter app.

### 18. Brand assets
`assets/brand/` contains the official ORE brand kit (logos, wordmarks, color palette). Fine to include if intended, but consider whether starter app builders should use these.

---

## Summary of Required Actions (Priority Order)

| Priority | Item | Action |
|----------|------|--------|
| CRITICAL | Ironforge API key | Remove & rotate key |
| CRITICAL | Privy App ID | Remove or replace with placeholder |
| CRITICAL | Android cert fingerprint | Remove `assetlinks.json` |
| HIGH | Hardcoded `ore.com`/`api.ore.com` URLs | Replace with placeholders |
| HIGH | Entropy & Privacy API URLs | Replace with placeholders |
| MEDIUM | `changelog/` directory | Delete |
| MEDIUM | `DEAD_CODE.md` | Delete |
| MEDIUM | `.claude/worktrees/` | Remove from git, add to `.gitignore` |
| MEDIUM | `.DS_Store` files | Remove from git |
| MEDIUM | Commercial fonts | Remove or verify license |
| LOW | Sticker assets | Remove if chat not in starter |
| LOW | Privacy/circuit assets | Remove if privacy not in starter |
| LOW | Commented-out dev URLs | Clean up |
