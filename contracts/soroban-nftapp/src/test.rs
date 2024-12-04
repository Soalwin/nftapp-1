#![cfg(test)]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, Symbol, String};

// Import the NFT contract from your main contract file
use crate::NftContract;

#[cfg(test)]
mod tests {
    use super::*; // Ensure contract is brought into scope

    fn set_up_contract() -> Env {
        Env::default() // Initialize the environment for testing
    }

    #[test]
    fn test_mint_nft() {
        let env = set_up_contract();
        
        // Define test data
        let token_id = Symbol::short("token1");
        let metadata_uri = String::from_slice(&env, "https://example.com/metadata/token1");
        let owner = Symbol::short("user1");
        
        // Call the mint function
        NftContract::mint(env.clone(), token_id.clone(), metadata_uri.clone(), owner.clone());
        
        // Check ownership map for the new token
        let ownership_key = symbol_short!("ownership");
        let ownership: Map<Symbol, Symbol> = env
            .storage()
            .persistent()
            .get(&ownership_key)
            .unwrap_or_else(|| Map::new(&env));
        
        // Assert that the token's owner is correctly set
        assert_eq!(ownership.get(token_id.clone()).unwrap(), owner);

        // Check metadata map for the new token
        let metadata_key = symbol_short!("metadata");
        let metadata: Map<Symbol, String> = env
            .storage()
            .persistent()
            .get(&metadata_key)
            .unwrap_or_else(|| Map::new(&env));

        // Assert that the metadata URI is correctly set
        assert_eq!(metadata.get(token_id.clone()).unwrap(), metadata_uri);
    }

    #[test]
    fn test_transfer_nft() {
        let env = set_up_contract();
        
        // Define test data for minting
        let token_id = Symbol::short("token2");
        let metadata_uri = String::from_slice(&env, "https://example.com/metadata/token2");
        let owner = Symbol::short("user1");
        let new_owner = Symbol::short("user2");
        
        // Mint the NFT
        NftContract::mint(env.clone(), token_id.clone(), metadata_uri.clone(), owner.clone());

        // Now, transfer the NFT to the new owner
        NftContract::transfer(env.clone(), token_id.clone(), new_owner.clone());
        
        // Check the updated ownership map
        let ownership_key = symbol_short!("ownership");
        let ownership: Map<Symbol, Symbol> = env
            .storage()
            .persistent()
            .get(&ownership_key)
            .unwrap_or_else(|| Map::new(&env));
        
        // Assert that the new owner is now set correctly
        assert_eq!(ownership.get(token_id.clone()).unwrap(), new_owner);
    }
}