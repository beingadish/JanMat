//! IVR session state machine for voice-based voting.
//!
//! Citizens without smartphones call a number, authenticate via OTP,
//! and navigate a menu to cast votes or listen to topic summaries.
//! Each IvrSession is a state machine that maps DTMF input to actions.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IvrStep {
    /// Greet caller and request language selection
    LanguageSelect,
    /// Request OTP for identity verification
    OtpChallenge,
    /// Main menu: 1=Browse Topics, 2=Cast Vote, 3=Profile
    MainMenu,
    /// Announce topic summary for a given topic_id
    TopicSummary { topic_id: String },
    /// Read out vote options for a given vote_id
    VoteCast { vote_id: String },
    /// Confirm the cast vote (read back choice, press 1 to confirm)
    VoteConfirm { vote_id: String, choice: String },
    /// Session complete
    Goodbye,
}

#[derive(Debug, Clone)]
pub struct IvrSession {
    pub caller_number: String,
    pub current_step: IvrStep,
    pub wallet: Option<String>,
    pub history: Vec<IvrStep>,
}

impl IvrSession {
    pub fn new(caller_number: &str) -> Self {
        Self {
            caller_number: caller_number.to_owned(),
            current_step: IvrStep::LanguageSelect,
            wallet: None,
            history: Vec::new(),
        }
    }

    /// Advance session based on DTMF input digit.
    pub fn advance(&mut self, dtmf: &str) -> &IvrStep {
        let prev = self.current_step.clone();
        self.current_step = match (&self.current_step, dtmf) {
            (IvrStep::LanguageSelect, _) => IvrStep::OtpChallenge,
            (IvrStep::OtpChallenge, otp) if otp.len() == 6 => {
                self.wallet = Some(format!("wallet-otp-{otp}"));
                IvrStep::MainMenu
            }
            (IvrStep::OtpChallenge, _) => IvrStep::OtpChallenge,
            (IvrStep::MainMenu, "1") => IvrStep::TopicSummary {
                topic_id: "t/latest".into(),
            },
            (IvrStep::MainMenu, "2") => IvrStep::VoteCast {
                vote_id: "v/latest".into(),
            },
            (IvrStep::MainMenu, "9") => IvrStep::Goodbye,
            (IvrStep::VoteCast { vote_id }, choice) => IvrStep::VoteConfirm {
                vote_id: vote_id.clone(),
                choice: choice.to_owned(),
            },
            (IvrStep::VoteConfirm { .. }, "1") => IvrStep::Goodbye,
            (IvrStep::VoteConfirm { vote_id, choice: _ }, _) => IvrStep::VoteCast {
                vote_id: vote_id.clone(),
            },
            _ => IvrStep::Goodbye,
        };
        self.history.push(prev);
        &self.current_step
    }

    pub fn is_authenticated(&self) -> bool {
        self.wallet.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_vote_flow() {
        let mut s = IvrSession::new("+911234567890");
        s.advance("1"); // select language
        s.advance("123456"); // OTP
        assert!(s.is_authenticated());
        s.advance("2"); // main menu → vote
        let step = s.advance("1"); // choose option 1 (YES)
        assert!(matches!(step, IvrStep::VoteConfirm { .. }));
        let final_step = s.advance("1"); // confirm
        assert_eq!(*final_step, IvrStep::Goodbye);
    }

    #[test]
    fn wrong_otp_stays_at_challenge() {
        let mut s = IvrSession::new("+91x");
        s.advance("1");
        let step = s.advance("123"); // too short — not 6 digits
        assert_eq!(*step, IvrStep::OtpChallenge);
        assert!(!s.is_authenticated());
    }

    #[test]
    fn history_records_visited_steps() {
        let mut s = IvrSession::new("+91x");
        s.advance("1");
        s.advance("123456");
        assert!(s.history.len() >= 2);
    }
}
