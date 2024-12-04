#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, Symbol, String};

#[contract]
pub struct NftContract;

#[contractimpl]
impl NftContract {
    pub fn mint(env: Env, token_id: Symbol, metadata_uri: String, owner: Symbol) {
        let ownership_key = symbol_short!("ownership");
        let metadata_key = symbol_short!("metadata");

        // Retrieve or create a new ownership map
        let mut ownership: Map<Symbol, Symbol> = env
            .storage()
            .persistent()
            .get(&ownership_key)
            .unwrap_or_else(|| Map::new(&env));
        
        // Retrieve or create a new metadata map
        let mut metadata: Map<Symbol, String> = env
            .storage()
            .persistent()
            .get(&metadata_key)
            .unwrap_or_else(|| Map::new(&env));

        // Ensure the token ID is unique
        if ownership.contains_key(token_id.clone()) {
            panic!("Token ID already exists");
        }

        // Set the owner and metadata URI in the maps
        ownership.set(token_id.clone(), owner);
        metadata.set(token_id.clone(), metadata_uri);

        // Save the updated maps back to storage
        env.storage().persistent().set(&ownership_key, &ownership);
        env.storage().persistent().set(&metadata_key, &metadata);
    }

    pub fn transfer(env: Env, token_id: Symbol, new_owner: Symbol) {
        let ownership_key = symbol_short!("ownership");

        // Retrieve the existing ownership map
        let mut ownership: Map<Symbol, Symbol> = env
            .storage()
            .persistent()
            .get(&ownership_key)
            .unwrap_or_else(|| Map::new(&env));

        // Ensure the token ID exists
        if !ownership.contains_key(token_id.clone()) {
            panic!("Token ID does not exist");
        }

        // Update the owner in the map
        ownership.set(token_id, new_owner);

        // Save the updated ownership map back to storage
        env.storage().persistent().set(&ownership_key, &ownership);
    }

    pub fn get_metadata(env: Env, token_id: Symbol) -> String {
        let metadata_key = symbol_short!("metadata");

        // Retrieve the existing metadata map
        let metadata: Map<Symbol, String> = env
            .storage()
            .persistent()
            .get(&metadata_key)
            .unwrap_or_else(|| Map::new(&env));

        // Ensure the token ID exists and return its metadata URI
        metadata
            .get(token_id.clone())
            .unwrap_or_else(|| panic!("Token ID does not exist"))
    }
}

mod test;

