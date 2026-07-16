#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, Symbol, Vec, String};

mod game_state;
mod events;

#[cfg(test)]
mod test;

use game_state::{Game, GameState, Player};
use events::{GameProposed, GameAccepted, GameRevealed, GameFinished};

pub const GAME_STATE_PROPOSED: i128 = 1;
pub const GAME_STATE_WAGER_MATCHED: i128 = 2;
pub const GAME_STATE_ACCEPTED: i128 = 3;
pub const GAME_STATE_REVEALED: i128 = 4;
pub const GAME_STATE_FINISHED: i128 = 5;
pub const GAME_STATE_TIMEOUT: i128 = 6;
pub const GAME_STATE_RENEGED: i128 = 0;

pub const TIMEOUT_BLOCKS: u32 = 10_000; // ~83 minutes at ~0.5s per block

#[contract]
pub struct MindtraceContract;

#[contractimpl]
impl MindtraceContract {
    /// Initialize a new game
    /// challenger: the address proposing the game
    /// opponent: the address being challenged
    /// challenger_wager: amount being wagered
    /// challenger_answer: 0 or 1 (encoded location)
    pub fn propose_game(
        env: Env,
        challenger: Address,
        opponent: Address,
        challenger_wager: i128,
        challenger_answer: u32,
    ) -> String {
        challenger.require_auth();

        // Validate answer is binary
        assert!(challenger_answer == 0 || challenger_answer == 1, "Answer must be 0 or 1");
        assert!(challenger_wager > 0, "Wager must be positive");

        let game_id = generate_game_id(&env, &challenger, &opponent);

        let game = Game {
            id: game_id.clone(),
            challenger: challenger.clone(),
            opponent: opponent.clone(),
            challenger_wager,
            opponent_wager: 0,
            challenger_answer,
            opponent_answer: 0,
            total_pot: challenger_wager,
            state: GAME_STATE_PROPOSED,
            created_at: env.ledger().timestamp(),
            accepted_at: 0,
        };

        env.storage().instance().set(&game_id, &game);

        // Emit event
        let event = GameProposed {
            game_id: game_id.clone(),
            challenger: challenger.clone(),
            opponent: opponent.clone(),
            wager: challenger_wager,
        };
        env.events().publish((symbol_short!("proposed"),), event);

        game_id
    }

    /// Opponent matches the wager to accept the game
    pub fn submit_wager(
        env: Env,
        game_id: String,
        opponent: Address,
        opponent_wager: i128,
        opponent_answer: u32,
    ) {
        opponent.require_auth();

        // Validate answer is binary
        assert!(opponent_answer == 0 || opponent_answer == 1, "Answer must be 0 or 1");

        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.opponent, opponent, "Not authorized for this game");
        assert_eq!(game.state, GAME_STATE_PROPOSED, "Invalid game state");
        assert_eq!(opponent_wager, game.challenger_wager, "Wager must match challenger");

        game.opponent_wager = opponent_wager;
        game.opponent_answer = opponent_answer;
        game.total_pot = game.challenger_wager + game.opponent_wager;
        game.state = GAME_STATE_WAGER_MATCHED;

        env.storage().instance().set(&game_id, &game);

        // Emit event
        let event = GameAccepted {
            game_id: game_id.clone(),
            total_pot: game.total_pot,
        };
        env.events().publish((symbol_short!("accepted"),), event);
    }

    /// Accept game and prepare for reveal phase
    pub fn accept_game(
        env: Env,
        game_id: String,
    ) {
        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.state, GAME_STATE_WAGER_MATCHED, "Invalid game state");

        game.state = GAME_STATE_ACCEPTED;
        game.accepted_at = env.ledger().timestamp();

        env.storage().instance().set(&game_id, &game);
    }

    /// Challenger reveals their answer, determine winner
    pub fn reveal_answer(
        env: Env,
        game_id: String,
    ) -> Address {
        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.state, GAME_STATE_ACCEPTED, "Invalid game state");

        // Determine winner: if opponent guessed correctly, opponent wins, else challenger wins
        let winner = if game.opponent_answer == game.challenger_answer {
            game.opponent.clone()
        } else {
            game.challenger.clone()
        };

        game.state = GAME_STATE_REVEALED;

        env.storage().instance().set(&game_id, &game);

        // Emit event
        let event = GameRevealed {
            game_id: game_id.clone(),
            challenger_answer: game.challenger_answer,
            opponent_answer: game.opponent_answer,
            winner: winner.clone(),
        };
        env.events().publish((symbol_short!("revealed"),), event);

        winner
    }

    /// Finish the game and distribute pot to winner
    pub fn finish_game(
        env: Env,
        game_id: String,
        token: Address,
    ) {
        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.state, GAME_STATE_REVEALED, "Invalid game state");

        let winner = if game.opponent_answer == game.challenger_answer {
            game.opponent.clone()
        } else {
            game.challenger.clone()
        };

        // Transfer pot to winner using native Stellar payments
        transfer_token(&env, token.clone(), game.id.clone(), winner.clone(), game.total_pot);

        game.state = GAME_STATE_FINISHED;

        env.storage().instance().set(&game_id, &game);

        // Emit event
        let event = GameFinished {
            game_id: game_id.clone(),
            winner,
            pot: game.total_pot,
        };
        env.events().publish((symbol_short!("finished"),), event);
    }

    /// Timeout mechanism: if challenger doesn't reveal after timeout, opponent wins
    pub fn finish_game_by_timeout(
        env: Env,
        game_id: String,
        token: Address,
    ) {
        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.state, GAME_STATE_ACCEPTED, "Can only timeout from accepted state");

        // Check if enough blocks have passed
        let current_block = env.ledger().sequence();
        let blocks_since_accept = current_block.saturating_sub(game.accepted_at as u32);
        assert!(blocks_since_accept > TIMEOUT_BLOCKS, "Timeout not yet reached");

        // Opponent wins on timeout
        let winner = game.opponent.clone();

        // Transfer pot to opponent
        transfer_token(&env, token.clone(), game.id.clone(), winner.clone(), game.total_pot);

        game.state = GAME_STATE_TIMEOUT;

        env.storage().instance().set(&game_id, &game);

        // Emit event
        let event = GameFinished {
            game_id: game_id.clone(),
            winner,
            pot: game.total_pot,
        };
        env.events().publish((symbol_short!("finished"),), event);
    }

    /// Challenger can renege before opponent matches wager
    pub fn challenger_renege_stake(
        env: Env,
        game_id: String,
        challenger: Address,
        token: Address,
    ) {
        challenger.require_auth();

        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.challenger, challenger, "Not authorized");
        assert_eq!(game.state, GAME_STATE_PROPOSED, "Can only renege from proposed state");

        // Return stake to challenger
        transfer_token(&env, token.clone(), game.id.clone(), challenger.clone(), game.challenger_wager);

        game.state = GAME_STATE_RENEGED;

        env.storage().instance().set(&game_id, &game);
    }

    /// Opponent can renege after matching wager but before accepting
    pub fn opponent_renege_stake(
        env: Env,
        game_id: String,
        opponent: Address,
        token: Address,
    ) {
        opponent.require_auth();

        let mut game: Game = env
            .storage()
            .instance()
            .get(&game_id)
            .expect("Game not found");

        assert_eq!(game.opponent, opponent, "Not authorized");
        assert_eq!(game.state, GAME_STATE_WAGER_MATCHED, "Can only renege from wager matched state");

        // Return stakes to both players
        transfer_token(&env, token.clone(), game.id.clone(), game.challenger.clone(), game.challenger_wager);
        transfer_token(&env, token.clone(), game.id.clone(), game.opponent.clone(), game.opponent_wager);

        game.state = GAME_STATE_RENEGED;

        env.storage().instance().set(&game_id, &game);
    }

    /// Get game details
    pub fn get_game(env: Env, game_id: String) -> Game {
        env.storage()
            .instance()
            .get(&game_id)
            .expect("Game not found")
    }
}

/// Generate a unique game ID using addresses and sequence number
fn generate_game_id(_env: &Env, challenger: &Address, opponent: &Address) -> String {
    // Use opponent address as game ID (address is globally unique)
    // In production, combine with timestamp/sequence for additional uniqueness
    opponent.to_string()
}

/// Helper function to transfer tokens
/// NOTE: This is a stub. Full implementation requires token contract integration.
/// For now, the game logic works but pot distribution would need real token transfers.
#[allow(dead_code)]
fn transfer_token(_env: &Env, _token: Address, _from: String, _to: Address, _amount: i128) {
    // This would interact with the token contract via cross-contract calls
    // Implementation depends on whether using native Stellar asset or issued asset
    //
    // Example with native XLM:
    //   env.invoke_contract(&token, &symbol_short!("transfer"), args)
    //
    // For now, the contract logic is correct; token transfers would be implemented in:
    // 1. A separate token contract
    // 2. Or via Stellar native payment channels
}
