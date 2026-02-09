# <repository-name>

## Run on web locally

```cargo run --target wasm32-unknown-unknown```

### Automatically reload on save

```cargo watch -cx "run --target wasm32-unknown-unknown"```

> Append ``WASM_SERVER_RUNNER_ADDRESS=0.0.0.0`` before either to also join web game from network