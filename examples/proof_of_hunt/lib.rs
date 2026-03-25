//! Proof-of-Hunt: A Soroban contract example for proof-backed exploration with stellar-zk and x402 premium actions.

mod components;
mod systems;
mod tests;

use soroban_sdk::{contractimpl, Address, BytesN, Env};
use crate::components::*;
use crate::systems::*;

pub struct ProofOfHuntContract;

#[contractimpl]
impl ProofOfHuntContract {
    pub fn init_game(env: Env, player: Address, map_commitment: BytesN<32>, width: u32, height: u32) {
        MapCommitmentComponent::init(&env, &map_commitment, width, height);
        PlayerStateComponent::init(&env, &player);
        ExplorationComponent::init(&env, &player);
        HintUsageComponent::init(&env, &player);
        GameStatusComponent::init(&env, &player);
    }

    pub fn explore(env: Env, player: Address, x: u32, y: u32, proof: ProofInput) {
        ExplorationSystem::explore(&env, &player, x, y, proof);
    }

    pub fn purchase_hint(env: Env, player: Address, hint_type: u32) {
        HintPurchaseSystem::purchase_hint(&env, &player, hint_type);
    }

    pub fn get_state(env: Env) -> GameState {
        GameStatusComponent::get_state(&env)
    }

    pub fn is_finished(env: Env) -> bool {
        EndConditionSystem::is_finished(&env)
    }
}
