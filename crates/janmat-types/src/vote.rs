use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VoteVisibility {
    #[default]
    Anonymous,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VoteStatus {
    #[default]
    Open,
    Closed,
}
