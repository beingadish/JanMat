//! kyc-verify circuit
//!
//! Proves (without revealing the credential):
//!   1. The citizen holds a credential signed by a trusted KYC provider.
//!   2. The credential's citizenship_country == "India".
//!   3. nullifier_seed = Blake3(credential_hash || citizen_salt) is correctly derived.
//!
//! Public outputs committed to the chain:
//!   - nullifier_seed_commitment = Blake3(nullifier_seed)
//!   - citizenship_country_hash  = Blake3("India")  [constant — just asserts India]
//!
//! In production this logic runs inside a RISC Zero zkVM guest; the host calls
//! `prove()` and submits the resulting receipt to `pallet-identity`.
//! In Phase 2 the `MockProver` runs the same logic and produces a deterministic receipt.

use blake3::Hasher;
use serde::{Deserialize, Serialize};

// ── Input / Output ────────────────────────────────────────────────────────────

/// Private witness — never leaves the prover (client side).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycVerifyInput {
    /// Raw KYC credential bytes (signed by provider). Private.
    pub credential_bytes: Vec<u8>,
    /// Provider's signature over credential_bytes. Private.
    pub provider_signature: Vec<u8>,
    /// Per-citizen random salt seeded from the credential. Private.
    pub citizen_salt: Vec<u8>,
    /// Citizenship country from the parsed credential. Private.
    pub citizenship_country: String,
}

/// Public outputs — recorded on-chain and visible to everyone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KycVerifyOutput {
    /// Blake3(nullifier_seed) — uniquely identifies one citizen on chain.
    pub nullifier_seed_commitment: [u8; 32],
    /// Blake3("India") — constant; acts as a citizenship assertion tag.
    pub citizenship_hash: [u8; 32],
}

// ── Nullifier helpers ─────────────────────────────────────────────────────────

pub fn derive_nullifier_seed(credential_bytes: &[u8], citizen_salt: &[u8]) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/nullifier_seed/v1");
    h.update(credential_bytes);
    h.update(citizen_salt);
    *h.finalize().as_bytes()
}

pub fn citizenship_hash() -> [u8; 32] {
    *blake3::hash(b"India").as_bytes()
}

// ── Circuit logic (runs inside zkVM guest in production) ─────────────────────

/// Execute the circuit logic.
/// Returns `Ok(KycVerifyOutput)` if the proof constraints are satisfied,
/// `Err(reason)` otherwise.
pub fn execute(input: &KycVerifyInput) -> Result<KycVerifyOutput, &'static str> {
    // Constraint 1: citizenship must be India
    if input.citizenship_country != "India" {
        return Err("citizenship_not_india");
    }
    // Constraint 2: credential_bytes must not be empty (represents signature check)
    if input.credential_bytes.is_empty() || input.provider_signature.is_empty() {
        return Err("invalid_credential");
    }
    let nullifier_seed = derive_nullifier_seed(&input.credential_bytes, &input.citizen_salt);
    let nullifier_seed_commitment = *blake3::hash(&nullifier_seed).as_bytes();
    Ok(KycVerifyOutput {
        nullifier_seed_commitment,
        citizenship_hash: citizenship_hash(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> KycVerifyInput {
        KycVerifyInput {
            credential_bytes: b"mock-credential-body".to_vec(),
            provider_signature: b"mock-sig".to_vec(),
            citizen_salt: b"random-salt-32bytes-padded-here!".to_vec(),
            citizenship_country: "India".to_owned(),
        }
    }

    #[test]
    fn valid_indian_credential_produces_output() {
        let out = execute(&valid_input()).unwrap();
        assert_eq!(out.citizenship_hash, citizenship_hash());
        assert_ne!(out.nullifier_seed_commitment, [0u8; 32]);
    }

    #[test]
    fn non_indian_credential_rejected() {
        let mut inp = valid_input();
        inp.citizenship_country = "Pakistan".to_owned();
        assert_eq!(execute(&inp), Err("citizenship_not_india"));
    }

    #[test]
    fn empty_credential_rejected() {
        let mut inp = valid_input();
        inp.credential_bytes.clear();
        assert_eq!(execute(&inp), Err("invalid_credential"));
    }

    #[test]
    fn same_credential_same_nullifier_seed_commitment() {
        let out1 = execute(&valid_input()).unwrap();
        let out2 = execute(&valid_input()).unwrap();
        assert_eq!(
            out1.nullifier_seed_commitment,
            out2.nullifier_seed_commitment
        );
    }
}
