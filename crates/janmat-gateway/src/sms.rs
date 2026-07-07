use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsMessage {
    pub to: String,
    pub body: String,
    pub from: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmsResponse {
    Sent { message_id: String },
    Failed { reason: String },
}

/// Trait implemented by Twilio (prod) and MockSmsGateway (test/dev).
pub trait SmsGateway {
    fn send(&self, msg: SmsMessage) -> SmsResponse;
}

// ── Mock ──────────────────────────────────────────────────────────────────────

use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct MockSmsGateway {
    pub sent: Arc<Mutex<Vec<SmsMessage>>>,
}

impl SmsGateway for MockSmsGateway {
    fn send(&self, msg: SmsMessage) -> SmsResponse {
        let id = format!("mock-{}", self.sent.lock().unwrap().len() + 1);
        self.sent.lock().unwrap().push(msg);
        SmsResponse::Sent { message_id: id }
    }
}

impl MockSmsGateway {
    pub fn message_count(&self) -> usize {
        self.sent.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_records_sent_messages() {
        let gw = MockSmsGateway::default();
        gw.send(SmsMessage {
            to: "+911234567890".into(),
            body: "Your OTP: 123456".into(),
            from: "JANMAT".into(),
        });
        gw.send(SmsMessage {
            to: "+911234567891".into(),
            body: "Vote recorded.".into(),
            from: "JANMAT".into(),
        });
        assert_eq!(gw.message_count(), 2);
    }

    #[test]
    fn mock_returns_incremental_message_ids() {
        let gw = MockSmsGateway::default();
        let r1 = gw.send(SmsMessage {
            to: "+91x".into(),
            body: "a".into(),
            from: "J".into(),
        });
        let r2 = gw.send(SmsMessage {
            to: "+91y".into(),
            body: "b".into(),
            from: "J".into(),
        });
        assert_eq!(
            r1,
            SmsResponse::Sent {
                message_id: "mock-1".into()
            }
        );
        assert_eq!(
            r2,
            SmsResponse::Sent {
                message_id: "mock-2".into()
            }
        );
    }
}
