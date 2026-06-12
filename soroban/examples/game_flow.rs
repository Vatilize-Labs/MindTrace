/// Example: Complete MindTrace game flow on Stellar/Soroban
///
/// This demonstrates how to call the contract functions in sequence
/// to play a complete game from proposal to finish.

fn main() {
    println!("=== MindTrace Game Flow on Stellar ===\n");

    // Example addresses (would be real Stellar addresses in practice)
    let challenger_addr = "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let opponent_addr = "GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY";
    let token_addr = "GZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ";

    // Step 1: Challenger proposes game
    println!("Step 1: Challenger proposes game");
    println!("  challenger: {}", challenger_addr);
    println!("  opponent: {}", opponent_addr);
    println!("  wager: 1000");
    println!("  challenger_answer: 0 (in weeds)");
    println!("  → Contract creates Game with state = PROPOSED");
    println!("  → Event emitted: GameProposed\n");

    let game_id = opponent_addr;  // Game ID is opponent address

    // Step 2: Opponent matches wager
    println!("Step 2: Opponent matches wager");
    println!("  game_id: {}", game_id);
    println!("  opponent_wager: 1000");
    println!("  opponent_answer: 0 (also in weeds)");
    println!("  → Contract updates Game state = WAGER_MATCHED");
    println!("  → total_pot = 2000");
    println!("  → Event emitted: GameAccepted\n");

    // Step 3: Accept game (can be called by either player)
    println!("Step 3: Accept game");
    println!("  game_id: {}", game_id);
    println!("  → Contract updates Game state = ACCEPTED");
    println!("  → Timeout timer starts (10,000 blocks ≈ 83 minutes)");
    println!("  → Now waiting for challenger to reveal\n");

    // Step 4: Challenger reveals answer
    println!("Step 4: Challenger reveals answer");
    println!("  game_id: {}", game_id);
    println!("  → Contract compares answers:");
    println!("     challenger_answer = 0");
    println!("     opponent_answer = 0");
    println!("     → MATCH! Opponent wins!");
    println!("  → Game state = REVEALED");
    println!("  → Event emitted: GameRevealed\n");

    // Step 5: Finish game and distribute pot
    println!("Step 5: Finish game and distribute pot");
    println!("  game_id: {}", game_id);
    println!("  token: {}", token_addr);
    println!("  → Contract transfers 2000 tokens to opponent");
    println!("  → Game state = FINISHED");
    println!("  → Event emitted: GameFinished\n");

    println!("=== Game Complete ===");
    println!("Winner: opponent");
    println!("Pot Received: 2000 tokens");
}

/// Alternative scenario: Timeout
pub fn timeout_scenario() {
    println!("\n=== Timeout Scenario ===\n");

    println!("If challenger doesn't reveal within timeout:");
    println!("  → After 10,000 blocks (~83 min)");
    println!("  → Any player can call finish_game_by_timeout");
    println!("  → Opponent automatically wins");
    println!("  → Pot transferred to opponent\n");
}

/// Alternative scenario: Renegation
pub fn renegation_scenario() {
    println!("\n=== Renegation Scenarios ===\n");

    println!("Before opponent matches (PROPOSED state):");
    println!("  → Challenger can call challenger_renege_stake");
    println!("  → Gets refund of 1000 tokens\n");

    println!("After opponent matches but before accept (WAGER_MATCHED state):");
    println!("  → Opponent can call opponent_renege_stake");
    println!("  → Both get refunds (1000 each)\n");
}

/// Example of answer meanings
pub fn answer_key() {
    println!("\n=== Answer Key ===\n");
    println!("0 = 'in the weeds'");
    println!("1 = 'behind the building'");
    println!("\nWinner logic:");
    println!("  If opponent_answer == challenger_answer");
    println!("    → Opponent wins (correct deduction)");
    println!("  Else");
    println!("    → Challenger wins (opponent failed to deduce)\n");
}
