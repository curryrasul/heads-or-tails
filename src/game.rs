use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::sha256;
use near_sdk::{serde::Serialize, AccountId, Balance};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum GameState {
    Created,
    Initialized,
    Revealed,
    Ended,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub game_state: GameState,

    pub player1: AccountId,
    pub player2: Option<AccountId>,

    pub player1_guess: bool,

    pub deposit: Balance,

    pub player1_commit: Vec<u8>,
    pub player2_commit: Option<Vec<u8>>,

    pub player1_reveal: Option<Vec<u8>>,
    pub player2_reveal: Option<Vec<u8>>,

    pub reveal_time: Option<u64>,

    pub winner: Option<AccountId>,
}

impl Game {
    pub fn commit_reveal(commit: &[u8], reveal: &Vec<u8>) -> bool {
        sha256(commit) == *reveal
    }
}
