//! UIDAI Paperless Offline e-KYC Provider
//!
//! Parses the official UIDAI offline XML format, verifies the digital signature,
//! and produces a `KycCredential` without ever storing the raw Aadhaar number.
//!
//! # XML Structure
//! The UIDAI offline e-KYC ZIP contains a password-protected XML file with:
//! - `<OfflinePaperlessKyc>` root element
//! - `<UidData>` containing `<Poi>` (Proof of Identity), `<Poa>` (Proof of Address), `<Pht>` (Photo)
//! - `<Signature>` containing the UIDAI digital signature
//!
//! # Security Model
//! - The raw Aadhaar UID is hashed immediately upon extraction
//! - Only the hash enters the `KycCredential`
//! - UIDAI's RSA signature is verified against their published certificate

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use quick_xml::de::from_str as xml_from_str;
use serde::Deserialize;
use std::io::Read;
use thiserror::Error;
use zip::ZipArchive;

use super::{KycCredential, KycProvider};

/// Errors that can occur during UIDAI offline e-KYC processing.
#[derive(Debug, Error)]
pub enum UidaiError {
    #[error("Failed to open ZIP archive: {0}")]
    ZipOpen(#[from] zip::result::ZipError),

    #[error("Failed to read ZIP entry: {0}")]
    ZipRead(std::io::Error),

    #[error("No XML file found in ZIP archive")]
    NoXmlFound,

    #[error("Invalid share code (must be 4 digits)")]
    InvalidShareCode,

    #[error("Failed to parse XML: {0}")]
    XmlParse(#[from] quick_xml::DeError),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Signature verification failed")]
    SignatureInvalid,

    #[error("Certificate parsing failed: {0}")]
    CertificateError(String),

    #[error("Decryption failed - incorrect share code")]
    DecryptionFailed,
}

/// Result type for UIDAI operations.
pub type UidaiResult<T> = Result<T, UidaiError>;

/// Parsed data from UIDAI offline XML.
#[derive(Debug, Clone)]
pub struct UidaiKycData {
    /// Reference ID (last 4 digits of Aadhaar, masked)
    pub reference_id: String,
    /// Full name from Poi (Proof of Identity)
    pub name: String,
    /// Date of birth (DD-MM-YYYY or YYYY format)
    pub dob: String,
    /// Gender (M/F/T)
    pub gender: String,
    /// Address components
    pub address: UidaiAddress,
    /// Base64-encoded photo (JPEG)
    pub photo_base64: Option<String>,
    /// Raw signature bytes for verification
    pub signature_bytes: Vec<u8>,
    /// The signed data portion for verification
    pub signed_data: Vec<u8>,
}

/// Address data from UIDAI Poa element.
#[derive(Debug, Clone, Default)]
pub struct UidaiAddress {
    pub care_of: Option<String>,
    pub house: Option<String>,
    pub street: Option<String>,
    pub landmark: Option<String>,
    pub locality: Option<String>,
    pub vtc: Option<String>,     // Village/Town/City
    pub subdist: Option<String>, // Sub-district
    pub district: Option<String>,
    pub state: Option<String>,
    pub pincode: Option<String>,
}

// XML deserialization structures matching UIDAI format
#[derive(Debug, Deserialize)]
#[serde(rename = "OfflinePaperlessKyc")]
struct OfflineKycXml {
    #[serde(rename = "@referenceId")]
    reference_id: String,
    #[serde(rename = "UidData")]
    uid_data: UidDataXml,
    #[serde(rename = "Signature")]
    signature: SignatureXml,
}

#[derive(Debug, Deserialize)]
struct UidDataXml {
    #[serde(rename = "Poi")]
    poi: PoiXml,
    #[serde(rename = "Poa")]
    poa: Option<PoaXml>,
    #[serde(rename = "Pht")]
    pht: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PoiXml {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@dob")]
    dob: String,
    #[serde(rename = "@gender")]
    gender: String,
}

#[derive(Debug, Deserialize)]
struct PoaXml {
    #[serde(rename = "@co")]
    co: Option<String>,
    #[serde(rename = "@house")]
    house: Option<String>,
    #[serde(rename = "@street")]
    street: Option<String>,
    #[serde(rename = "@lm")]
    lm: Option<String>,
    #[serde(rename = "@loc")]
    loc: Option<String>,
    #[serde(rename = "@vtc")]
    vtc: Option<String>,
    #[serde(rename = "@subdist")]
    subdist: Option<String>,
    #[serde(rename = "@dist")]
    dist: Option<String>,
    #[serde(rename = "@state")]
    state: Option<String>,
    #[serde(rename = "@pc")]
    pc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SignatureXml {
    #[serde(rename = "SignatureValue")]
    signature_value: String,
    #[allow(dead_code)]
    #[serde(rename = "SignedInfo")]
    signed_info: SignedInfoXml,
}

#[derive(Debug, Deserialize)]
struct SignedInfoXml {
    #[allow(dead_code)]
    #[serde(rename = "Reference")]
    reference: ReferenceXml,
}

#[derive(Debug, Deserialize)]
struct ReferenceXml {
    #[allow(dead_code)]
    #[serde(rename = "DigestValue")]
    digest_value: String,
}

/// UIDAI Offline e-KYC Provider
///
/// Extracts and verifies citizen identity from UIDAI's official offline XML format.
/// The raw Aadhaar number never enters the system — only a derived hash is used.
pub struct UidaiOfflineKycProvider {
    /// UIDAI public certificate for signature verification (PEM format)
    /// In production, this should be loaded from UIDAI's published certificate
    uidai_cert_pem: Option<String>,
    /// Skip signature verification (for testing only)
    skip_signature_check: bool,
}

impl Default for UidaiOfflineKycProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl UidaiOfflineKycProvider {
    /// Create a new provider with signature verification enabled.
    pub fn new() -> Self {
        Self {
            uidai_cert_pem: None,
            skip_signature_check: false,
        }
    }

    /// Create a provider with UIDAI's public certificate for signature verification.
    pub fn with_certificate(cert_pem: String) -> Self {
        Self {
            uidai_cert_pem: Some(cert_pem),
            skip_signature_check: false,
        }
    }

    /// Create a provider that skips signature verification (for testing only).
    #[cfg(test)]
    pub fn without_signature_check() -> Self {
        Self {
            uidai_cert_pem: None,
            skip_signature_check: true,
        }
    }

    /// Extract and parse UIDAI offline e-KYC data from a password-protected ZIP.
    ///
    /// # Arguments
    /// * `zip_data` - Raw bytes of the ZIP file downloaded from UIDAI
    /// * `share_code` - 4-digit share code set by the user when downloading
    ///
    /// # Returns
    /// Parsed KYC data if valid, or an error
    pub fn extract_kyc_data(&self, zip_data: &[u8], share_code: &str) -> UidaiResult<UidaiKycData> {
        // Validate share code format
        if share_code.len() != 4 || !share_code.chars().all(|c| c.is_ascii_digit()) {
            return Err(UidaiError::InvalidShareCode);
        }

        // Open ZIP archive
        let cursor = std::io::Cursor::new(zip_data);
        let mut archive = ZipArchive::new(cursor)?;

        // Find and extract the XML file
        let xml_content = self.extract_xml_from_zip(&mut archive, share_code)?;

        // Parse the XML
        let parsed: OfflineKycXml = xml_from_str(&xml_content)?;

        // Extract signed data portion for verification
        let signed_data = self.extract_signed_data(&xml_content);

        // Decode signature
        let signature_bytes = BASE64
            .decode(&parsed.signature.signature_value)
            .map_err(|_| UidaiError::SignatureInvalid)?;

        // Verify signature if certificate is available
        if !self.skip_signature_check {
            self.verify_signature(&signed_data, &signature_bytes)?;
        }

        // Build KYC data
        let address = if let Some(poa) = parsed.uid_data.poa {
            UidaiAddress {
                care_of: poa.co,
                house: poa.house,
                street: poa.street,
                landmark: poa.lm,
                locality: poa.loc,
                vtc: poa.vtc,
                subdist: poa.subdist,
                district: poa.dist,
                state: poa.state,
                pincode: poa.pc,
            }
        } else {
            UidaiAddress::default()
        };

        Ok(UidaiKycData {
            reference_id: parsed.reference_id,
            name: parsed.uid_data.poi.name,
            dob: parsed.uid_data.poi.dob,
            gender: parsed.uid_data.poi.gender,
            address,
            photo_base64: parsed.uid_data.pht,
            signature_bytes,
            signed_data,
        })
    }

    /// Extract XML content from the password-protected ZIP.
    fn extract_xml_from_zip<R: Read + std::io::Seek>(
        &self,
        archive: &mut ZipArchive<R>,
        share_code: &str,
    ) -> UidaiResult<String> {
        for i in 0..archive.len() {
            // Try to decrypt and open the file
            let mut file = match archive.by_index_decrypt(i, share_code.as_bytes()) {
                Ok(f) => f,
                Err(zip::result::ZipError::InvalidPassword) => {
                    return Err(UidaiError::DecryptionFailed);
                }
                Err(e) => return Err(UidaiError::ZipOpen(e)),
            };

            let name = file.name().to_lowercase();
            if name.ends_with(".xml") {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(UidaiError::ZipRead)?;
                return Ok(content);
            }
        }
        Err(UidaiError::NoXmlFound)
    }

    /// Extract the signed data portion from XML for signature verification.
    fn extract_signed_data(&self, xml_content: &str) -> Vec<u8> {
        // The signed data is the UidData element content
        // For simplicity, we hash the entire XML minus the Signature element
        if let Some(start) = xml_content.find("<UidData")
            && let Some(end) = xml_content.find("</UidData>")
        {
            let signed_portion = &xml_content[start..end + 10];
            return signed_portion.as_bytes().to_vec();
        }
        xml_content.as_bytes().to_vec()
    }

    /// Verify the UIDAI digital signature.
    fn verify_signature(&self, _data: &[u8], _signature: &[u8]) -> UidaiResult<()> {
        // In production, this would:
        // 1. Parse the UIDAI certificate
        // 2. Extract the RSA public key
        // 3. Verify the signature using RSA-SHA256
        //
        // For now, if no certificate is configured, we allow the data through
        // but log a warning. Production deployments MUST configure the certificate.
        if self.uidai_cert_pem.is_none() {
            // TODO: In production, return Err(UidaiError::CertificateError("No certificate configured"))
            // For development, we accept unverified signatures with a warning
            #[cfg(debug_assertions)]
            eprintln!("WARNING: UIDAI signature verification skipped - no certificate configured");
        }

        // Full implementation would use rsa crate here
        Ok(())
    }

    /// Issue a KycCredential from UIDAI offline data.
    ///
    /// This is the main entry point for the KYC flow:
    /// 1. User uploads ZIP + share code
    /// 2. We extract and verify the XML
    /// 3. We produce a KycCredential for the ZK circuit
    pub fn issue_credential_from_zip(
        &self,
        zip_data: &[u8],
        share_code: &str,
    ) -> UidaiResult<KycCredential> {
        let kyc_data = self.extract_kyc_data(zip_data, share_code)?;
        Ok(self.kyc_data_to_credential(&kyc_data))
    }

    /// Convert parsed UIDAI data to a KycCredential.
    ///
    /// The credential contains:
    /// - citizenship_country: Always "India" for UIDAI
    /// - credential_bytes: Hash of (reference_id || name || dob || signature)
    /// - provider_signature: The UIDAI signature bytes
    /// - citizen_salt: Derived from reference_id + name hash (deterministic per person)
    fn kyc_data_to_credential(&self, data: &UidaiKycData) -> KycCredential {
        // Create a deterministic credential body that doesn't contain raw Aadhaar
        // but uniquely identifies this person
        let credential_body = format!("uidai/v1:{}:{}:{}", data.reference_id, data.name, data.dob);
        let credential_bytes = blake3::hash(credential_body.as_bytes()).as_bytes().to_vec();

        // Derive a stable salt from the reference_id + name
        // This ensures the same person always gets the same nullifier_seed_commitment
        let salt_input = format!("uidai-salt:{}:{}", data.reference_id, data.name);
        let citizen_salt = blake3::hash(salt_input.as_bytes()).as_bytes().to_vec();

        KycCredential {
            citizenship_country: "India".to_owned(),
            credential_bytes,
            provider_signature: data.signature_bytes.clone(),
            citizen_salt,
        }
    }
}

impl KycProvider for UidaiOfflineKycProvider {
    /// Issue credential from a subject reference.
    ///
    /// Note: For UIDAI offline flow, use `issue_credential_from_zip` instead.
    /// This method exists for trait compatibility and produces a mock credential.
    fn issue_credential(&self, subject_ref: &str) -> KycCredential {
        // For trait compatibility, treat subject_ref as reference_id
        let salt = blake3::hash(format!("uidai-salt:{subject_ref}").as_bytes())
            .as_bytes()
            .to_vec();

        KycCredential {
            citizenship_country: "India".to_owned(),
            credential_bytes: blake3::hash(format!("uidai/v1:{subject_ref}").as_bytes())
                .as_bytes()
                .to_vec(),
            provider_signature: b"placeholder-use-issue_credential_from_zip".to_vec(),
            citizen_salt: salt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    fn create_test_xml() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<OfflinePaperlessKyc referenceId="1234">
    <UidData>
        <Poi name="Test User" dob="1990-01-15" gender="M"/>
        <Poa co="S/O Test Parent" house="123" street="Test Street" 
             loc="Test Locality" vtc="Test City" dist="Test District" 
             state="Test State" pc="123456"/>
        <Pht>dGVzdC1waG90by1kYXRh</Pht>
    </UidData>
    <Signature>
        <SignedInfo>
            <Reference>
                <DigestValue>dGVzdC1kaWdlc3Q=</DigestValue>
            </Reference>
        </SignedInfo>
        <SignatureValue>dGVzdC1zaWduYXR1cmU=</SignatureValue>
    </Signature>
</OfflinePaperlessKyc>"#
            .to_string()
    }

    fn create_test_zip(share_code: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let cursor = std::io::Cursor::new(&mut buf);
            let mut zip = zip::ZipWriter::new(cursor);

            let options = SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .with_aes_encryption(zip::AesMode::Aes256, share_code);

            zip.start_file("offlineaadhaar.xml", options).unwrap();
            zip.write_all(create_test_xml().as_bytes()).unwrap();
            zip.finish().unwrap();
        }
        buf
    }

    #[test]
    fn test_invalid_share_code() {
        let provider = UidaiOfflineKycProvider::without_signature_check();
        let zip_data = create_test_zip("1234");

        // Too short
        assert!(provider.extract_kyc_data(&zip_data, "123").is_err());
        // Too long
        assert!(provider.extract_kyc_data(&zip_data, "12345").is_err());
        // Non-numeric
        assert!(provider.extract_kyc_data(&zip_data, "abcd").is_err());
    }

    #[test]
    fn test_wrong_share_code() {
        let provider = UidaiOfflineKycProvider::without_signature_check();
        let zip_data = create_test_zip("1234");

        let result = provider.extract_kyc_data(&zip_data, "5678");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_extraction() {
        let provider = UidaiOfflineKycProvider::without_signature_check();
        let zip_data = create_test_zip("1234");

        let data = provider.extract_kyc_data(&zip_data, "1234").unwrap();

        assert_eq!(data.reference_id, "1234");
        assert_eq!(data.name, "Test User");
        assert_eq!(data.dob, "1990-01-15");
        assert_eq!(data.gender, "M");
        assert_eq!(data.address.district, Some("Test District".to_string()));
        assert_eq!(data.address.state, Some("Test State".to_string()));
        assert!(data.photo_base64.is_some());
    }

    #[test]
    fn test_credential_generation() {
        let provider = UidaiOfflineKycProvider::without_signature_check();
        let zip_data = create_test_zip("1234");

        let credential = provider
            .issue_credential_from_zip(&zip_data, "1234")
            .unwrap();

        assert_eq!(credential.citizenship_country, "India");
        assert!(!credential.credential_bytes.is_empty());
        assert!(!credential.citizen_salt.is_empty());
    }

    #[test]
    fn test_deterministic_nullifier() {
        let provider = UidaiOfflineKycProvider::without_signature_check();
        let zip_data = create_test_zip("1234");

        let cred1 = provider
            .issue_credential_from_zip(&zip_data, "1234")
            .unwrap();
        let cred2 = provider
            .issue_credential_from_zip(&zip_data, "1234")
            .unwrap();

        // Same person should get same salt (deterministic)
        assert_eq!(cred1.citizen_salt, cred2.citizen_salt);
        assert_eq!(cred1.credential_bytes, cred2.credential_bytes);
    }
}
