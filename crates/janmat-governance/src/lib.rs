pub const REGULAR_QUORUM: f64 = 0.5;
pub const CONSTITUTIONAL_QUORUM: f64 = 0.67;

pub fn met_quorum(yes_votes: u64, total_votes: u64, constitutional: bool) -> bool {
    let target = if constitutional {
        CONSTITUTIONAL_QUORUM
    } else {
        REGULAR_QUORUM
    };
    if total_votes == 0 {
        return false;
    }
    (yes_votes as f64 / total_votes as f64) >= target
}
