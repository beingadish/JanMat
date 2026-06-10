//! janmat-gateway — SMS/IVR access layer
//!
//! Provides low-bandwidth access to JanMat for citizens without smartphones.
//! An SMS or IVR session maps to the same chain operations as the web frontend.
//! Phase 6 integrates a real Twilio/IVRS provider; this crate defines the
//! trait surface that prod and mock implementations share.

pub mod ivr;
pub mod sms;

pub use ivr::{IvrSession, IvrStep};
pub use sms::{MockSmsGateway, SmsGateway, SmsMessage, SmsResponse};
