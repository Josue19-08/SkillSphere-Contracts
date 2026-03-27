#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, Address, Address, ReputationScoringContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(ReputationScoringContract, ());
    let client = ReputationScoringContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let vault = Address::generate(&env);
    (env, admin, vault, client)
}

#[test]
fn test_initialize() {
    let (_env, admin, vault, client) = setup();
    client.init(&admin, &vault);
}

#[test]
fn test_initialize_twice_fails() {
    let (_env, admin, vault, client) = setup();
    client.init(&admin, &vault);
    assert!(client.try_init(&admin, &vault).is_err());
}

#[test]
fn test_transfer_admin() {
    let (env, admin, vault, client) = setup();
    client.init(&admin, &vault);
    let new_admin = Address::generate(&env);
    client.transfer_admin(&new_admin);
}

#[test]
fn test_pause_blocks_transfer_admin() {
    let (env, admin, vault, client) = setup();
    client.init(&admin, &vault);
    client.pause();
    let new_admin = Address::generate(&env);
    assert!(client.try_transfer_admin(&new_admin).is_err());
}

#[test]
fn test_unpause_restores_transfer_admin() {
    let (env, admin, vault, client) = setup();
    client.init(&admin, &vault);
    client.pause();
    client.unpause();
    let new_admin = Address::generate(&env);
    client.transfer_admin(&new_admin);
}
