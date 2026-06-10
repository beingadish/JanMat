use blake3::Hash;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchoredUlid {
    pub id: Ulid,
    pub anchor_hex: String,
}

impl AnchoredUlid {
    pub fn new(block_hash: &[u8]) -> Self {
        let id = Ulid::new();
        let mut payload = id.to_bytes().to_vec();
        payload.extend_from_slice(block_hash);
        let anchor_hex = hash_to_hex(blake3::hash(&payload));

        Self { id, anchor_hex }
    }
}

pub fn hash_to_hex(hash: Hash) -> String {
    hash.to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::AnchoredUlid;

    #[test]
    fn anchors_ulid_with_block_hash() {
        let anchored = AnchoredUlid::new(b"block-123");
        assert_eq!(anchored.anchor_hex.len(), 64);
    }
}
