// Systems for Proof-of-Hunt
use soroban_sdk::{Env, Address};
use crate::components::*;

pub struct ExplorationSystem;
impl ExplorationSystem {
    pub fn explore(env: &Env, player: &Address, x: u32, y: u32, _proof: ProofInput) {
        // TODO: Use stellar-zk to verify proof
        // For now, just add to explored
        let mut explored = env.storage().get::<_, Vec<(u32, u32)>>((b"explored", player)).unwrap_or_default();
        explored.push((x, y));
        env.storage().set((b"explored", player), &explored);
    }
}

pub struct HintPurchaseSystem;
impl HintPurchaseSystem {
    pub fn purchase_hint(env: &Env, player: &Address, hint_type: u32) {
        // TODO: Integrate x402 for paid actions
        let key = if hint_type == 0 { b"hints_used" } else { b"scans_used" };
        let used = env.storage().get::<_, u32>((key, player)).unwrap_or(0);
        env.storage().set((key, player), &(used + 1));
    }
}

pub struct EndConditionSystem;
impl EndConditionSystem {
    pub fn is_finished(env: &Env) -> bool {
        // TODO: Implement real end condition
        false
    }
}
