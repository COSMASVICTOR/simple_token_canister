```markdown
# 💰 Simple Token Canister

A basic implementation of an ICP canister smart contract for managing a custom fungible token. Built with Rust using the [IC CDK](https://docs.rs/ic-cdk/) and exposes a Candid interface for interaction via DFINITY's Candid UI.

---

## 📁 Project Structure

```

simple\_token\_canister/
├── src/
│   └── simple\_token\_canister\_backend/
│       ├── lib.rs                       # Main canister logic
│       ├── simple\_token\_canister\_backend.did  # Candid interface
├── dfx.json                             # DFINITY configuration
├── Cargo.toml                           # Rust project config
└── README.md                            # Project documentation

````

---

## 🚀 Features

- Mint tokens to a principal
- Burn tokens
- Transfer tokens between principals
- Query balance
- Get token info (name, symbol, decimals, supply)
- Export Candid interface for Candid UI support

---

## 🛠️ Build & Deploy (Local Development)

### 1. Install Required Tools

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove/)
- [Rust Toolchain](https://rustup.rs/)
- Add the WASM target:
  ```bash
  rustup target add wasm32-unknown-unknown
````

---

### 2. Build the WASM

```bash
cargo build --target wasm32-unknown-unknown --release
```

---

### 3. Start Local Replica

```bash
dfx start --background
```

---

### 4. Deploy Canister

```bash
dfx deploy
```

---

### 5. Open Candid UI

Find your canister ID with:

```bash
dfx canister id simple_token_canister_backend
```

Then open in browser:

```
http://127.0.0.1:4943/?canisterId=<your_canister_id>
```

---

## 🧪 Interface

### DID Summary:

```did
type Result = variant { Ok : text; Err : text };
type TokenInfo = record {
  decimals : nat8;
  name : text;
  total_supply : nat64;
  symbol : text;
};

service : {
  balance_of : (principal) -> (nat64) query;
  burn : (nat64) -> (Result);
  get_all_balances : () -> (vec record { principal; nat64 }) query;
  get_owner : () -> (opt principal) query;
  get_token_info : () -> (TokenInfo) query;
  mint : (principal, nat64) -> (Result);
  my_balance : () -> (nat64) query;
  transfer : (principal, nat64) -> (Result);
}
```

---

