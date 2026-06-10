#[derive(Debug, Clone)]
pub struct KycCredential {
    pub citizenship_country: String,
    pub nullifier_seed_commitment: String,
}

pub trait KycProvider {
    fn issue_credential(&self, subject_ref: &str) -> KycCredential;
}

pub struct MockKycProvider;

impl KycProvider for MockKycProvider {
    fn issue_credential(&self, subject_ref: &str) -> KycCredential {
        KycCredential {
            citizenship_country: "India".to_owned(),
            nullifier_seed_commitment: format!("mock-{subject_ref}"),
        }
    }
}
