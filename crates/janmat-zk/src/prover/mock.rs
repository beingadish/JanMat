//! MockProver — runs the circuit logic locally and produces a deterministic
//! ZkReceipt whose `proof_bytes` is Blake3(serialised_public_outputs).
//!
//! This lets the rest of the system (pallet-identity, pallet-voting, tests)
//! exercise the full receipt-based flow without the RISC Zero toolchain.
//! In production, replace `MockProver::prove_kyc` / `prove_vote` calls with
//! the RISC Zero host-side `prove()` API — the receipt shape is identical.

use crate::{
    CIRCUIT_KYC_VERIFY, CIRCUIT_VOTE_CAST, ZkReceipt,
    circuits::{
        kyc_verify::{self, KycVerifyInput},
        vote_cast::{self, VoteCastInput},
    },
};

pub struct MockProver;

impl MockProver {
    /// Prove a KYC credential. Returns `Ok(ZkReceipt)` on success.
    pub fn prove_kyc(&self, input: &KycVerifyInput) -> Result<ZkReceipt, String> {
        let output = kyc_verify::execute(input).map_err(|e| e.to_owned())?;
        let public_bytes =
            serde_json::to_vec(&output).expect("serialise KycVerifyOutput");
        let proof_bytes = blake3::hash(&public_bytes).as_bytes().to_vec();
        Ok(ZkReceipt {
            circuit_id: CIRCUIT_KYC_VERIFY.to_owned(),
            proof_bytes,
            public_outputs: public_bytes,
        })
    }

    /// Prove a vote cast. Returns `Ok(ZkReceipt)` on success.
    pub fn prove_vote(&self, input: &VoteCastInput) -> Result<ZkReceipt, String> {
        let output = vote_cast::execute(input).map_err(|e| e.to_owned())?;
        let public_bytes =
            serde_json::to_vec(&output).expect("serialise VoteCastOutput");
        let proof_bytes = blake3::hash(&public_bytes).as_bytes().to_vec();
        Ok(ZkReceipt {
            circuit_id: CIRCUIT_VOTE_CAST.to_owned(),
            proof_bytes,
            public_outputs: public_bytes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuits::{kyc_verify::KycVerifyInput, vote_cast::VoteCastInput};

    fn kyc_input() -> KycVerifyInput {
        KycVerifyInput {
            credential_bytes: b"mock-credential".to_vec(),
            provider_signature: b"mock-sig".to_vec(),
            citizen_salt: b"salt-32-bytes-padded-pad-pad-pad!".to_vec(),
            citizenship_country: "India".to_owned(),
        }
    }

    #[test]
    fn prove_kyc_produces_receipt() {
        let receipt = MockProver.prove_kyc(&kyc_input()).unwrap();
        assert_eq!(receipt.circuit_id, CIRCUIT_KYC_VERIFY);
        assert!(!receipt.proof_bytes.is_empty());
        assert!(!receipt.public_outputs.is_empty());
    }

    #[test]
    fn prove_kyc_non_india_fails() {
        let mut inp = kyc_input();
        inp.citizenship_country = "Pakistan".to_owned();
        assert!(MockProver.prove_kyc(&inp).is_err());
    }

    #[test]
    fn prove_vote_produces_receipt() {
        let inp = VoteCastInput {
            nullifier_seed: b"seed-from-kyc".to_vec(),
            vote_id: "v/01HXYZ".to_owned(),
            vote_choice: "YES".to_owned(),
            nonce: b"random-nonce".to_vec(),
        };
        let receipt = MockProver.prove_vote(&inp).unwrap();
        assert_eq!(receipt.circuit_id, CIRCUIT_VOTE_CAST);
    }
}
