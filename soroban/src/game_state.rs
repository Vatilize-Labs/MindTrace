use soroban_sdk::{Address, String, contracttype};

#[derive(Clone)]
#[contracttype]
pub struct Game {
    /// Unique game identifier
    pub id: String,
    /// The player who initiated the game
    pub challenger: Address,
    /// The player being challenged
    pub opponent: Address,
    /// Amount wagered by challenger
    pub challenger_wager: i128,
    /// Amount wagered by opponent
    pub opponent_wager: i128,
    /// Challenger's answer (0 or 1)
    pub challenger_answer: u32,
    /// Opponent's answer (0 or 1)
    pub opponent_answer: u32,
    /// Total pot (challenger_wager + opponent_wager)
    pub total_pot: i128,
    /// Current game state
    pub state: i128,
    /// Timestamp when game was created
    pub created_at: u64,
    /// Timestamp when game was accepted (for timeout tracking)
    pub accepted_at: u64,
}

pub enum GameState {
    Proposed = 1,
    WagerMatched = 2,
    Accepted = 3,
    Revealed = 4,
    Finished = 5,
    Timeout = 6,
    Reneged = 0,
}

pub enum Player {
    Challenger,
    Opponent,
}
