// Import necessary libraries and modules
use std::collections::HashMap; // Used for creating hash maps to store user accounts and their balances
use candid::{CandidType, Deserialize}; // for defining types to work with the Candid interface
use ic_cdk_macros::{update, query, init}; // Macros for defining functions in the Canister
use ic_cdk::api::caller; // To retrieve the identity of the caller

// Defining a struct to represent a Token, which holds the balance of a user
#[derive(Clone, CandidType, Deserialize, Debug, Default)]
struct Token {
    balance: u64, // balance of tokens in the wallet
}

// Define a struct for the TokenWallet, which contains user accounts
#[derive(Default)]
struct TokenWallet {
    accounts: HashMap<String, Token>, // Maps user addresses to their respective Token balances
}

// Declare a static mutable variable to hold the wallet instance
static mut WALLET: Option<TokenWallet> = None;

// Initialization function that sets up the wallet when the canister is created
#[init]
fn initialize() {
    unsafe {
        // Create a new TokenWallet instance and assign it to WALLET
        WALLET = Some(TokenWallet::default());
    }
}

// Function to send tokens from the caller's account to another account
#[update]
fn send_tokens(to: String, amount: u64) -> String {
    let sender = caller().to_string(); // Get the senders address

    unsafe {
        // access the wallet instance
        let wallet = WALLET.as_mut().unwrap();

        // get the senders current balance, defaulting to 0 if not found
        let sender_balance = wallet.accounts.entry(sender.clone()).or_default().balance;
        
        // Check if the sender has enough funds to send
        if sender_balance < amount {
            return format!("Insufficient funds: {}", sender_balance);
        }

        // Deduct the amount from the senders balance
        wallet.accounts.get_mut(&sender).unwrap().balance -= amount;

        // Add the amount to the recipients balance, creating the entry if it doesnt exist
        wallet.accounts.entry(to.clone()).or_insert_with(|| Token { balance: 0 }).balance += amount;

        format!("Sent {} tokens to {}", amount, to) // return a success message
    }
}

// Function to receive tokens and update the receivers balance
#[update]
fn receive_tokens(from: String, amount: u64) -> String {
    let receiver = caller().to_string(); // Get the receivers address

    unsafe {
        // Access the wallet instance
        let wallet = WALLET.as_mut().unwrap();
        
        // add the amount to the receivers balance, creating the entry if it doesnt exist
        wallet.accounts.entry(receiver.clone()).or_insert_with(|| Token { balance: 0 }).balance += amount;

        format!("Received {} tokens from {}", amount, from) // return a success message
    }
}

// Query function to get the balance of the callers account
#[query]
fn get_balance() -> u64 {
    let caller = caller().to_string(); // Get the callers address

    unsafe {
        // Access the wallet instance
        let wallet = WALLET.as_ref().unwrap();
        
        // Retrieve the balance for the caller, defaulting to 0 if not found
        wallet.accounts.get(&caller).unwrap_or(&Token { balance: 0 }).balance
    }
}

// Query function to get all balances in the wallet
#[query]
fn get_all_balances() -> HashMap<String, u64> {
    unsafe {
        // Collect balances for all accounts into a new HashMap
        WALLET.as_ref()
            .unwrap()
            .accounts
            .iter()
            .map(|(k, v)| (k.clone(), v.balance)) // Map each account to its balance
            .collect() // Collect into a HashMap
    }
}
