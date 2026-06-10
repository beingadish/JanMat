use janmat_zk::circuits::kyc_verify::{self, KycVerifyInput};

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
