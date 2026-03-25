# Proof-of-Hunt: Soroban Contract Example

This example demonstrates a treasure-discovery game on Stellar using:
- **stellar-zk** for zero-knowledge proof-backed exploration
- **x402** for premium paid actions (hints, scans)
- Soroban contract patterns

## Features
- Hidden map state committed off-chain
- On-chain proof validation for discoveries
- Premium actions paid via x402
- Deterministic, testable progression

## Architecture
- **Components:** MapCommitment, PlayerState, Exploration, HintUsage, GameStatus
- **Systems:** Exploration, ProofValidation, DiscoveryResolution, HintPurchase, EndCondition

## Contract API
- `init_game(env, player, map_commitment, width, height)`
- `explore(env, player, x, y, proof)`
- `purchase_hint(env, player, hint_type)`
- `get_state(env)`
- `is_finished(env)`

## Stellar-ZK
- Off-chain: Map layout, treasure positions
- On-chain: Proofs of valid exploration/discovery
- See [stellar-zk](https://github.com/salazarsebas/stellar-zk)

## x402
- Used for premium actions (hints, scans)
- See [x402 docs](https://developers.stellar.org/docs/build/apps/x402)

## Build & Test
```sh
cd examples/proof_of_hunt
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
stellar contract build
```

## CI
See `.github/workflows/proof_of_hunt.yml` for workflow.

## Privacy & Anti-Cheat
- Map and treasures are hidden off-chain
- Only valid discoveries are accepted via ZK proofs
- Premium actions require payment via x402

---

For details, see the code and comments in each module.
