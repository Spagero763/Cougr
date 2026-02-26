## feat: implement Guild Arena with social recovery and multi-device (#48)

### Summary

PvP arena game example demonstrating guild-based social recovery and multi-device play using Cougr-Core account abstractions. Players register with guild members as guardians, play from multiple devices with policy-controlled permissions, and recover accounts through guardian quorum + timelock if keys are lost.

### What's New

- **`examples/guild_arena/`** — Full Soroban smart contract with ECS-patterned combat, social recovery, and multi-device support
- **`.github/workflows/guild_arena.yml`** — CI pipeline (fmt, clippy, build, test, stellar contract build)

### Key Features

| Feature               | Implementation                                                                        |
| --------------------- | ------------------------------------------------------------------------------------- |
| Social Recovery       | `RecoverableAccount` with configurable guardian threshold (2-of-3) and 7-day timelock |
| Multi-Device          | `DeviceManager` with per-device policies (Full, PlayOnly)                             |
| PvP Combat            | Turn-based arena with Attack/Defend/Special actions                                   |
| Elo Rating            | Simplified rating updates after each match                                            |
| Progress Preservation | Stats, rating, and history survive key recovery                                       |

### Contract API

```
register_player(player, guardians, threshold, timelock)
add_device(player, device_key, policy)
remove_device(player, device_key)
start_match(device_key)
submit_action(device_key, action) → RoundResult
initiate_recovery(guardian, player, new_key)
approve_recovery(guardian, player, new_key)
finalize_recovery(player)
get_player(player) → PlayerProfile
get_match() → ArenaState
```

### Test Coverage (10 tests)

- ✅ Player registration with guardians
- ✅ Multi-device add/remove
- ✅ Multi-device play (desktop + mobile)
- ✅ Device policy enforcement (PlayOnly blocked from admin)
- ✅ Full combat match resolution
- ✅ Rating updates after match
- ✅ Special vs Defend damage comparison
- ✅ Complete recovery lifecycle (initiate → approve → timelock → finalize)
- ✅ Recovery with insufficient approvals (rejected)
- ✅ Game state preservation through recovery

### Build Proof

<!-- Attach screenshot of successful build output here -->
<!-- Example: cargo build, cargo test, cargo clippy results -->

![Build Proof](<!-- paste image URL or drag image here -->)

### How to Get Build Proof

Run these commands and screenshot the output:

```bash
cd examples/guild_arena
cargo build 2>&1
cargo test 2>&1
cargo clippy --all-targets --all-features -- -D warnings 2>&1
cargo build --target wasm32-unknown-unknown --release 2>&1
```

### References

- Closes #48
- Uses `cougr_core::accounts::recovery::RecoverableAccount` ([src/accounts/recovery.rs](../src/accounts/recovery.rs))
- Uses `cougr_core::accounts::multi_device::DeviceManager` ([src/accounts/multi_device.rs](../src/accounts/multi_device.rs))
- Follows existing example patterns (see `examples/pong/`)
