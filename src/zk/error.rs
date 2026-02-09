use soroban_sdk::contracterror;

/// Error types for the Cougr ZK subsystem.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ZKError {
    /// The submitted proof is structurally invalid.
    InvalidProof = 10,
    /// An elliptic curve point is not on the curve or not in the subgroup.
    InvalidPoint = 11,
    /// A scalar value is out of range for the target field.
    InvalidScalar = 12,
    /// Proof verification failed (valid structure, but proof is incorrect).
    VerificationFailed = 13,
    /// Input data is malformed or has the wrong length.
    InvalidInput = 14,
    /// The verification key is malformed or incompatible with the proof.
    InvalidVerificationKey = 15,
}
