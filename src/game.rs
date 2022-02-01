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
    pub fn commit_reveal(commit: &[u8], reveal: &[u8]) -> bool {
        sha256(reveal) == *commit
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, MockedBlockchain};

    #[test]
    fn commit_reveal_test() {
        let context = VMContextBuilder::new().build();
        testing_env!(context);

        let bn = 1_000_000_000_000_000u128.to_be_bytes().to_vec();

        let commit = sha256(&bn);

        assert!(Game::commit_reveal(&commit, &bn));
    }
}
