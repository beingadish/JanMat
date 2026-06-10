use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationFlag {
    pub opinion_id: String,
    pub reason_code: String,
}

#[derive(Default)]
pub struct ModerationPallet {
    pub flags: Vec<ModerationFlag>,
}

impl ModerationPallet {
    pub fn flag_opinion(&mut self, opinion_id: &str, reason_code: &str) {
        self.flags.push(ModerationFlag {
            opinion_id: opinion_id.to_owned(),
            reason_code: reason_code.to_owned(),
        });
    }
}
