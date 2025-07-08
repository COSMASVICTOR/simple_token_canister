use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::caller;
use ic_cdk_macros::*;
use std::collections::HashMap;

// ---------------- Token Structure ----------------
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}

// ---------------- Global State ----------------
thread_local! {
    static BALANCES: std::cell::RefCell<HashMap<Principal, u64>> = std::cell::RefCell::new(HashMap::new());
    static TOKEN_INFO: std::cell::RefCell<TokenInfo> = std::cell::RefCell::new(TokenInfo {
        name: "Simple Token".to_string(),
        symbol: "STK".to_string(),
        decimals: 8,
        total_supply: 0,
    });
    static OWNER: std::cell::RefCell<Option<Principal>> = std::cell::RefCell::new(None);
}

// ---------------- Initialization ----------------
#[init]
fn init() {
    let owner = caller();
    OWNER.with(|o| *o.borrow_mut() = Some(owner));
}

// ---------------- Mint Tokens ----------------
#[update]
#[candid::candid_method(update)]
fn mint(to: Principal, amount: u64) -> Result<String, String> {
    let caller = caller();
    let is_owner = OWNER.with(|o| o.borrow().map_or(false, |owner| owner == caller));
    if !is_owner {
        return Err("Only the owner can mint tokens".to_string());
    }
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let current = *balances.get(&to).unwrap_or(&0);
        balances.insert(to, current + amount);
    });

    TOKEN_INFO.with(|info| info.borrow_mut().total_supply += amount);

    Ok(format!("Successfully minted {} tokens to {}", amount, to))
}

// ---------------- Transfer Tokens ----------------
#[update]
#[candid::candid_method(update)]
fn transfer(to: Principal, amount: u64) -> Result<String, String> {
    let from = caller();
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }
    if from == to {
        return Err("Cannot transfer to yourself".to_string());
    }

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let from_balance = *balances.get(&from).unwrap_or(&0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        balances.insert(from, from_balance - amount);
        let to_balance = *balances.get(&to).unwrap_or(&0);
        balances.insert(to, to_balance + amount);

        Ok(format!("Transferred {} tokens from {} to {}", amount, from, to))
    })
}

// ---------------- Queries ----------------
#[query]
#[candid::candid_method(query)]
fn balance_of(user: Principal) -> u64 {
    BALANCES.with(|b| *b.borrow().get(&user).unwrap_or(&0))
}

#[query]
#[candid::candid_method(query)]
fn my_balance() -> u64 {
    let caller = caller();
    balance_of(caller)
}

#[query]
#[candid::candid_method(query)]
fn get_token_info() -> TokenInfo {
    TOKEN_INFO.with(|info| info.borrow().clone())
}

#[query]
#[candid::candid_method(query)]
fn get_owner() -> Option<Principal> {
    OWNER.with(|o| *o.borrow())
}

#[query]
#[candid::candid_method(query)]
fn get_all_balances() -> Vec<(Principal, u64)> {
    BALANCES.with(|balances| {
        balances
            .borrow()
            .iter()
            .filter(|(_, &bal)| bal > 0)
            .map(|(&p, &b)| (p, b))
            .collect()
    })
}

// ---------------- Burn Tokens ----------------
#[update]
#[candid::candid_method(update)]
fn burn(amount: u64) -> Result<String, String> {
    let caller = caller();
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let current = *balances.get(&caller).unwrap_or(&0);
        if current < amount {
            return Err("Insufficient balance to burn".to_string());
        }

        balances.insert(caller, current - amount);
        TOKEN_INFO.with(|info| info.borrow_mut().total_supply -= amount);

        Ok(format!("Successfully burned {} tokens", amount))
    })
}

// ---------------- Candid Export ----------------
use candid::export_service;
export_service!();

// Optional trait interface (for testing & docs only)
#[cfg(not(target_arch = "wasm32"))]
pub trait TokenInterface {
    fn mint(to: Principal, amount: u64) -> Result<String, String>;
    fn transfer(to: Principal, amount: u64) -> Result<String, String>;
    fn balance_of(user: Principal) -> u64;
    fn my_balance() -> u64;
    fn get_token_info() -> TokenInfo;
    fn get_owner() -> Option<Principal>;
    fn get_all_balances() -> Vec<(Principal, u64)>;
    fn burn(amount: u64) -> Result<String, String>;
}

// ---------------- Generate .did File ----------------
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use std::fs::File;
    use std::io::Write;

    let did = __export_service();
    let mut file = File::create("simple_token_canister_backend.did")
        .expect("Could not create .did file");
    file.write_all(did.as_bytes())
        .expect("Failed to write .did file");
    println!("{}", did);
}

// For wasm32 (IC), no-op
#[cfg(target_arch = "wasm32")]
pub fn main() {}
