# Claude Code Guidelines for ore-starter-app

## Dioxus Signal Patterns

### Reading Signals
Always use `signal()` to read signal values. Never use `*signal.read()`.

```rust
// CORRECT
if let Wallet::Connected(address) = wallet() {
    // ...
}

let value = my_signal();

// INCORRECT - Do not use this pattern
if let Wallet::Connected(address) = *wallet.read() {
    // ...
}

let value = *my_signal.read();
```

The `signal()` syntax clones the value immediately and avoids holding borrow references that can cause conflicts with signal updates elsewhere in the app.

### Setting Signals in Async Contexts
When setting signals after async operations that involve `eval()` or cross FFI boundaries (WASM to JS), wrap the signal update in `spawn`:

```rust
// CORRECT
spawn(async move {
    my_signal.set(new_value);
});

// INCORRECT - May panic after crossing FFI boundaries
my_signal.set(new_value);
```

This ensures signal mutations happen in a fresh reactive context.
