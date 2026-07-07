use janmat_zk::circuits::kyc_verify::{self, KycVerifyInput};

pub mod uidai_offline;
pub use uidai_offline::{UidaiError, UidaiKycData, UidaiOfflineKycProvider, UidaiResult};

/// A KYC credential issued by a licensed provider.
/// The raw Aadhaar number never enters this struct.
#[derive(Debug, Clone)]
pub struct KycCredential {
    pub citizenship_country: String,
    /// Raw credential body (opaque bytes in prod; deterministic mock in tests)
    pub credential_bytes: Vec<u8>,
    /// Provider's signature over credential_bytes
    pub provider_signature: Vec<u8>,
    /// Per-citizen salt — derived from credential, never the raw Aadhaar number
    pub citizen_salt: Vec<u8>,
}

impl KycCredential {
    /// Convert to the ZK circuit input (private witness).
    pub fn to_kyc_input(&self) -> KycVerifyInput {
        KycVerifyInput {
            credential_bytes: self.credential_bytes.clone(),
            provider_signature: self.provider_signature.clone(),
            citizen_salt: self.citizen_salt.clone(),
            citizenship_country: self.citizenship_country.clone(),
        }
    }
}

pub trait KycProvider {
    fn issue_credential(&self, subject_ref: &str) -> KycCredential;
}

/// Mock provider used in all pre-Phase-6 environments.
/// Issues deterministic credentials from a subject reference string.
pub struct MockKycProvider;

impl KycProvider for MockKycProvider {
    fn issue_credential(&self, subject_ref: &str) -> KycCredential {
        // Derive a stable, reproducible salt so the same subject always
        // gets the same nullifier_seed_commitment — mirroring real UIDAI behaviour.
        let salt = blake3::hash(
            format!("mock-salt:{subject_ref}").as_bytes()
        ).as_bytes().to_vec();

        KycCredential {
            citizenship_country: "India".to_owned(),
            credential_bytes: format!("mock-credential:{subject_ref}").into_bytes(),
            provider_signature: b"mock-provider-signature".to_vec(),
            citizen_salt: salt,
        }
    }
}

/// Derive the on-chain nullifier_seed_commitment for a credential.
/// This is what `pallet-identity` stores — never the seed itself.
pub fn nullifier_seed_commitment(cred: &KycCredential) -> [u8; 32] {
    let seed = kyc_verify::derive_nullifier_seed(&cred.credential_bytes, &cred.citizen_salt);
    *blake3::hash(&seed).as_bytes()
}

/// POC identity anchor: a deterministic commitment over PAN + DOB + Aadhaar.
///
/// This BYPASSES real UIDAI verification and exists only for the proof of
/// concept (obtaining the production KYC licence takes time). The same person
/// always maps to the same commitment — giving one citizen exactly one
/// nullifier seed — and the raw identifiers are never stored, only this digest.
pub fn poc_identity_commitment(pan: &str, dob: &str, aadhaar: &str) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/poc-identity/v1");
    h.update(pan.trim().to_uppercase().as_bytes());
    h.update(b"|");
    h.update(dob.trim().as_bytes());
    h.update(b"|");
    h.update(aadhaar.trim().as_bytes());
    *h.finalize().as_bytes()
}

/// POC unique citizen UID — a time-based hash over PAN + DOB + Aadhaar + phone
/// plus a caller-supplied unique salt (e.g. a fresh ULID). Because the salt is
/// time-based and random, every registration yields a distinct UID. No external
/// verification is performed — this is strictly for the proof of concept.
pub fn poc_uid(pan: &str, dob: &str, aadhaar: &str, phone: &str, salt: &str) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/poc-uid/v1");
    for part in [pan.trim().to_uppercase().as_str(), dob.trim(), aadhaar.trim(), phone.trim(), salt] {
        h.update(part.as_bytes());
        h.update(b"|");
    }
    *h.finalize().as_bytes()
}
