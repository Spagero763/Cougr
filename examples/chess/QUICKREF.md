# Quick Reference: Chess Contract

## Build Commands

```bash
# Development build
cargo build

# Run tests (17 tests)
cargo test

# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# With Stellar CLI (if installed)
stellar contract build
```

## Test Results

```
✅ test_init_game
✅ test_board_initialization
✅ test_state_hash_consistency
✅ test_move_without_init (should panic)
✅ test_wrong_turn
✅ test_resign
✅ test_move_after_resignation
✅ test_component_trait_board
✅ test_component_trait_turn
✅ test_king_capture_checkmate
✅ test_piece_movement_pawn
✅ test_piece_movement_knight
✅ test_piece_movement_rook
✅ test_turn_switching
✅ test_move_count_increment
✅ test_all_piece_types_present
✅ test_proof_record_update

Total: 17 passed, 0 failed
```

## Key Files

| File | Lines | Purpose |
|------|-------|---------|
| `src/lib.rs` | 473 | Main contract with ZK verification |
| `src/test.rs` | 364 | Comprehensive test suite |
| `README.md` | 366 | Architecture documentation |
| `Cargo.toml` | 30 | Dependencies and build config |
| `.github/workflows/chess.yml` | 99 | CI/CD pipeline |
| `IMPLEMENTATION.md` | 205 | Implementation summary |

## Core Concepts

### ZK Proof Flow

```
Player (Off-Chain)              Contract (On-Chain)
──────────────────              ───────────────────
1. Generate move
2. Build circuit with:
   - state_hash
   - from position
   - to position
3. Prove move legality    →    4. Verify proof
                                5. Check turn
                                6. Apply move
                                7. Update state_hash
                                8. Switch turn
```

### State Hash

Computed as SHA-256 of:
```
for each occupied square:
  position (4 bytes) + piece_kind (1 byte) + color (1 byte)
```

Binds proofs to specific board states, preventing replay attacks.

### Circuit Public Inputs

```rust
[state_hash, from, to]
```

Private inputs (in circuit, not on-chain):
- Full board state
- Piece movement rules
- Path obstruction checks

## Cougr-Core Usage

### GameCircuit Trait
```rust
use cougr_core::zk::circuits::CustomCircuit;

let circuit = CustomCircuit::builder(vk)
    .add_bytes32(&state_hash)
    .add_u32(&env, from)
    .add_u32(&env, to)
    .build();

let valid = circuit.verify(&env, &proof)?;
```

### ComponentTrait
```rust
use cougr_core::component::ComponentTrait;

impl ComponentTrait for BoardState {
    fn component_type() -> Symbol { symbol_short!("board") }
    fn serialize(&self, env: &Env) -> Bytes { /* ... */ }
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> { /* ... */ }
}
```

## API Quick Reference

```rust
// Initialize game
new_game(env: Env, white: Address, black: Address)

// Submit move with proof
submit_move(
    env: Env,
    player: Address,
    from: u32,      // 0-63
    to: u32,        // 0-63
    proof: Bytes
) -> MoveResult

// Resign
resign(env: Env, player: Address)

// Query state
get_board(env: Env) -> BoardState
get_state(env: Env) -> GameState

// Admin
set_vk(env: Env, vk: VerificationKey)
```

## Board Layout

```
Position mapping (0-63):
56 57 58 59 60 61 62 63  ← Black pieces
48 49 50 51 52 53 54 55  ← Black pawns
40 41 42 43 44 45 46 47
32 33 34 35 36 37 38 39
24 25 26 27 28 29 30 31
16 17 18 19 20 21 22 23
 8  9 10 11 12 13 14 15  ← White pawns
 0  1  2  3  4  5  6  7  ← White pieces
```

## Move Results

```rust
pub enum MoveResult {
    Success,        // Move applied
    InvalidProof,   // ZK proof failed
    WrongTurn,      // Not player's turn
    GameOver,       // Game already ended
}
```

## Game Status

```rust
pub enum GameStatus {
    Playing,        // Game in progress
    Checkmate,      // King captured
    Draw,           // Not implemented
    Resigned,       // Player resigned
}
```

## CI/CD Pipeline

Runs on push/PR to `examples/chess/**`:
1. Format check (`cargo fmt`)
2. Lint check (`cargo clippy`)
3. Unit tests (`cargo test`)
4. WASM build
5. Size check
6. Artifact upload

## Next Steps

1. **Off-chain prover**: Implement circuit for move validation
2. **Proof serialization**: Define encoding for Groth16 proofs
3. **VK generation**: Create verification key from circuit
4. **Integration test**: End-to-end test with real proofs
5. **Deployment**: Deploy to Stellar testnet
6. **Frontend**: Build UI for move submission

## Resources

- [Cougr Repository](https://github.com/salazarsebas/Cougr)
- [Issue #45](https://github.com/salazarsebas/Cougr/issues/45)
- [Soroban Docs](https://developers.stellar.org/docs/build/smart-contracts)
- [Groth16 Paper](https://eprint.iacr.org/2016/260.pdf)
