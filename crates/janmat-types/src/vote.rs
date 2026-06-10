use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteVisibility {
    Anonymous,
    Public,
}

impl Default for VoteVisibility {
    fn default() -> Self {
        Self::Anonymous
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteStatus {
    Open,
    Closed,
}

impl Default for VoteStatus {
    fn default() -> Self {
        Self::Open
    }
}
