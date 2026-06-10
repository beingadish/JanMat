pub fn passes_quorum(yes_votes: u64, total_votes: u64, required_ratio: f64) -> bool {
    if total_votes == 0 {
        return false;
    }
    (yes_votes as f64 / total_votes as f64) >= required_ratio
}

#[cfg(test)]
mod tests {
    use super::passes_quorum;

    #[test]
    fn enforces_supermajority() {
        assert!(!passes_quorum(65, 100, 0.67));
        assert!(passes_quorum(70, 100, 0.67));
    }
}
