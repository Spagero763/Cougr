// Components for Proof-of-Hunt
use soroban_sdk::{Env, Address, BytesN};

pub struct MapCommitmentComponent;
impl MapCommitmentComponent {
    pub fn init(env: &Env, map_commitment: &BytesN<32>, width: u32, height: u32) {
        env.storage().set(b"map_commitment", map_commitment);
        env.storage().set(b"map_width", &width);
        env.storage().set(b"map_height", &height);
    }
}

pub struct PlayerStateComponent;
impl PlayerStateComponent {
    pub fn init(env: &Env, player: &Address) {
        env.storage().set((b"player_state", player), &(0u32, 0u32, 0u32)); // position, score, health
    }
}

pub struct ExplorationComponent;
impl ExplorationComponent {
    pub fn init(env: &Env, player: &Address) {
        env.storage().set((b"explored", player), &Vec::<(u32, u32)>::new());
    }
}

pub struct HintUsageComponent;
impl HintUsageComponent {
    pub fn init(env: &Env, player: &Address) {
        env.storage().set((b"hints_used", player), &0u32);
        env.storage().set((b"scans_used", player), &0u32);
    }
}

pub struct GameStatusComponent;
impl GameStatusComponent {
    pub fn init(env: &Env, player: &Address) {
        env.storage().set((b"game_status", player), &0u32); // 0: active, 1: won, 2: lost
    }
    pub fn get_state(env: &Env) -> super::GameState {
        // ...return dummy state for now...
        super::GameState {
            status: 0,
            score: 0,
            health: 0,
            discoveries: Vec::new(),
        }
    }
}

// Dummy types for now
pub struct ProofInput;
pub struct GameState {
    pub status: u32,
    pub score: u32,
    pub health: u32,
    pub discoveries: Vec<(u32, u32)>,
}
