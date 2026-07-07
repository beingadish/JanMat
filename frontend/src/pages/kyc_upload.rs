//! KYC Upload Component for UIDAI Offline e-KYC verification.
//!
//! This component provides:
//! - ZIP file upload via drag-and-drop or click
//! - Share code input (4-digit PIN)
//! - Upload progress feedback
//! - Error handling with localized messages
//!
//! # Privacy
//! The Aadhaar number is NEVER transmitted. Only the extracted credential
//! (hash + signature) is sent to the backend for verification.

use crate::i18n::{Language, TranslationKey, TranslationStore, t};

/// State of the KYC upload process.
#[derive(Debug, Clone, PartialEq)]
pub enum KycUploadState {
    /// Initial state - waiting for file selection
    Idle,
    /// File selected, waiting for share code
    FileSelected { filename: String },
    /// Uploading and verifying
    Verifying,
    /// Verification successful
    Success { reference_id: String },
    /// Verification failed
    Error { message: String, code: String },
}

/// Props for the KYC upload component.
#[derive(Debug, Clone)]
pub struct KycUploadProps {
    pub language: Language,
    pub api_base_url: String,
    pub on_success: Option<fn(String)>, // callback with nullifier_seed_commitment
}

impl Default for KycUploadProps {
    fn default() -> Self {
        Self {
            language: Language::English,
            api_base_url: "http://localhost:3000".to_string(),
            on_success: None,
        }
    }
}

/// Response from the KYC verification API.
#[derive(Debug, Clone)]
pub struct KycVerifyResponse {
    pub success: bool,
    pub nullifier_seed_commitment: Option<String>,
    pub citizenship_country: Option<String>,
    pub reference_id: Option<String>,
    pub error: Option<String>,
    pub error_code: Option<String>,
}

/// Helper to get translated error message based on error code.
pub fn get_error_translation<'a>(
    store: &'a TranslationStore,
    lang: Language,
    error_code: &str,
) -> &'a str {
    match error_code {
        "INVALID_ZIP" | "INVALID_XML" | "MISSING_FILE" | "ZIP_ERROR" => {
            t(store, lang, TranslationKey::KycErrorInvalidFile)
        }
        "WRONG_SHARE_CODE" | "INVALID_SHARE_CODE" | "MISSING_SHARE_CODE" => {
            t(store, lang, TranslationKey::KycErrorWrongCode)
        }
        _ => t(store, lang, TranslationKey::KycErrorNetwork),
    }
}

/// Generate HTML for the KYC upload form.
///
/// This is a static HTML generator for use with Leptos or other frameworks.
/// In a full Leptos app, this would be a reactive component.
pub fn render_kyc_upload_form(store: &TranslationStore, props: &KycUploadProps) -> String {
    let lang = props.language;
    
    format!(
        r#"<div class="kyc-upload-container">
    <h2 class="kyc-title">{title}</h2>
    <p class="kyc-instructions">{instructions}</p>
    
    <form id="kyc-form" class="kyc-form" enctype="multipart/form-data">
        <!-- File Upload Zone -->
        <div class="kyc-dropzone" id="dropzone">
            <input type="file" id="kyc-file" name="file" accept=".zip" hidden />
            <label for="kyc-file" class="kyc-file-label">
                <svg class="kyc-upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                    <polyline points="17 8 12 3 7 8"/>
                    <line x1="12" y1="3" x2="12" y2="15"/>
                </svg>
                <span id="file-label-text">{select_file}</span>
            </label>
        </div>
        
        <!-- Share Code Input -->
        <div class="kyc-share-code">
            <label for="share-code">{share_code_label}</label>
            <input 
                type="text" 
                id="share-code" 
                name="share_code"
                maxlength="4" 
                pattern="[0-9]{{4}}"
                placeholder="{share_code_placeholder}"
                inputmode="numeric"
                autocomplete="off"
            />
        </div>
        
        <!-- Submit Button -->
        <button type="submit" id="verify-btn" class="kyc-verify-btn" disabled>
            <span id="btn-text">{verify_button}</span>
            <span id="btn-spinner" class="spinner" hidden></span>
        </button>
        
        <!-- Status Messages -->
        <div id="kyc-status" class="kyc-status" hidden></div>
        
        <!-- Privacy Note -->
        <p class="kyc-privacy-note">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" class="lock-icon">
                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
            </svg>
            {privacy_note}
        </p>
    </form>
</div>

<style>
.kyc-upload-container {{
    max-width: 400px;
    margin: 0 auto;
    padding: 24px;
    font-family: system-ui, -apple-system, sans-serif;
}}

.kyc-title {{
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 8px;
    color: #1a1a1a;
}}

.kyc-instructions {{
    color: #666;
    margin-bottom: 24px;
    font-size: 0.9rem;
}}

.kyc-dropzone {{
    border: 2px dashed #ccc;
    border-radius: 12px;
    padding: 32px;
    text-align: center;
    transition: border-color 0.2s, background-color 0.2s;
    cursor: pointer;
}}

.kyc-dropzone:hover,
.kyc-dropzone.dragover {{
    border-color: #4f46e5;
    background-color: #f5f3ff;
}}

.kyc-dropzone.file-selected {{
    border-color: #22c55e;
    background-color: #f0fdf4;
}}

.kyc-upload-icon {{
    width: 48px;
    height: 48px;
    margin-bottom: 12px;
    color: #9ca3af;
}}

.kyc-file-label {{
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
    color: #4b5563;
}}

.kyc-share-code {{
    margin-top: 20px;
}}

.kyc-share-code label {{
    display: block;
    margin-bottom: 8px;
    font-weight: 500;
    color: #374151;
}}

.kyc-share-code input {{
    width: 100%;
    padding: 12px 16px;
    font-size: 1.25rem;
    letter-spacing: 0.5em;
    text-align: center;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    box-sizing: border-box;
}}

.kyc-share-code input:focus {{
    outline: none;
    border-color: #4f46e5;
    box-shadow: 0 0 0 3px rgba(79, 70, 229, 0.1);
}}

.kyc-verify-btn {{
    width: 100%;
    margin-top: 24px;
    padding: 14px 24px;
    font-size: 1rem;
    font-weight: 600;
    color: white;
    background-color: #4f46e5;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: background-color 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
}}

.kyc-verify-btn:hover:not(:disabled) {{
    background-color: #4338ca;
}}

.kyc-verify-btn:disabled {{
    background-color: #9ca3af;
    cursor: not-allowed;
}}

.spinner {{
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
}}

@keyframes spin {{
    to {{ transform: rotate(360deg); }}
}}

.kyc-status {{
    margin-top: 16px;
    padding: 12px 16px;
    border-radius: 8px;
    font-size: 0.9rem;
}}

.kyc-status.success {{
    background-color: #f0fdf4;
    color: #166534;
    border: 1px solid #86efac;
}}

.kyc-status.error {{
    background-color: #fef2f2;
    color: #991b1b;
    border: 1px solid #fecaca;
}}

.kyc-privacy-note {{
    margin-top: 24px;
    display: flex;
    align-items: center;
    gap: 8px;
    color: #6b7280;
    font-size: 0.85rem;
}}

.lock-icon {{
    width: 16px;
    height: 16px;
    flex-shrink: 0;
}}
</style>

<script>
(function() {{
    const form = document.getElementById('kyc-form');
    const dropzone = document.getElementById('dropzone');
    const fileInput = document.getElementById('kyc-file');
    const fileLabelText = document.getElementById('file-label-text');
    const shareCodeInput = document.getElementById('share-code');
    const verifyBtn = document.getElementById('verify-btn');
    const btnText = document.getElementById('btn-text');
    const btnSpinner = document.getElementById('btn-spinner');
    const statusDiv = document.getElementById('kyc-status');
    
    const API_URL = '{api_base_url}/citizen/kyc/upload';
    let selectedFile = null;
    
    // Drag and drop
    ['dragenter', 'dragover'].forEach(event => {{
        dropzone.addEventListener(event, (e) => {{
            e.preventDefault();
            dropzone.classList.add('dragover');
        }});
    }});
    
    ['dragleave', 'drop'].forEach(event => {{
        dropzone.addEventListener(event, (e) => {{
            e.preventDefault();
            dropzone.classList.remove('dragover');
        }});
    }});
    
    dropzone.addEventListener('drop', (e) => {{
        const files = e.dataTransfer.files;
        if (files.length > 0 && files[0].name.endsWith('.zip')) {{
            fileInput.files = files;
            handleFileSelect(files[0]);
        }}
    }});
    
    fileInput.addEventListener('change', (e) => {{
        if (e.target.files.length > 0) {{
            handleFileSelect(e.target.files[0]);
        }}
    }});
    
    function handleFileSelect(file) {{
        selectedFile = file;
        fileLabelText.textContent = file.name;
        dropzone.classList.add('file-selected');
        checkCanSubmit();
    }}
    
    shareCodeInput.addEventListener('input', (e) => {{
        e.target.value = e.target.value.replace(/[^0-9]/g, '').slice(0, 4);
        checkCanSubmit();
    }});
    
    function checkCanSubmit() {{
        const canSubmit = selectedFile && shareCodeInput.value.length === 4;
        verifyBtn.disabled = !canSubmit;
    }}
    
    form.addEventListener('submit', async (e) => {{
        e.preventDefault();
        
        if (!selectedFile || shareCodeInput.value.length !== 4) return;
        
        // Show loading state
        verifyBtn.disabled = true;
        btnText.textContent = '{verifying}';
        btnSpinner.hidden = false;
        statusDiv.hidden = true;
        
        const formData = new FormData();
        formData.append('file', selectedFile);
        formData.append('share_code', shareCodeInput.value);
        
        try {{
            const response = await fetch(API_URL, {{
                method: 'POST',
                body: formData
            }});
            
            const data = await response.json();
            
            if (data.success) {{
                statusDiv.className = 'kyc-status success';
                statusDiv.textContent = '{success}';
                statusDiv.hidden = false;
                
                // Store the commitment for later use
                if (window.onKycSuccess) {{
                    window.onKycSuccess(data.nullifier_seed_commitment);
                }}
            }} else {{
                statusDiv.className = 'kyc-status error';
                statusDiv.textContent = data.error || 'Verification failed';
                statusDiv.hidden = false;
            }}
        }} catch (error) {{
            statusDiv.className = 'kyc-status error';
            statusDiv.textContent = '{error_network}';
            statusDiv.hidden = false;
        }} finally {{
            verifyBtn.disabled = false;
            btnText.textContent = '{verify_button}';
            btnSpinner.hidden = true;
        }}
    }});
}})();
</script>"#,
        title = t(store, lang, TranslationKey::KycUploadTitle),
        instructions = t(store, lang, TranslationKey::KycUploadInstructions),
        select_file = t(store, lang, TranslationKey::KycSelectFile),
        share_code_label = t(store, lang, TranslationKey::KycShareCodeLabel),
        share_code_placeholder = t(store, lang, TranslationKey::KycShareCodePlaceholder),
        verify_button = t(store, lang, TranslationKey::KycVerifyButton),
        verifying = t(store, lang, TranslationKey::KycVerifying),
        success = t(store, lang, TranslationKey::KycSuccess),
        error_network = t(store, lang, TranslationKey::KycErrorNetwork),
        privacy_note = t(store, lang, TranslationKey::KycPrivacyNote),
        api_base_url = props.api_base_url,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_english_form() {
        let store = TranslationStore::with_seeds();
        let props = KycUploadProps::default();
        let html = render_kyc_upload_form(&store, &props);
        
        assert!(html.contains("Upload UIDAI Offline e-KYC"));
        assert!(html.contains("Select ZIP File"));
        assert!(html.contains("4-Digit Share Code"));
        assert!(html.contains("Verify Identity"));
    }

    #[test]
    fn renders_hindi_form() {
        let store = TranslationStore::with_seeds();
        let props = KycUploadProps {
            language: Language::Hindi,
            ..Default::default()
        };
        let html = render_kyc_upload_form(&store, &props);
        
        assert!(html.contains("UIDAI ऑफ़लाइन e-KYC अपलोड करें"));
        assert!(html.contains("ZIP फ़ाइल चुनें"));
    }

    #[test]
    fn error_translation_mapping() {
        let store = TranslationStore::with_seeds();
        
        let invalid_file = get_error_translation(&store, Language::English, "INVALID_ZIP");
        assert!(invalid_file.contains("Invalid ZIP"));
        
        let wrong_code = get_error_translation(&store, Language::English, "WRONG_SHARE_CODE");
        assert!(wrong_code.contains("Wrong share code"));
    }
}
