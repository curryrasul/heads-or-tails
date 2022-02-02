use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::UnorderedMap, env, log, near_bindgen, Balance, PanicOnDefault, Promise,
};

near_sdk::setup_alloc!();

mod game;
pub use game::*;

type GameId = u64;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;

const ONE_SECOND: u64 = 1_000_000_000;
const ONE_MINUTE: u64 = 60 * ONE_SECOND;

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
        assert!(
            commit.len() == 32,
            "Invalid commit format. Must be byte-vector with size 32"
        );

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
        assert!(
            commit.len() == 32,
            "Invalid commit format. Must be byte-vector with size 32"
        );

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

    pub fn first_reveal(&mut self, game_id: GameId, reveal: Vec<u8>) {
        assert!(
            reveal.len() == 16,
            "Invalid reveal format. Must be byte-vector with size 16"
        );

        assert!(
            self.games.get(&game_id).is_some(),
            "No game with such GameId"
        );

        let mut game = self.games.get(&game_id).unwrap();

        if let GameState::Initialized = game.game_state {
            assert_eq!(
                game.player1.clone(),
                env::predecessor_account_id(),
                "Player1 is not {}",
                env::predecessor_account_id()
            );

            game.player1_reveal = Some(reveal);

            if Game::commit_reveal(
                &(game.player1_commit),
                &(game.player1_reveal.clone().unwrap()),
            ) {
                game.game_state = GameState::Revealed;
                game.reveal_time = Some(env::block_timestamp());
            } else {
                log!("Player1 is not honest");

                game.game_state = GameState::Ended;
                game.winner = Some(game.player2.clone().unwrap());

                Promise::new(game.player2.clone().unwrap()).transfer(game.deposit * 2);
            }

            self.games.insert(&game_id, &game);
        } else {
            panic!("Game is not active");
        }
    }

    pub fn second_reveal(&mut self, game_id: GameId, reveal: Vec<u8>) {
        assert!(
            reveal.len() == 16,
            "Invalid reveal format. Must be byte-vector with size 16"
        );

        assert!(
            self.games.get(&game_id).is_some(),
            "No game with such GameId"
        );

        let mut game = self.games.get(&game_id).unwrap();

        if let GameState::Revealed = game.game_state {
            assert_eq!(
                game.player2.clone().unwrap(),
                env::predecessor_account_id(),
                "Player2 is not {}",
                env::predecessor_account_id()
            );

            game.player2_reveal = Some(reveal);

            if Game::commit_reveal(
                &(game.player2_commit.clone().unwrap()),
                &(game.player2_reveal.clone().unwrap()),
            ) {
                let player1_reveal = game.player1_reveal.clone().unwrap();
                let player2_reveal = game.player1_reveal.clone().unwrap();

                let first_guess = u128::from_be_bytes(player1_reveal.try_into().unwrap());
                let second_guess = u128::from_be_bytes(player2_reveal.try_into().unwrap());

                if game.player1_guess == ((first_guess + second_guess) % 2 == 0) {
                    log!("Player1 is Winner");
                    game.winner = Some(game.player1.clone());

                    Promise::new(game.player1.clone()).transfer(game.deposit * 2);
                } else {
                    log!("Player2 is Winner");
                    game.winner = Some(game.player2.clone().unwrap());

                    Promise::new(game.player2.clone().unwrap()).transfer(game.deposit * 2);
                }
            } else {
                log!("Player2 is not honest");

                game.winner = Some(game.player1.clone());
                Promise::new(game.player1.clone()).transfer(game.deposit * 2);
            }

            game.game_state = GameState::Ended;
            self.games.insert(&game_id, &game);
        } else {
            panic!("Game is not active");
        }
    }

    pub fn get_prize(&mut self, game_id: GameId) {
        assert!(
            self.games.get(&game_id).is_some(),
            "No game with such GameId"
        );

        let mut game = self.games.get(&game_id).unwrap();

        if let GameState::Revealed = game.game_state {
            assert_eq!(game.player1.clone(), env::predecessor_account_id());

            let reveal_time = game.reveal_time.clone().unwrap();
            assert!(
                env::block_timestamp() - reveal_time > ONE_MINUTE,
                "Can't demand prize yet"
            );

            log!("Player2 is late. Player1 is winner");

            game.winner = Some(env::predecessor_account_id());
            game.game_state = GameState::Ended;

            Promise::new(env::predecessor_account_id()).transfer(2 * game.deposit);

            self.games.insert(&game_id, &game);
        } else {
            panic!("Game is not active");
        }
    }

    pub fn get_game_state(&self, game_id: GameId) -> Game {
        self.games.get(&game_id).expect("No game with such GameId")
    }

    #[private]
    pub fn state_cleaner(&mut self) {
        let ended_games: Vec<_> = self
            .games
            .iter()
            .filter(|(_, v)| {
                if let GameState::Ended = v.game_state {
                    true
                } else {
                    false
                }
            })
            .map(|(k, _)| k)
            .collect();

        for i in ended_games {
            self.games.remove(&i);
        }
    }
}
