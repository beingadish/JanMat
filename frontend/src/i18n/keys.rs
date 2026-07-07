//! All translatable string keys used in the JanMat frontend.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    AppTitle,
    NavTopics,
    NavVote,
    NavProfile,
    VoteYes,
    VoteNo,
    VoteAbstain,
    SubmitOpinion,
    OnboardTitle,
    OnboardSubtitle,
    ErrorBlocked,
    WarnClone,
    ConstitutionTitle,
    // KYC Upload keys
    KycUploadTitle,
    KycUploadInstructions,
    KycSelectFile,
    KycShareCodeLabel,
    KycShareCodePlaceholder,
    KycVerifyButton,
    KycVerifying,
    KycSuccess,
    KycErrorInvalidFile,
    KycErrorWrongCode,
    KycErrorNetwork,
    KycPrivacyNote,
}
