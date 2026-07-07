//! Internationalisation for JanMat — 22 Indian languages + English.
//!
//! Phase 6 ships with Hindi and English strings seeded.
//! Other languages are loaded from JSON bundles at runtime (see `bundles/`).
//! The Leptos UI components call `t(lang, key)` — never hardcode strings.

pub mod keys;
pub use keys::TranslationKey;

use std::collections::HashMap;

// ── Supported languages ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Hindi,
    Bengali,
    Telugu,
    Marathi,
    Tamil,
    Urdu,
    Gujarati,
    Kannada,
    Malayalam,
    Odia,
    Punjabi,
    Assamese,
    Maithili,
    Santali,
    Kashmiri,
    Nepali,
    Sindhi,
    Konkani,
    Manipuri,
    Dogri,
    Bodo,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::English,
        Language::Hindi,
        Language::Bengali,
        Language::Telugu,
        Language::Marathi,
        Language::Tamil,
        Language::Urdu,
        Language::Gujarati,
        Language::Kannada,
        Language::Malayalam,
        Language::Odia,
        Language::Punjabi,
        Language::Assamese,
        Language::Maithili,
        Language::Santali,
        Language::Kashmiri,
        Language::Nepali,
        Language::Sindhi,
        Language::Konkani,
        Language::Manipuri,
        Language::Dogri,
        Language::Bodo,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Hindi => "hi",
            Language::Bengali => "bn",
            Language::Telugu => "te",
            Language::Marathi => "mr",
            Language::Tamil => "ta",
            Language::Urdu => "ur",
            Language::Gujarati => "gu",
            Language::Kannada => "kn",
            Language::Malayalam => "ml",
            Language::Odia => "or",
            Language::Punjabi => "pa",
            Language::Assamese => "as",
            Language::Maithili => "mai",
            Language::Santali => "sat",
            Language::Kashmiri => "ks",
            Language::Nepali => "ne",
            Language::Sindhi => "sd",
            Language::Konkani => "kok",
            Language::Manipuri => "mni",
            Language::Dogri => "doi",
            Language::Bodo => "brx",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Hindi => "हिन्दी",
            Language::Bengali => "বাংলা",
            Language::Telugu => "తెలుగు",
            Language::Marathi => "मराठी",
            Language::Tamil => "தமிழ்",
            Language::Urdu => "اردو",
            Language::Gujarati => "ગુજરાતી",
            Language::Kannada => "ಕನ್ನಡ",
            Language::Malayalam => "മലയാളം",
            Language::Odia => "ଓଡ଼ିଆ",
            Language::Punjabi => "ਪੰਜਾਬੀ",
            Language::Assamese => "অসমীয়া",
            Language::Maithili => "मैथिली",
            Language::Santali => "ᱥᱟᱱᱛᱟᱲᱤ",
            Language::Kashmiri => "کٲشُر",
            Language::Nepali => "नेपाली",
            Language::Sindhi => "سنڌي",
            Language::Konkani => "कोंकणी",
            Language::Manipuri => "মৈতৈলোন্",
            Language::Dogri => "डोगरी",
            Language::Bodo => "बर'/बड़",
        }
    }
}

// ── Translation store ─────────────────────────────────────────────────────────

pub struct TranslationStore {
    strings: HashMap<(Language, TranslationKey), String>,
}

impl TranslationStore {
    /// Build from seeded strings. Phase 6 extends this from JSON bundles.
    pub fn with_seeds() -> Self {
        let mut s = Self {
            strings: HashMap::new(),
        };
        s.seed();
        s
    }

    pub fn get(&self, lang: Language, key: TranslationKey) -> &str {
        self.strings
            .get(&(lang, key))
            .or_else(|| self.strings.get(&(Language::English, key)))
            .map(|s| s.as_str())
            .unwrap_or("[missing]")
    }

    fn insert(&mut self, lang: Language, key: TranslationKey, val: &str) {
        self.strings.insert((lang, key), val.to_owned());
    }

    fn seed(&mut self) {
        use TranslationKey::*;
        // English
        self.insert(Language::English, AppTitle, "JanMat — People's Opinion");
        self.insert(Language::English, NavTopics, "Topics");
        self.insert(Language::English, NavVote, "Vote");
        self.insert(Language::English, NavProfile, "Profile");
        self.insert(Language::English, VoteYes, "Yes");
        self.insert(Language::English, VoteNo, "No");
        self.insert(Language::English, VoteAbstain, "Abstain");
        self.insert(Language::English, SubmitOpinion, "Submit Opinion");
        self.insert(Language::English, OnboardTitle, "Verify your identity");
        self.insert(
            Language::English,
            OnboardSubtitle,
            "Your Aadhaar number never leaves your device",
        );
        self.insert(
            Language::English,
            ErrorBlocked,
            "This content cannot be submitted",
        );
        self.insert(
            Language::English,
            WarnClone,
            "Similar opinion exists. Acknowledge to continue.",
        );
        self.insert(Language::English, ConstitutionTitle, "Constitution");
        // English - KYC Upload
        self.insert(
            Language::English,
            KycUploadTitle,
            "Upload UIDAI Offline e-KYC",
        );
        self.insert(
            Language::English,
            KycUploadInstructions,
            "Download your e-KYC ZIP from resident.uidai.gov.in",
        );
        self.insert(Language::English, KycSelectFile, "Select ZIP File");
        self.insert(Language::English, KycShareCodeLabel, "4-Digit Share Code");
        self.insert(
            Language::English,
            KycShareCodePlaceholder,
            "Enter your share code",
        );
        self.insert(Language::English, KycVerifyButton, "Verify Identity");
        self.insert(Language::English, KycVerifying, "Verifying...");
        self.insert(
            Language::English,
            KycSuccess,
            "Identity verified successfully!",
        );
        self.insert(
            Language::English,
            KycErrorInvalidFile,
            "Invalid ZIP file. Please download from UIDAI.",
        );
        self.insert(
            Language::English,
            KycErrorWrongCode,
            "Wrong share code. Please try again.",
        );
        self.insert(
            Language::English,
            KycErrorNetwork,
            "Network error. Please try again.",
        );
        self.insert(
            Language::English,
            KycPrivacyNote,
            "Your Aadhaar number never leaves your device.",
        );
        // Hindi
        self.insert(Language::Hindi, AppTitle, "जनमत — लोगों की राय");
        self.insert(Language::Hindi, NavTopics, "विषय");
        self.insert(Language::Hindi, NavVote, "मतदान");
        self.insert(Language::Hindi, NavProfile, "प्रोफ़ाइल");
        self.insert(Language::Hindi, VoteYes, "हाँ");
        self.insert(Language::Hindi, VoteNo, "नहीं");
        self.insert(Language::Hindi, VoteAbstain, "अनुपस्थित");
        self.insert(Language::Hindi, SubmitOpinion, "राय दर्ज करें");
        self.insert(Language::Hindi, OnboardTitle, "अपनी पहचान सत्यापित करें");
        self.insert(
            Language::Hindi,
            OnboardSubtitle,
            "आपका आधार नंबर आपके डिवाइस से बाहर नहीं जाता",
        );
        self.insert(
            Language::Hindi,
            ErrorBlocked,
            "यह सामग्री सबमिट नहीं की जा सकती",
        );
        self.insert(
            Language::Hindi,
            WarnClone,
            "समान राय पहले से मौजूद है। जारी रखने के लिए स्वीकार करें।",
        );
        self.insert(Language::Hindi, ConstitutionTitle, "संविधान");
        // Hindi - KYC Upload
        self.insert(
            Language::Hindi,
            KycUploadTitle,
            "UIDAI ऑफ़लाइन e-KYC अपलोड करें",
        );
        self.insert(
            Language::Hindi,
            KycUploadInstructions,
            "resident.uidai.gov.in से अपना e-KYC ZIP डाउनलोड करें",
        );
        self.insert(Language::Hindi, KycSelectFile, "ZIP फ़ाइल चुनें");
        self.insert(Language::Hindi, KycShareCodeLabel, "4-अंकीय शेयर कोड");
        self.insert(
            Language::Hindi,
            KycShareCodePlaceholder,
            "अपना शेयर कोड दर्ज करें",
        );
        self.insert(Language::Hindi, KycVerifyButton, "पहचान सत्यापित करें");
        self.insert(Language::Hindi, KycVerifying, "सत्यापन हो रहा है...");
        self.insert(Language::Hindi, KycSuccess, "पहचान सफलतापूर्वक सत्यापित!");
        self.insert(
            Language::Hindi,
            KycErrorInvalidFile,
            "अमान्य ZIP फ़ाइल। कृपया UIDAI से डाउनलोड करें।",
        );
        self.insert(
            Language::Hindi,
            KycErrorWrongCode,
            "गलत शेयर कोड। कृपया पुनः प्रयास करें।",
        );
        self.insert(
            Language::Hindi,
            KycErrorNetwork,
            "नेटवर्क त्रुटि। कृपया पुनः प्रयास करें।",
        );
        self.insert(
            Language::Hindi,
            KycPrivacyNote,
            "आपका आधार नंबर आपके डिवाइस से बाहर नहीं जाता।",
        );
    }
}

/// Convenience function: translate key to string for given language.
pub fn t(store: &TranslationStore, lang: Language, key: TranslationKey) -> &str {
    store.get(lang, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_22_languages_defined() {
        assert_eq!(Language::ALL.len(), 22);
    }

    #[test]
    fn every_language_has_unique_code() {
        let codes: std::collections::HashSet<_> = Language::ALL.iter().map(|l| l.code()).collect();
        assert_eq!(codes.len(), Language::ALL.len());
    }

    #[test]
    fn english_fallback_for_unseeded_language() {
        let store = TranslationStore::with_seeds();
        // Bengali is not seeded — should fall back to English
        let val = store.get(Language::Bengali, TranslationKey::AppTitle);
        assert_eq!(val, "JanMat — People's Opinion");
    }

    #[test]
    fn hindi_translation_returned() {
        let store = TranslationStore::with_seeds();
        let val = store.get(Language::Hindi, TranslationKey::AppTitle);
        assert_eq!(val, "जनमत — लोगों की राय");
    }

    #[test]
    fn missing_key_returns_placeholder() {
        let store = TranslationStore {
            strings: std::collections::HashMap::new(),
        };
        let val = store.get(Language::English, TranslationKey::AppTitle);
        assert_eq!(val, "[missing]");
    }
}
