// Import everything from the main module for testing
use super::*;
use ic_cdk::api::caller; // Import the caller function to get the current users identity

#[cfg(test)] // This module is only included when running tests
mod tests {
    use super::*; // Import everything from the main module

    // A helper function to reset the wallet to a clean state before each test
    fn reset_wallet() {
        unsafe {
            // Set WALLET to a new, empty TokenWallet instance
            WALLET = Some(TokenWallet::default());
        }
    }

    // Test to check if the wallet initializes correctly
    #[test]
    fn test_initialize() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Call the function to initialize the wallet
        unsafe {
            // Make sure the WALLET is now set up
            assert!(WALLET.is_some());
        }
    }

    // Test to see if tokens can be sent successfully
    #[test]
    fn test_send_tokens_success() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Initialize the wallet

        // Give the sender an initial balance
        unsafe {
            let wallet = WALLET.as_mut().unwrap();
            wallet.accounts.insert("sender".to_string(), Token { balance: 100 });
        }

        // Try to send tokens from the sender to the receiver
        let result = send_tokens("receiver".to_string(), 50);
        assert_eq!(result, "Sent 50 tokens to receiver"); // Check if the message is correct
        
        unsafe {
            let wallet = WALLET.as_ref().unwrap();
            // Check if the senders and receiver's balances are updated correctly
            assert_eq!(wallet.accounts.get("sender").unwrap().balance, 50);
            assert_eq!(wallet.accounts.get("receiver").unwrap().balance, 50);
        }
    }

    // Test to see what happens if there are not enough tokens to send
    #[test]
    fn test_send_tokens_insufficient_funds() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Initialize the wallet

        // Set up a sender with not enough tokens
        unsafe {
            let wallet = WALLET.as_mut().unwrap();
            wallet.accounts.insert("sender".to_string(), Token { balance: 10 });
        }

        // Try to send more tokens than the sender has
        let result = send_tokens("receiver".to_string(), 50);
        // Check that the right error message is shown
        assert_eq!(result, "Insufficient funds: 10");
    }

    // Test to check if receiving tokens correctly updates the receivers balance
    #[test]
    fn test_receive_tokens() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Initialize the wallet

        // Try to receive tokens from the sender
        let result = receive_tokens("sender".to_string(), 30);
        assert_eq!(result, "Received 30 tokens from sender"); // Check if the message is correct

        unsafe {
            let wallet = WALLET.as_ref().unwrap();
            // Make sure the receivers balance is updated correctly
            assert_eq!(wallet.accounts.get("receiver").unwrap().balance, 30);
        }
    }

    // Test to check if a user can retrieve their balance
    #[test]
    fn test_get_balance() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Initialize the wallet

        // Give the caller an initial balance
        unsafe {
            let wallet = WALLET.as_mut().unwrap();
            wallet.accounts.insert(caller().to_string(), Token { balance: 100 });
        }

        // Check the balance for the caller
        let balance = get_balance();
        assert_eq!(balance, 100); // Make sure the balance is correct
    }

    // Test to check what happens when trying to get the balance of a non-existent account
    #[test]
    fn test_get_balance_for_nonexistent_account() {
        reset_wallet(); // Start with a fresh wallet
        initialize(); // Initialize the wallet
        
        // Get the balance for an account that doesnt exist; it should be 0
        let balance = get_balance(); 
        assert_eq!(balance, 0); // Confirm that the balance is 0
    }
}
