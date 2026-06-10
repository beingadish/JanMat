use blake3::Hasher;

pub trait IpfsClient {
    fn put(&self, content: &[u8]) -> IpfsObjectRef;
    fn get(&self, cid: &str) -> Option<Vec<u8>>;
}

#[derive(Debug, Clone)]
pub struct IpfsObjectRef {
    pub cid: String,
    /// Blake3 hex hash of content — matches the value stored on-chain
    pub content_hash: String,
}

pub fn content_hash(bytes: &[u8]) -> String {
    let mut h = Hasher::new();
    h.update(bytes);
    h.finalize().to_hex().to_string()
}

pub fn verify_content_hash(stored_hash: &str, content: &[u8]) -> bool {
    content_hash(content) == stored_hash
}

// ── Mock implementation (tests + Phase 1 dev) ─────────────────────────────────

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct MockIpfsClient {
    store: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    counter: Arc<Mutex<u64>>,
}

impl IpfsClient for MockIpfsClient {
    fn put(&self, content: &[u8]) -> IpfsObjectRef {
        let hash = content_hash(content);
        let mut ctr = self.counter.lock().unwrap();
        *ctr += 1;
        let cid = format!("QmMock{ctr:08}");
        self.store.lock().unwrap().insert(cid.clone(), content.to_vec());
        IpfsObjectRef {
            cid,
            content_hash: hash,
        }
    }

    fn get(&self, cid: &str) -> Option<Vec<u8>> {
        self.store.lock().unwrap().get(cid).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_hash_verification() {
        let client = MockIpfsClient::default();
        let text = b"NEP reform is crucial for rural India";
        let obj = client.put(text);
        let retrieved = client.get(&obj.cid).unwrap();
        assert!(verify_content_hash(&obj.content_hash, &retrieved));
    }

    #[test]
    fn tampered_content_fails_verification() {
        let client = MockIpfsClient::default();
        let obj = client.put(b"original content");
        assert!(!verify_content_hash(&obj.content_hash, b"tampered content"));
    }
}
