use heads_or_tails::*;

use near_sdk::env::sha256;
use near_sdk::json_types::ValidAccountId;
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};

fn to_valid_account(predecessor: String) -> ValidAccountId {
    ValidAccountId::try_from(predecessor).expect("Invalid account name")
}

fn get_context(predecessor: String) -> VMContext {
    VMContextBuilder::new()
        .predecessor_account_id(to_valid_account(predecessor))
        .build()
}

#[test]
fn first_winner_honest() {
    let context = get_context("magamedrasul.near".to_string());
    testing_env!(context);

    // let mut contract = Contract::new();

    let player1_reveal = 1_000_000_000_000u128.to_be_bytes();
    let player1_commit = sha256(&player1_reveal);

    // let game_id = contract.create_game(true, player1_commit);

    let player2_reveal = 1_000_000_000_500u128.to_be_bytes();
    let player2_commit = sha256(&player2_reveal);

    println!("{:?}", player1_commit);

    // contract.join_game(game_id, player2_commit);

    // contract.first_reveal(game_id, player1_reveal.to_vec());
    // contract.second_reveal(game_id, player2_reveal.to_vec());
}

#[test]
fn second_winner_honest() {}

#[test]
fn first_cheat() {}

#[test]
fn second_cheat() {}

#[test]
fn second_late() {}
