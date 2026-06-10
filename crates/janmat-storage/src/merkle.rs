//! Blake3 binary Merkle tree for ZK rollup batch anchoring.
//!
//! Batches of vote commitment hashes are Merkle-hashed into a single root.
//! This root is periodically anchored to Ethereum / Polkadot as an L1 proof.
//! Phase 6 wires the actual bridge call; this module handles the tree logic.

use blake3::Hasher;

// ── Tree ──────────────────────────────────────────────────────────────────────

/// Build a Merkle root from an ordered slice of leaf hashes (32-byte arrays).
/// Leaves are hashed in pairs; odd-length levels duplicate the last leaf.
/// An empty leaf list returns the Blake3 hash of an empty input.
pub fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return *blake3::hash(b"janmat/empty-merkle-tree").as_bytes();
    }
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    while level.len() > 1 {
        if !level.len().is_multiple_of(2) {
            level.push(*level.last().unwrap()); // duplicate last
        }
        level = level
            .chunks(2)
            .map(|pair| hash_pair(&pair[0], &pair[1]))
            .collect();
    }
    level[0]
}

fn hash_pair(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut h = Hasher::new();
    h.update(b"janmat/merkle-node");
    h.update(a);
    h.update(b);
    *h.finalize().as_bytes()
}

/// Hash a commitment string to a 32-byte leaf.
pub fn commitment_leaf(commitment_hex: &str) -> [u8; 32] {
    *blake3::hash(commitment_hex.as_bytes()).as_bytes()
}

/// Compute the Merkle root for a batch of vote commitments (hex strings).
pub fn batch_votes_root(commitments: &[String]) -> [u8; 32] {
    let leaves: Vec<[u8; 32]> = commitments.iter().map(|c| commitment_leaf(c)).collect();
    merkle_root(&leaves)
}

/// Anchor stub — in Phase 6 this submits `root` to an Ethereum/Polkadot bridge contract.
#[derive(Default)]
pub struct MerkleAnchor {
    pub anchored_roots: Vec<AnchorRecord>,
}

#[derive(Debug, Clone)]
pub struct AnchorRecord {
    pub batch_block: u64,
    pub root_hex: String,
    pub commitment_count: usize,
}


impl MerkleAnchor {
    pub fn anchor_batch(
        &mut self,
        commitments: &[String],
        batch_block: u64,
    ) -> AnchorRecord {
        let root = batch_votes_root(commitments);
        let root_hex: String = root.iter().map(|b| format!("{b:02x}")).collect();
        let record = AnchorRecord {
            batch_block,
            root_hex: root_hex.clone(),
            commitment_count: commitments.len(),
        };
        self.anchored_roots.push(record.clone());
        record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_leaf_returns_that_leaf() {
        let leaf = [1u8; 32];
        assert_eq!(merkle_root(&[leaf]), leaf);
    }

    #[test]
    fn root_is_deterministic() {
        let leaves = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        assert_eq!(merkle_root(&leaves), merkle_root(&leaves));
    }

    #[test]
    fn different_order_different_root() {
        let a = [[1u8; 32], [2u8; 32]];
        let b = [[2u8; 32], [1u8; 32]];
        assert_ne!(merkle_root(&a), merkle_root(&b));
    }

    #[test]
    fn empty_returns_known_constant() {
        let r1 = merkle_root(&[]);
        let r2 = merkle_root(&[]);
        assert_eq!(r1, r2);
        assert_ne!(r1, [0u8; 32]);
    }

    #[test]
    fn batch_votes_root_anchored_correctly() {
        let mut anchor = MerkleAnchor::default();
        let commits: Vec<String> = (0..5).map(|i| format!("commit-{i:064x}")).collect();
        let rec = anchor.anchor_batch(&commits, 1000);
        assert_eq!(rec.commitment_count, 5);
        assert_eq!(rec.batch_block, 1000);
        assert_eq!(anchor.anchored_roots.len(), 1);
    }
}
