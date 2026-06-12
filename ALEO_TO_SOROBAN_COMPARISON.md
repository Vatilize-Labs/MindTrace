# MindTrace: Aleo vs Soroban Code Comparison

## 1. Proposing a Game

### Aleo (Leo)
```leo
transition propose_game (
    wager_record: my_puzzle_pieces.leo/Piece.record,
    challenger_wager_amount: u64,
    sender: address,
    challenger: address,
    opponent: address,
    game_multisig: address,
    challenger_message_1: field,
    challenger_message_2: field,
    challenger_message_3: field,
    challenger_message_4: field,
    challenger_message_5: field,
    challenger_sig: signature,
    challenger_nonce: field,
    challenger_answer: field,
    game_multisig_seed: field,
) -> (PieceClaim, PieceStake, ...) {
    assert_eq(self.caller, self.signer);
    assert(challenger_answer == 0field || challenger_answer == 1field);
    
    // Hash answer with nonce (ZK commitment)
    let challenger_commit: field = BHP256::commit_to_field(
        challenger_answer, 
        challenger_nonce as scalar
    );
    
    // Consume wager record and split it
    let (piece_claim, piece_stake, ...) = 
        my_puzzle_pieces.leo/stake_transfer_in(...);
    
    // Create game record
    let game_record: Game = Game {
        owner: game_multisig,
        challenger_commit: challenger_commit,
        opponent_answer: 123field,
        total_pot: challenger_wager_amount + challenger_wager_amount,
        challenger_address: challenger,
        opponent_address: opponent,
        game_multisig: game_multisig,
        game_state: 1field,
        ix: 1u32,
    };
    
    return (piece_claim, piece_stake, ..., game_record, ...);
}
```

### Stellar (Soroban/Rust)
```rust
pub fn propose_game(
    env: Env,
    challenger: Address,
    opponent: Address,
    challenger_wager: i128,
    challenger_answer: u32,
) -> String {
    challenger.require_auth();
    
    // Validate answer
    assert!(challenger_answer == 0 || challenger_answer == 1, 
            "Answer must be 0 or 1");
    assert!(challenger_wager > 0, "Wager must be positive");
    
    // Generate unique game ID
    let game_id = generate_game_id(&env, &challenger, &opponent);
    
    // Create game struct (stored in ledger)
    let game = Game {
        id: game_id.clone(),
        challenger: challenger.clone(),
        opponent: opponent.clone(),
        challenger_wager,
        opponent_wager: 0,
        challenger_answer,          // Stored plainly
        opponent_answer: 0,
        total_pot: challenger_wager,
        state: GAME_STATE_PROPOSED,
        created_at: env.ledger().timestamp(),
        accepted_at: 0,
    };
    
    // Store in contract storage
    env.storage().instance().set(&game_id, &game);
    
    // Emit event
    env.events().publish(
        (symbol_short!("GameProposed"),),
        GameProposed {
            game_id: game_id.clone(),
            challenger: challenger.clone(),
            opponent: opponent.clone(),
            wager: challenger_wager,
        }
    );
    
    game_id
}
```

### Key Differences
| Aspect | Aleo | Soroban |
|--------|------|---------|
| **Input** | Record consumption | Direct values |
| **Privacy** | ZK commitment (hash) | Plaintext storage |
| **State** | Returned as output record | Stored in ledger |
| **Verification** | Signature validation | `require_auth()` |
| **Events** | Notification records | Contract events |
| **Return** | Multiple records | Single ID |

---

## 2. Accepting a Wager

### Aleo (Leo)
```leo
transition submit_wager (
    opponent_wager_record: my_puzzle_pieces.leo/Piece.record,
    key_record: multiparty_pvp_utils_v015.leo/Key.record,
    game_req_notification: GameReqNotification,
    opponent_message_1: field,
    opponent_message_2: field,
    opponent_message_3: field,
    opponent_message_4: field,
    opponent_message_5: field,
    opponent_sig: signature,
) -> (...) {
    assert_eq(self.caller, self.signer);
    assert(opponent_wager_record.amount > 0u64);
    assert(opponent_wager_record.amount >= 
           game_req_notification.total_pot / 2u64);
    
    let (piece_claim_opponent, piece_stake_opponent, ...) = 
        my_puzzle_pieces.leo/stake_transfer_in(
            opponent_wager_record,
            game_req_notification.opponent_address,
            game_req_notification.challenger_address,
            game_req_notification.opponent_address,
            game_req_notification.game_multisig,
            game_req_notification.total_pot / 2u64,
            ...
        );
    
    return (...);
}
```

### Stellar (Soroban/Rust)
```rust
pub fn submit_wager(
    env: Env,
    game_id: String,
    opponent: Address,
    opponent_wager: i128,
    opponent_answer: u32,
) {
    opponent.require_auth();
    
    // Validate answer
    assert!(opponent_answer == 0 || opponent_answer == 1, 
            "Answer must be 0 or 1");
    
    // Retrieve game from storage
    let mut game: Game = env
        .storage()
        .instance()
        .get(&game_id)
        .expect("Game not found");
    
    // Validate state and player
    assert_eq!(game.opponent, opponent, "Not authorized for this game");
    assert_eq!(game.state, GAME_STATE_PROPOSED, "Invalid game state");
    assert_eq!(opponent_wager, game.challenger_wager, 
               "Wager must match challenger");
    
    // Update game
    game.opponent_wager = opponent_wager;
    game.opponent_answer = opponent_answer;
    game.total_pot = game.challenger_wager + game.opponent_wager;
    game.state = GAME_STATE_WAGER_MATCHED;
    
    // Save updated game
    env.storage().instance().set(&game_id, &game);
    
    // Emit event
    env.events().publish(
        (symbol_short!("GameAccepted"),),
        GameAccepted {
            game_id: game_id.clone(),
            total_pot: game.total_pot,
        }
    );
}
```

### Key Differences
| Aspect | Aleo | Soroban |
|--------|------|---------|
| **Record Handling** | Consume & split records | Read & update in storage |
| **State Lookup** | Via notification records | Direct storage lookup |
| **Multi-sig** | Complex multisig logic | Simpler state machine |
| **Answer** | Not stored yet | Stored immediately |
| **Total Pot** | From notification | Calculated from game |

---

## 3. Revealing and Determining Winner

### Aleo (Leo)
```leo
transition reveal_answer_game(
    reveal_answer_notification_record: RevealAnswerNotification,
    challenger_answer_record: multiparty_pvp_utils_v015.leo/Answer.record,
    joint_piece_state: my_puzzle_pieces.leo/JointPieceState.record,
    challenger_claim_signature: my_puzzle_pieces.leo/ClaimSignature.record,
) -> (...) {
    assert_eq(joint_piece_state.state_updater_address, self.signer);
    assert_eq(self.caller, self.signer);
    
    // Determine winner by comparing revealed answer
    let winner_address: address = 
        reveal_answer_notification_record.opponent_answer == 
        challenger_answer_record.answer ? 
            challenger_answer_record.opponent_address : 
            challenger_answer_record.challenger_address;
    
    let loser_address: address = 
        winner_address == challenger_answer_record.challenger_address ? 
            challenger_answer_record.opponent_address : 
            challenger_answer_record.challenger_address;
    
    // Reveal answer record
    let (revealed_answer_record, ...) = 
        multiparty_pvp_utils_v015.leo/reveal_answer(...);
    
    // Update joint piece state
    let (joint_piece_winner, ...) = 
        my_puzzle_pieces.leo/joint_stake_state_update(...);
    
    return (...);
}
```

### Stellar (Soroban/Rust)
```rust
pub fn reveal_answer(
    env: Env,
    game_id: String,
) -> Address {
    // Retrieve game
    let mut game: Game = env
        .storage()
        .instance()
        .get(&game_id)
        .expect("Game not found");
    
    // Validate state
    assert_eq!(game.state, GAME_STATE_ACCEPTED, 
               "Invalid game state");
    
    // Determine winner (simple comparison)
    let winner = if game.opponent_answer == game.challenger_answer {
        game.opponent.clone()
    } else {
        game.challenger.clone()
    };
    
    // Update game state
    game.state = GAME_STATE_REVEALED;
    env.storage().instance().set(&game_id, &game);
    
    // Emit event
    env.events().publish(
        (symbol_short!("GameRevealed"),),
        GameRevealed {
            game_id: game_id.clone(),
            challenger_answer: game.challenger_answer,
            opponent_answer: game.opponent_answer,
            winner: winner.clone(),
        }
    );
    
    winner
}
```

### Key Differences
| Aspect | Aleo | Soroban |
|--------|------|---------|
| **Answer Reveal** | Cryptographic record reveal | Already stored plainly |
| **Winner Logic** | Ternary operator | If/else statement |
| **State Update** | Multiple record outputs | Single record update |
| **Complexity** | High (multisig, joint pieces) | Simple (direct comparison) |
| **Proof Generation** | Implicit ZK proof | N/A (no privacy) |

---

## 4. Finishing and Distributing Pot

### Aleo (Leo)
```leo
transition finish_game(
    game_record: Game,
    joint_piece_winner: my_puzzle_pieces.leo/JointPieceWinner.record,
    piece_joint_stake: my_puzzle_pieces.leo/PieceJointStake.record,
    joint_piece_time_claim: my_puzzle_pieces.leo/JointPieceTimeClaim.record,
) -> (Piece, AuditTransferToWinner, AuditPieceOwner, Game, ...) {
    assert_eq(game_record.owner, self.signer);
    assert_eq(self.caller, self.signer);
    
    let loser_address: address = 
        joint_piece_winner.winner == game_record.challenger_address ? 
            game_record.opponent_address : 
            game_record.challenger_address;
    
    // Transfer from joint stake to winner
    let (piece_record, audit_transfer_to_winner_record, ...) = 
        my_puzzle_pieces.leo/joint_stake_transfer_to_winner(
            joint_piece_winner,
            piece_joint_stake,
            joint_piece_time_claim,
        );
    
    return (piece_record, audit_transfer_to_winner_record, ...);
}
```

### Stellar (Soroban/Rust)
```rust
pub fn finish_game(
    env: Env,
    game_id: String,
    token: Address,
) {
    // Retrieve game
    let mut game: Game = env
        .storage()
        .instance()
        .get(&game_id)
        .expect("Game not found");
    
    // Validate state
    assert_eq!(game.state, GAME_STATE_REVEALED, "Invalid game state");
    
    // Determine winner
    let winner = if game.opponent_answer == game.challenger_answer {
        game.opponent.clone()
    } else {
        game.challenger.clone()
    };
    
    // Transfer pot to winner
    transfer_token(&env, token.clone(), game.id.clone(), 
                   winner.clone(), game.total_pot);
    
    // Update game state
    game.state = GAME_STATE_FINISHED;
    env.storage().instance().set(&game_id, &game);
    
    // Emit event
    env.events().publish(
        (symbol_short!("GameFinished"),),
        GameFinished {
            game_id: game_id.clone(),
            winner,
            pot: game.total_pot,
        }
    );
}
```

### Key Differences
| Aspect | Aleo | Soroban |
|--------|------|---------|
| **Pot Storage** | Joint piece stake record | Ledger balance |
| **Transfer** | Record consumption | Token contract call |
| **Winner Lookup** | From joint piece record | From game state |
| **Audit Trail** | Audit records | Contract events |
| **Return Values** | Multiple records | None (state changed) |

---

## Summary Table

| Function | Aleo Inputs | Aleo Outputs | Soroban Inputs | Soroban Outputs |
|----------|-----------|----------|---------------|-----------------|
| **propose_game** | Records + signatures | Multiple records | Basic params | Game ID |
| **submit_wager** | Records + signatures | Multiple records | Game ID + params | None (state change) |
| **accept_game** | Records + params | Multiple records | Game ID | None (state change) |
| **reveal_answer** | Records + proof | Multiple records | Game ID | Winner address |
| **finish_game** | Records | Multiple records | Game ID + token | None (state change) |
| **renege_stake** | Notification records | Records + refund | Game ID + token | None (state change) |

---

## Lines of Code Comparison

| Component | Aleo (Leo) | Soroban (Rust) |
|-----------|-----------|----------------|
| **Main contract** | 837 lines | ~200 lines |
| **Utility modules** | ~300 lines | ~50 lines |
| **Total** | ~1,137 lines | ~250 lines |

**Note**: Soroban code is much simpler because:
- No ZK proof generation
- No record consumption/creation
- No multisig complexity
- Simpler state management

---

## Lessons Learned

### What Made Aleo Complex
1. Zero-knowledge proof generation
2. Record-based state (outputs needed for next tx)
3. Multi-phase verification (propose → match → accept → reveal)
4. Multisig authorization
5. Audit records for compliance

### What Makes Soroban Simpler
1. Transparent on-chain state
2. Direct storage (no record outputs)
3. Simpler state machine
4. Address-based auth
5. Event logs for history

### Trade-offs Accepted
| Aleo Advantage | Soroban Cost |
|---------------|------------|
| Answers hidden until reveal | Answers public immediately |
| True privacy for cheating prevention | Rely on game logic (weaker) |
| Cryptographically verifiable | Transparent verification |
| Complex but trustless | Simpler but requires trust in contract |
