#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

#[test]
fn test_register_and_resolve_identity() {
    let env = Env::default();
    let contract_id = env.register(RoyaltyContract, ());
    let client = RoyaltyContractClient::new(&env, &contract_id);

    let addr = Address::generate(&env);
    let id_hash = BytesN::from_array(&env, &[7u8; 32]);

    env.mock_all_auths(); 
    client.register_identity(&id_hash, &addr);

    let resolved = client.resolve_identity(&id_hash);
    assert_eq!(resolved, addr);
}
