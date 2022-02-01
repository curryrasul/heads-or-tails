use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::UnorderedMap, env, log, near_bindgen, Balance, PanicOnDefault, Promise,
};

near_sdk::setup_alloc!();

mod game;
use game::*;

type GameId = u64;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

// const ONE_SECOND: u64 = 1_000_000_000;
// const ONE_MINUTE: u64 = 60 * ONE_SECOND;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    games: UnorderedMap<GameId, Game>,
    next_game_id: GameId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        log!("Contract initialized");

        Self {
            games: UnorderedMap::new(b'a'),
            next_game_id: 0,
        }
    }

    #[payable]
    pub fn create_game(&mut self, guess: bool, commit: Vec<u8>) -> GameId {
        let amount = env::attached_deposit();

        assert!(amount >= ONE_NEAR, "Deposit have to be >= than 1 NEAR");

        let game_id = self.next_game_id;

        let game = Game {
            game_state: GameState::Created,
            player1: env::predecessor_account_id(),
            player2: None,
            player1_guess: guess,
            deposit: amount,
            player1_commit: commit,
            player2_commit: None,
            player1_reveal: None,
            player2_reveal: None,
            reveal_time: None,
            winner: None,
        };

        self.games.insert(&game_id, &game);
        log!(
            "Player {} created the game with GameId: {} and deposit: {} NEAR. His guess: {}",
            env::predecessor_account_id(),
            game_id,
            amount,
            if guess { "head" } else { "tail" }
        );

        self.next_game_id += 1;

        game_id
    }

    #[payable]
    pub fn join_game(&mut self, game_id: GameId, commit: Vec<u8>) {
        let amount = env::attached_deposit();

        assert!(
            self.games.get(&game_id).is_some(),
            "No game with such GameId"
        );

        let mut game = self.games.get(&game_id).unwrap();

        if let GameState::Created = game.game_state {
            assert!(
                game.deposit <= amount,
                "Wrong deposit. Player1's bet is {} NEAR",
                game.deposit
            );

            if amount > game.deposit {
                let refund = amount - game.deposit;
                Promise::new(env::predecessor_account_id()).transfer(refund);

                log!(
                    "Refunded {} NEAR to {}",
                    refund,
                    env::predecessor_account_id()
                );
            }

            game.player2 = Some(env::predecessor_account_id());
            game.game_state = GameState::Initialized;
            game.player2_commit = Some(commit);

            self.games.insert(&game_id, &game);

            log!(
                "Player {} joined the game {}. His guess: {}",
                env::predecessor_account_id(),
                game_id,
                if game.player1_guess { "tail" } else { "head" }
            );
        } else {
            panic!("Game is not active");
        }
    }
}
