use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, PanicOnDefault, log, env};
use primitive_types::U256;

near_sdk::setup_alloc!();

mod game;
use game::*;

type GameId = u64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    games: UnorderedMap<GameId, Game>,
    next_game_id: GameId,
}

// #[near_bindgen]
impl Contract {
    // #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        log!("Contract initialized");

        Self {
            games: UnorderedMap::new(b'a'),
            next_game_id: 0,
        }
    }

    // pub fn create_game(&mut self, commit: [U256]) -> GameId {
    //     let s = commit.0;

    //     let game_id = self.next_game_id;
    //     self.next_game_id += 1;

    //     game_id
    // }
}
