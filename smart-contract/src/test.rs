#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

struct GameTest<'a> {
    env: Env,
    client: MindtraceContractClient<'a>,
    challenger: Address,
    opponent: Address,
    token: Address,
}

impl GameTest<'_> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(MindtraceContract, ());
        let client = MindtraceContractClient::new(&env, &contract_id);
        let challenger = Address::generate(&env);
        let opponent = Address::generate(&env);
        let token = Address::generate(&env);
        GameTest {
            env,
            client,
            challenger,
            opponent,
            token,
        }
    }

    /// Drive a game to the WAGER_MATCHED state and return its id.
    fn propose_and_match(&self, challenger_answer: u32, opponent_answer: u32) -> String {
        let game_id =
            self.client
                .propose_game(&self.challenger, &self.opponent, &100, &challenger_answer);
        self.client
            .submit_wager(&game_id, &self.opponent, &100, &opponent_answer);
        game_id
    }
}

#[test]
fn propose_game_stores_game_in_proposed_state() {
    let t = GameTest::setup();

    let game_id = t.client.propose_game(&t.challenger, &t.opponent, &100, &1);

    let game = t.client.get_game(&game_id);
    assert_eq!(game.state, GAME_STATE_PROPOSED);
    assert_eq!(game.challenger, t.challenger);
    assert_eq!(game.opponent, t.opponent);
    assert_eq!(game.challenger_wager, 100);
    assert_eq!(game.challenger_answer, 1);
    assert_eq!(game.opponent_wager, 0);
    assert_eq!(game.total_pot, 100);
}

#[test]
fn submit_wager_matches_pot_and_advances_state() {
    let t = GameTest::setup();

    let game_id = t.propose_and_match(1, 0);

    let game = t.client.get_game(&game_id);
    assert_eq!(game.state, GAME_STATE_WAGER_MATCHED);
    assert_eq!(game.opponent_wager, 100);
    assert_eq!(game.opponent_answer, 0);
    assert_eq!(game.total_pot, 200);
}

#[test]
fn accept_game_advances_to_accepted() {
    let t = GameTest::setup();

    let game_id = t.propose_and_match(1, 0);
    t.client.accept_game(&game_id);

    let game = t.client.get_game(&game_id);
    assert_eq!(game.state, GAME_STATE_ACCEPTED);
}

#[test]
fn opponent_wins_reveal_when_guess_matches() {
    let t = GameTest::setup();

    let game_id = t.propose_and_match(1, 1);
    t.client.accept_game(&game_id);
    let winner = t.client.reveal_answer(&game_id);

    assert_eq!(winner, t.opponent);
    assert_eq!(t.client.get_game(&game_id).state, GAME_STATE_REVEALED);
}

#[test]
fn challenger_wins_reveal_when_guess_differs() {
    let t = GameTest::setup();

    let game_id = t.propose_and_match(1, 0);
    t.client.accept_game(&game_id);
    let winner = t.client.reveal_answer(&game_id);

    assert_eq!(winner, t.challenger);
}

#[test]
fn propose_game_rejects_non_binary_answer() {
    let t = GameTest::setup();
    assert!(t.client.try_propose_game(&t.challenger, &t.opponent, &100, &2).is_err());
}

#[test]
fn propose_game_rejects_zero_wager() {
    let t = GameTest::setup();
    assert!(t.client.try_propose_game(&t.challenger, &t.opponent, &0, &1).is_err());
}

#[test]
fn submit_wager_rejects_mismatched_amount() {
    let t = GameTest::setup();
    let game_id = t.client.propose_game(&t.challenger, &t.opponent, &100, &1);
    assert!(t.client.try_submit_wager(&game_id, &t.opponent, &50, &0).is_err());
}

#[test]
fn submit_wager_rejects_wrong_opponent() {
    let t = GameTest::setup();
    let game_id = t.client.propose_game(&t.challenger, &t.opponent, &100, &1);
    let intruder = Address::generate(&t.env);
    assert!(t.client.try_submit_wager(&game_id, &intruder, &100, &0).is_err());
}

#[test]
fn accept_game_requires_matched_wager() {
    let t = GameTest::setup();
    let game_id = t.client.propose_game(&t.challenger, &t.opponent, &100, &1);
    assert!(t.client.try_accept_game(&game_id).is_err());
}

#[test]
fn reveal_answer_requires_accepted_state() {
    let t = GameTest::setup();
    let game_id = t.propose_and_match(1, 0);
    assert!(t.client.try_reveal_answer(&game_id).is_err());
}

#[test]
fn finish_game_requires_revealed_state() {
    let t = GameTest::setup();
    let game_id = t.propose_and_match(1, 0);
    t.client.accept_game(&game_id);
    assert!(t.client.try_finish_game(&game_id, &t.token).is_err());
}

#[test]
fn get_game_panics_for_unknown_id() {
    let t = GameTest::setup();
    assert!(t.client.try_get_game(&String::from_str(&t.env, "no-such-game")).is_err());
}

#[test]
fn finish_game_completes_revealed_game() {
    let t = GameTest::setup();

    let game_id = t.propose_and_match(0, 1);
    t.client.accept_game(&game_id);
    t.client.reveal_answer(&game_id);
    t.client.finish_game(&game_id, &t.token);

    assert_eq!(t.client.get_game(&game_id).state, GAME_STATE_FINISHED);
}
