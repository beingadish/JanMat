//! Nullifier derivation helpers for the identity + voting layers.
//!
//! These functions mirror the logic inside the ZK circuits — any change here
//! must be reflected in `janmat-zk/circuits/kyc_verify.rs` and `vote_cast.rs`.

use blake3::Hasher;

/// Derive the per-citizen nullifier seed from credential bytes and salt.
/// Mirrors `circuits::kyc_verify::derive_nullifier_seed`.
pub fn derive_nullifier_seed(credential_bytes: &[u8], citizen_salt: &[u8]) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/nullifier_seed/v1");
    h.update(credential_bytes);
    h.update(citizen_salt);
    *h.finalize().as_bytes()
}

/// Derive the per-vote nullifier: Blake3(domain || seed || vote_id).
/// Stored on-chain by `pallet-voting` to prevent double-voting.
/// Mirrors `circuits::vote_cast::derive_vote_nullifier`.
pub fn derive_vote_nullifier(nullifier_seed: &[u8], vote_id: &str) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/vote_nullifier/v1");
    h.update(nullifier_seed);
    h.update(vote_id.as_bytes());
    *h.finalize().as_bytes()
}

/// Derive a vote commitment: Blake3(domain || choice || nonce).
/// Seals the vote choice until the optional reveal toggle.
pub fn derive_commitment(vote_choice: &str, nonce: &[u8]) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/vote_commitment/v1");
    h.update(vote_choice.as_bytes());
    h.update(nonce);
    *h.finalize().as_bytes()
}

/// Hex-encode a 32-byte array for on-chain storage / display.
pub fn to_hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nullifier_seed_is_deterministic() {
        let a = derive_nullifier_seed(b"cred", b"salt");
        let b = derive_nullifier_seed(b"cred", b"salt");
        assert_eq!(a, b);
    }

    #[test]
    fn different_citizens_different_seeds() {
        let a = derive_nullifier_seed(b"cred-alice", b"salt-A");
        let b = derive_nullifier_seed(b"cred-bob", b"salt-B");
        assert_ne!(a, b);
    }

    #[test]
    fn vote_nullifier_unique_per_vote() {
        let seed = derive_nullifier_seed(b"cred", b"salt");
        let n1 = derive_vote_nullifier(&seed, "v/VOTE_A");
        let n2 = derive_vote_nullifier(&seed, "v/VOTE_B");
        assert_ne!(n1, n2);
    }

    #[test]
    fn commitment_hides_choice() {
        let c_yes = derive_commitment("YES", b"nonce");
        let c_no = derive_commitment("NO", b"nonce");
        assert_ne!(c_yes, c_no);
    }
}
