pub mod circuits;
pub mod prover;

use serde::{Deserialize, Serialize};

// ── Circuit identifiers ───────────────────────────────────────────────────────

pub const CIRCUIT_KYC_VERIFY: &str = "janmat/kyc-verify/v1";
pub const CIRCUIT_VOTE_CAST: &str = "janmat/vote-cast/v1";

// ── ZK Receipt ────────────────────────────────────────────────────────────────

/// An opaque proof receipt produced by a ZK prover.
/// In production this holds a RISC Zero serialised receipt.
/// In Phase 2 / tests the `proof_bytes` is a deterministic mock commitment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkReceipt {
    pub circuit_id: String,
    /// Serialised proof bytes (RISC Zero receipt in production; mock hash in tests)
    pub proof_bytes: Vec<u8>,
    /// Public outputs committed to by the proof — the verifier reads only these
    pub public_outputs: Vec<u8>,
}

// ── Verifier trait ────────────────────────────────────────────────────────────

/// Any component that can verify a ZkReceipt against expected public outputs.
pub trait ZkVerifier {
    fn verify(&self, receipt: &ZkReceipt, expected_circuit: &str) -> bool;
}

// ── Default (mock) verifier — accepts any receipt with the right circuit id ──

pub struct MockVerifier;

impl ZkVerifier for MockVerifier {
    fn verify(&self, receipt: &ZkReceipt, expected_circuit: &str) -> bool {
        receipt.circuit_id == expected_circuit && !receipt.proof_bytes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_verifier_accepts_matching_circuit() {
        let receipt = ZkReceipt {
            circuit_id: CIRCUIT_KYC_VERIFY.to_owned(),
            proof_bytes: vec![1, 2, 3],
            public_outputs: vec![],
        };
        assert!(MockVerifier.verify(&receipt, CIRCUIT_KYC_VERIFY));
    }

    #[test]
    fn mock_verifier_rejects_wrong_circuit() {
        let receipt = ZkReceipt {
            circuit_id: CIRCUIT_KYC_VERIFY.to_owned(),
            proof_bytes: vec![1],
            public_outputs: vec![],
        };
        assert!(!MockVerifier.verify(&receipt, CIRCUIT_VOTE_CAST));
    }
}
