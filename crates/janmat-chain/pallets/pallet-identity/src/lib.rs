use janmat_zk::{
    CIRCUIT_KYC_VERIFY, ZkReceipt, ZkVerifier,
    circuits::kyc_verify::KycVerifyOutput,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct IdentityAnchor {
    pub wallet_pubkey: String,
    /// Hex of nullifier_seed_commitment from the KYC proof public outputs
    pub nullifier_seed_commitment_hex: String,
    pub registered_at_block: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentityError {
    AlreadyRegistered,
    WalletAlreadyBound,
    InvalidProof,
    InvalidPublicOutputs,
}

#[derive(Default)]
pub struct IdentityPallet {
    /// nullifier_seed_commitment_hex → anchor
    anchors: BTreeMap<String, IdentityAnchor>,
    /// wallet_pubkey set — one wallet per citizen
    wallet_index: BTreeSet<String>,
}

impl IdentityPallet {
    /// Register a wallet backed by a ZK KYC receipt.
    /// The verifier is injected so tests can use `MockVerifier` and
    /// production can plug in the on-chain RISC Zero verifier.
    pub fn register_wallet<V: ZkVerifier>(
        &mut self,
        wallet_pubkey: &str,
        kyc_receipt: &ZkReceipt,
        block_number: u64,
        verifier: &V,
    ) -> Result<IdentityAnchor, IdentityError> {
        // 1. Verify the receipt is a valid kyc-verify proof
        if !verifier.verify(kyc_receipt, CIRCUIT_KYC_VERIFY) {
            return Err(IdentityError::InvalidProof);
        }
        // 2. Deserialise public outputs to extract nullifier_seed_commitment
        let output: KycVerifyOutput =
            serde_json::from_slice(&kyc_receipt.public_outputs)
                .map_err(|_| IdentityError::InvalidPublicOutputs)?;
        let nsc_hex: String = output
            .nullifier_seed_commitment
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        // 3. Reject duplicate registrations
        if self.anchors.contains_key(&nsc_hex) {
            return Err(IdentityError::AlreadyRegistered);
        }
        if self.wallet_index.contains(wallet_pubkey) {
            return Err(IdentityError::WalletAlreadyBound);
        }
        let anchor = IdentityAnchor {
            wallet_pubkey: wallet_pubkey.to_owned(),
            nullifier_seed_commitment_hex: nsc_hex.clone(),
            registered_at_block: block_number,
        };
        self.anchors.insert(nsc_hex, anchor.clone());
        self.wallet_index.insert(wallet_pubkey.to_owned());
        Ok(anchor)
    }

    pub fn is_registered(&self, nullifier_seed_commitment_hex: &str) -> bool {
        self.anchors.contains_key(nullifier_seed_commitment_hex)
    }

    pub fn citizen_count(&self) -> usize {
        self.anchors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use janmat_zk::{MockVerifier, prover::MockProver};
    use janmat_identity::kyc_provider::{KycProvider, MockKycProvider};

    fn register(
        pallet: &mut IdentityPallet,
        wallet: &str,
        subject: &str,
    ) -> Result<IdentityAnchor, IdentityError> {
        let cred = MockKycProvider.issue_credential(subject);
        let receipt = MockProver.prove_kyc(&cred.to_kyc_input()).unwrap();
        pallet.register_wallet(wallet, &receipt, 1, &MockVerifier)
    }

    #[test]
    fn full_kyc_registration_flow() {
        let mut p = IdentityPallet::default();
        register(&mut p, "wallet-A", "citizen-001").unwrap();
        assert_eq!(p.citizen_count(), 1);
    }

    #[test]
    fn double_registration_same_credential_rejected() {
        let mut p = IdentityPallet::default();
        register(&mut p, "wallet-A", "citizen-001").unwrap();
        // Second attempt with same credential → same nullifier_seed_commitment → rejected
        let err = register(&mut p, "wallet-B", "citizen-001").unwrap_err();
        assert_eq!(err, IdentityError::AlreadyRegistered);
    }

    #[test]
    fn same_wallet_two_credentials_rejected() {
        let mut p = IdentityPallet::default();
        register(&mut p, "wallet-A", "citizen-001").unwrap();
        let err = register(&mut p, "wallet-A", "citizen-002").unwrap_err();
        assert_eq!(err, IdentityError::WalletAlreadyBound);
    }

    #[test]
    fn invalid_proof_rejected() {
        let mut p = IdentityPallet::default();
        let bad_receipt = ZkReceipt {
            circuit_id: "wrong-circuit".to_owned(),
            proof_bytes: vec![1, 2, 3],
            public_outputs: vec![],
        };
        let err = p
            .register_wallet("wallet-X", &bad_receipt, 1, &MockVerifier)
            .unwrap_err();
        assert_eq!(err, IdentityError::InvalidProof);
    }
}
