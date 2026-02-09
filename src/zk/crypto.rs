use soroban_sdk::crypto::bn254::{Bn254G1Affine, Bn254G2Affine, Fr};
use soroban_sdk::{Env, Vec};

use super::error::ZKError;
use super::types::{G1Point, G2Point, Scalar};

// ─── BN254 Wrappers ───────────────────────────────────────────

/// Add two BN254 G1 points.
///
/// Wraps `env.crypto().bn254().g1_add()`.
pub fn bn254_g1_add(env: &Env, p1: &G1Point, p2: &G1Point) -> Result<G1Point, ZKError> {
    let a = Bn254G1Affine::from_bytes(p1.bytes.clone());
    let b = Bn254G1Affine::from_bytes(p2.bytes.clone());
    let result = env.crypto().bn254().g1_add(&a, &b);
    Ok(G1Point {
        bytes: result.to_bytes(),
    })
}

/// Multiply a BN254 G1 point by a scalar.
///
/// Wraps `env.crypto().bn254().g1_mul()`.
pub fn bn254_g1_mul(env: &Env, point: &G1Point, scalar: &Scalar) -> Result<G1Point, ZKError> {
    let p = Bn254G1Affine::from_bytes(point.bytes.clone());
    let s = Fr::from_bytes(scalar.bytes.clone());
    let result = env.crypto().bn254().g1_mul(&p, &s);
    Ok(G1Point {
        bytes: result.to_bytes(),
    })
}

/// Perform a BN254 multi-pairing check.
///
/// Returns `true` if the pairing equation holds:
///   e(g1_points[0], g2_points[0]) * e(g1_points[1], g2_points[1]) * ... == 1
///
/// This is the core primitive for Groth16 verification.
pub fn bn254_pairing_check(
    env: &Env,
    g1_points: &[G1Point],
    g2_points: &[G2Point],
) -> Result<bool, ZKError> {
    if g1_points.len() != g2_points.len() {
        return Err(ZKError::InvalidInput);
    }
    if g1_points.is_empty() {
        return Err(ZKError::InvalidInput);
    }

    let mut vp1: Vec<Bn254G1Affine> = Vec::new(env);
    let mut vp2: Vec<Bn254G2Affine> = Vec::new(env);

    for p in g1_points {
        vp1.push_back(Bn254G1Affine::from_bytes(p.bytes.clone()));
    }
    for p in g2_points {
        vp2.push_back(Bn254G2Affine::from_bytes(p.bytes.clone()));
    }

    Ok(env.crypto().bn254().pairing_check(vp1, vp2))
}

// ─── Poseidon Wrappers ───────────────────────────────────────────
//
// Poseidon permutations require the `hazmat-crypto` feature.
// Enable it in Cargo.toml: `cougr-core = { features = ["hazmat-crypto"] }`

/// Compute a Poseidon permutation over field elements.
///
/// Requires the `hazmat-crypto` feature.
///
/// This is the low-level permutation function. Parameters:
/// - `input`: State vector of field elements (as `U256`)
/// - `field`: Field identifier symbol (e.g., `Symbol::new(env, "BN254")`)
/// - `t`: State size (width of the permutation)
/// - `d`: S-box degree (typically 5 for BN254)
/// - `rounds_f`: Number of full rounds (must be even)
/// - `rounds_p`: Number of partial rounds
/// - `mds`: MDS matrix as Vec of Vec of U256
/// - `round_constants`: Round constants as Vec of Vec of U256
///
/// Returns the permuted state vector.
#[cfg(feature = "hazmat-crypto")]
pub fn poseidon_permutation(
    env: &Env,
    input: &soroban_sdk::Vec<soroban_sdk::U256>,
    field: soroban_sdk::Symbol,
    t: u32,
    d: u32,
    rounds_f: u32,
    rounds_p: u32,
    mds: &soroban_sdk::Vec<soroban_sdk::Vec<soroban_sdk::U256>>,
    round_constants: &soroban_sdk::Vec<soroban_sdk::Vec<soroban_sdk::U256>>,
) -> soroban_sdk::Vec<soroban_sdk::U256> {
    let hazmat = soroban_sdk::crypto::CryptoHazmat::new(env);
    hazmat.poseidon_permutation(input, field, t, d, rounds_f, rounds_p, mds, round_constants)
}

/// Compute a Poseidon2 permutation over field elements.
///
/// Requires the `hazmat-crypto` feature.
///
/// Poseidon2 uses a diagonal internal matrix for faster computation.
///
/// Parameters are the same as `poseidon_permutation` except:
/// - `mat_internal_diag_m_1`: Diagonal entries of the internal matrix minus identity
#[cfg(feature = "hazmat-crypto")]
pub fn poseidon2_permutation(
    env: &Env,
    input: &soroban_sdk::Vec<soroban_sdk::U256>,
    field: soroban_sdk::Symbol,
    t: u32,
    d: u32,
    rounds_f: u32,
    rounds_p: u32,
    mat_internal_diag_m_1: &soroban_sdk::Vec<soroban_sdk::U256>,
    round_constants: &soroban_sdk::Vec<soroban_sdk::Vec<soroban_sdk::U256>>,
) -> soroban_sdk::Vec<soroban_sdk::U256> {
    let hazmat = soroban_sdk::crypto::CryptoHazmat::new(env);
    hazmat.poseidon2_permutation(
        input,
        field,
        t,
        d,
        rounds_f,
        rounds_p,
        mat_internal_diag_m_1,
        round_constants,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{BytesN, Env};

    #[test]
    fn test_pairing_check_empty_input_fails() {
        let result = bn254_pairing_check(&Env::default(), &[], &[]);
        assert_eq!(result, Err(ZKError::InvalidInput));
    }

    #[test]
    fn test_pairing_check_mismatched_lengths() {
        let env = Env::default();
        let g1 = G1Point {
            bytes: BytesN::from_array(&env, &[0u8; 64]),
        };
        let result = bn254_pairing_check(&env, &[g1], &[]);
        assert_eq!(result, Err(ZKError::InvalidInput));
    }
}
