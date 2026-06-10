//! Per-wallet rate limiting middleware.
//!
//! Limits defined in the JanMat constitution:
//!   - Topic creation:  1  per 24h per wallet
//!   - Opinion submit:  5  per 24h per wallet
//!   - Vote cast:       unlimited (one per v/ enforced on-chain by nullifier)
//!
//! Phase 6: wire to Redis or PostgreSQL for distributed rate limit state.
//! Phase 5 and below: InMemoryRateLimitStore for single-process dev/test.

#![allow(dead_code)]

use std::collections::HashMap;

pub const TOPIC_LIMIT_PER_DAY: u32  = 1;
pub const OPINION_LIMIT_PER_DAY: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RateLimitAction {
    CreateTopic,
    SubmitOpinion,
    CastVote,
}

pub trait RateLimitStore {
    /// Returns `true` if the action is allowed for this wallet, `false` if limited.
    /// Increments the counter if allowed.
    fn check_and_increment(&mut self, wallet: &str, action: RateLimitAction) -> bool;
    fn reset(&mut self, wallet: &str);
}

// ── In-memory store ───────────────────────────────────────────────────────────

#[derive(Default)]
pub struct InMemoryRateLimitStore {
    /// wallet → (action → count)
    counts: HashMap<String, HashMap<RateLimitAction, u32>>,
}

impl RateLimitStore for InMemoryRateLimitStore {
    fn check_and_increment(&mut self, wallet: &str, action: RateLimitAction) -> bool {
        let limit = match action {
            RateLimitAction::CreateTopic    => TOPIC_LIMIT_PER_DAY,
            RateLimitAction::SubmitOpinion  => OPINION_LIMIT_PER_DAY,
            RateLimitAction::CastVote       => u32::MAX, // unlimited
        };
        let entry = self.counts
            .entry(wallet.to_owned())
            .or_default()
            .entry(action)
            .or_insert(0);
        if *entry >= limit {
            return false;
        }
        *entry += 1;
        true
    }

    fn reset(&mut self, wallet: &str) {
        self.counts.remove(wallet);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_limit_enforced() {
        let mut store = InMemoryRateLimitStore::default();
        assert!(store.check_and_increment("wallet-A", RateLimitAction::CreateTopic));
        assert!(!store.check_and_increment("wallet-A", RateLimitAction::CreateTopic));
    }

    #[test]
    fn opinion_limit_allows_5_then_blocks() {
        let mut store = InMemoryRateLimitStore::default();
        for _ in 0..5 {
            assert!(store.check_and_increment("wallet-A", RateLimitAction::SubmitOpinion));
        }
        assert!(!store.check_and_increment("wallet-A", RateLimitAction::SubmitOpinion));
    }

    #[test]
    fn vote_cast_is_unlimited() {
        let mut store = InMemoryRateLimitStore::default();
        for _ in 0..100 {
            assert!(store.check_and_increment("wallet-A", RateLimitAction::CastVote));
        }
    }

    #[test]
    fn different_wallets_tracked_independently() {
        let mut store = InMemoryRateLimitStore::default();
        assert!(store.check_and_increment("wallet-A", RateLimitAction::CreateTopic));
        assert!(!store.check_and_increment("wallet-A", RateLimitAction::CreateTopic));
        // wallet-B is unaffected
        assert!(store.check_and_increment("wallet-B", RateLimitAction::CreateTopic));
    }

    #[test]
    fn reset_clears_counts() {
        let mut store = InMemoryRateLimitStore::default();
        store.check_and_increment("wallet-A", RateLimitAction::CreateTopic);
        store.reset("wallet-A");
        assert!(store.check_and_increment("wallet-A", RateLimitAction::CreateTopic));
    }
}
