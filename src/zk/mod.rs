//! Zero-knowledge proof support for Cougr.
//!
//! This module provides ergonomic wrappers around Stellar Protocol 25 (X-Ray)
//! cryptographic host functions for use in on-chain game verification.
//!
//! ## Architecture
//!
//! - **`types`**: Core ZK types (`G1Point`, `G2Point`, `Scalar`, `Groth16Proof`, `VerificationKey`)
//! - **`crypto`**: Low-level BN254 and Poseidon wrappers
//! - **`groth16`**: Groth16 proof verification
//! - **`error`**: ZK-specific error types
//! - **`testing`**: Mock types for unit testing without real proofs
//!
//! ## Usage
//!
//! ```ignore
//! use cougr_core::zk::{crypto, groth16, types::*};
//!
//! // Verify a Groth16 proof on-chain
//! let result = groth16::verify_groth16(&env, &vk, &proof, &public_inputs);
//! ```

pub mod crypto;
pub mod error;
pub mod groth16;
pub mod testing;
pub mod types;

// Re-export commonly used items
pub use error::ZKError;
pub use groth16::verify_groth16;
pub use types::{G1Point, G2Point, Groth16Proof, Scalar, VerificationKey};
