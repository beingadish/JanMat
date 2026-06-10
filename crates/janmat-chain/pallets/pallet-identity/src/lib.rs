use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct IdentityAnchor {
    pub wallet_pubkey: String,
    pub nullifier_commitment: String,
    pub registered_at_block: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentityError {
    AlreadyRegistered,
    WalletAlreadyBound,
    InvalidProof,
}

#[derive(Default)]
pub struct IdentityPallet {
    /// nullifier_commitment → anchor
    anchors: BTreeMap<String, IdentityAnchor>,
    /// wallet_pubkey → nullifier_commitment (one wallet per citizen)
    wallet_index: BTreeSet<String>,
}

impl IdentityPallet {
    /// Register a wallet with a kyc_proof receipt.
    /// Phase 1: accepts any non-empty proof string as placeholder.
    /// Phase 2 will enforce a real RISC Zero receipt.
    pub fn register_wallet(
        &mut self,
        wallet_pubkey: &str,
        nullifier_commitment: &str,
        kyc_proof: &str,
        block_number: u64,
    ) -> Result<IdentityAnchor, IdentityError> {
        if kyc_proof.is_empty() {
            return Err(IdentityError::InvalidProof);
        }
        if self.anchors.contains_key(nullifier_commitment) {
            return Err(IdentityError::AlreadyRegistered);
        }
        if self.wallet_index.contains(wallet_pubkey) {
            return Err(IdentityError::WalletAlreadyBound);
        }
        let anchor = IdentityAnchor {
            wallet_pubkey: wallet_pubkey.to_owned(),
            nullifier_commitment: nullifier_commitment.to_owned(),
            registered_at_block: block_number,
        };
        self.anchors
            .insert(nullifier_commitment.to_owned(), anchor.clone());
        self.wallet_index.insert(wallet_pubkey.to_owned());
        Ok(anchor)
    }

    pub fn is_registered(&self, nullifier_commitment: &str) -> bool {
        self.anchors.contains_key(nullifier_commitment)
    }

    pub fn citizen_count(&self) -> usize {
        self.anchors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_new_wallet() {
        let mut p = IdentityPallet::default();
        p.register_wallet("wallet-A", "null-A", "proof-bytes", 100)
            .unwrap();
        assert_eq!(p.citizen_count(), 1);
    }

    #[test]
    fn same_nullifier_rejected() {
        let mut p = IdentityPallet::default();
        p.register_wallet("wallet-A", "null-A", "proof", 1).unwrap();
        let err = p
            .register_wallet("wallet-B", "null-A", "proof", 2)
            .unwrap_err();
        assert_eq!(err, IdentityError::AlreadyRegistered);
    }

    #[test]
    fn same_wallet_rejected() {
        let mut p = IdentityPallet::default();
        p.register_wallet("wallet-A", "null-A", "proof", 1).unwrap();
        let err = p
            .register_wallet("wallet-A", "null-B", "proof", 2)
            .unwrap_err();
        assert_eq!(err, IdentityError::WalletAlreadyBound);
    }

    #[test]
    fn empty_proof_rejected() {
        let mut p = IdentityPallet::default();
        let err = p
            .register_wallet("wallet-X", "null-X", "", 1)
            .unwrap_err();
        assert_eq!(err, IdentityError::InvalidProof);
    }
}
