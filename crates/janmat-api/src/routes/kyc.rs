//! KYC routes for citizen identity verification.
//!
//! Handles UIDAI Offline e-KYC flow:
//! 1. User uploads ZIP file + share code
//! 2. Backend extracts and verifies XML
//! 3. Returns credential for ZK proof generation

use axum::{
    Json,
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
};
use janmat_identity::kyc_provider::{
    nullifier_seed_commitment, UidaiError, UidaiOfflineKycProvider,
};
use serde::Serialize;

/// Response from successful KYC verification.
#[derive(Debug, Serialize)]
pub struct KycVerifyResponse {
    /// Whether verification succeeded
    pub success: bool,
    /// Nullifier seed commitment for on-chain registration
    pub nullifier_seed_commitment: String,
    /// Citizenship country (always "India" for UIDAI)
    pub citizenship_country: String,
    /// Masked reference (last 4 digits, for display only)
    pub reference_id: String,
}

/// Error response for KYC verification failures.
#[derive(Debug, Serialize)]
pub struct KycErrorResponse {
    pub success: bool,
    pub error: String,
    pub error_code: String,
}

impl KycErrorResponse {
    fn from_uidai_error(e: UidaiError) -> (StatusCode, Self) {
        let (status, code) = match &e {
            UidaiError::InvalidShareCode => (StatusCode::BAD_REQUEST, "INVALID_SHARE_CODE"),
            UidaiError::DecryptionFailed => (StatusCode::UNAUTHORIZED, "WRONG_SHARE_CODE"),
            UidaiError::NoXmlFound => (StatusCode::BAD_REQUEST, "INVALID_ZIP"),
            UidaiError::XmlParse(_) => (StatusCode::BAD_REQUEST, "INVALID_XML"),
            UidaiError::SignatureInvalid => (StatusCode::UNAUTHORIZED, "INVALID_SIGNATURE"),
            UidaiError::MissingField(_) => (StatusCode::BAD_REQUEST, "MISSING_FIELD"),
            UidaiError::ZipOpen(_) | UidaiError::ZipRead(_) => {
                (StatusCode::BAD_REQUEST, "ZIP_ERROR")
            }
            UidaiError::CertificateError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "CERT_ERROR")
            }
        };

        (
            status,
            Self {
                success: false,
                error: e.to_string(),
                error_code: code.to_string(),
            },
        )
    }
}

/// Upload UIDAI offline e-KYC ZIP and verify identity.
///
/// # Request
/// Multipart form with:
/// - `file`: The ZIP file downloaded from UIDAI
/// - `share_code`: 4-digit code set when downloading
///
/// # Response
/// On success: `KycVerifyResponse` with nullifier commitment
/// On failure: `KycErrorResponse` with error details
pub async fn upload_kyc(mut multipart: Multipart) -> impl IntoResponse {
    let mut zip_data: Option<Vec<u8>> = None;
    let mut share_code: Option<String> = None;

    // Parse multipart form
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" | "zip" | "kyc_zip" => {
                match field.bytes().await {
                    Ok(bytes) => zip_data = Some(bytes.to_vec()),
                    Err(e) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(KycErrorResponse {
                                success: false,
                                error: format!("Failed to read file: {e}"),
                                error_code: "FILE_READ_ERROR".to_string(),
                            }),
                        )
                            .into_response();
                    }
                }
            }
            "share_code" | "shareCode" | "pin" => {
                match field.text().await {
                    Ok(text) => share_code = Some(text),
                    Err(e) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(KycErrorResponse {
                                success: false,
                                error: format!("Failed to read share code: {e}"),
                                error_code: "SHARE_CODE_READ_ERROR".to_string(),
                            }),
                        )
                            .into_response();
                    }
                }
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    // Validate required fields
    let zip_data = match zip_data {
        Some(data) => data,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(KycErrorResponse {
                    success: false,
                    error: "Missing ZIP file. Upload with field name 'file' or 'zip'.".to_string(),
                    error_code: "MISSING_FILE".to_string(),
                }),
            )
                .into_response();
        }
    };

    let share_code = match share_code {
        Some(code) => code,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(KycErrorResponse {
                    success: false,
                    error: "Missing share code. Provide with field name 'share_code'.".to_string(),
                    error_code: "MISSING_SHARE_CODE".to_string(),
                }),
            )
                .into_response();
        }
    };

    // Process KYC
    let provider = UidaiOfflineKycProvider::new();

    match provider.issue_credential_from_zip(&zip_data, &share_code) {
        Ok(credential) => {
            let commitment = nullifier_seed_commitment(&credential);
            let commitment_hex: String = commitment.iter().map(|b| format!("{b:02x}")).collect();

            // Extract reference_id from the data for display
            let kyc_data = provider.extract_kyc_data(&zip_data, &share_code).unwrap();

            (
                StatusCode::OK,
                Json(KycVerifyResponse {
                    success: true,
                    nullifier_seed_commitment: commitment_hex,
                    citizenship_country: credential.citizenship_country,
                    reference_id: kyc_data.reference_id,
                }),
            )
                .into_response()
        }
        Err(e) => {
            let (status, error_response) = KycErrorResponse::from_uidai_error(e);
            (status, Json(error_response)).into_response()
        }
    }
}

/// Health check for KYC service.
#[derive(Debug, Serialize)]
pub struct KycHealthResponse {
    pub service: &'static str,
    pub status: &'static str,
    pub uidai_cert_loaded: bool,
}

pub async fn kyc_health() -> Json<KycHealthResponse> {
    Json(KycHealthResponse {
        service: "janmat-kyc",
        status: "ok",
        // In production, this would check if UIDAI cert is loaded
        uidai_cert_loaded: false,
    })
}
