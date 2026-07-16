use soroban_sdk::{Address, String, contracttype};

#[derive(Clone)]
#[contracttype]
pub struct GameProposed {
    pub game_id: String,
    pub challenger: Address,
    pub opponent: Address,
    pub wager: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct GameAccepted {
    pub game_id: String,
    pub total_pot: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct GameRevealed {
    pub game_id: String,
    pub challenger_answer: u32,
    pub opponent_answer: u32,
    pub winner: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct GameFinished {
    pub game_id: String,
    pub winner: Address,
    pub pot: i128,
}
