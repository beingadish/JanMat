//! POC login-password codec and the rolling Citizen-UID seed chain.
//!
//! The password is a 16-character, URL-safe token: a 96-bit block
//!   userId:u16 ‖ creationTime:u32(secs) ‖ fingerprint:48-bit
//! encrypted with a keyed Feistel network. The fingerprint is the first 6 bytes
//! of `identity_root = H(PAN | Aadhaar | DOB | username)`, so the password is
//! cryptographically bound to ALL four identity fields — logging in with the
//! password proves knowledge of every field, not just the username. Because the
//! Feistel key is a server secret, tokens are unguessable/unforgeable, yet the
//! server reverses them to recover the user, the creation time (expiry =
//! creation + 24h is carried by the password), and the fingerprint to match.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

const ROUNDS: u8 = 6;
const MASK48: u64 = 0x0000_FFFF_FFFF_FFFF;

fn be6(b: &[u8]) -> u64 {
    b.iter().fold(0u64, |acc, &c| (acc << 8) | c as u64)
}
fn to_be6(x: u64) -> [u8; 6] {
    let b = x.to_be_bytes();
    [b[2], b[3], b[4], b[5], b[6], b[7]]
}

fn round_f(key: &[u8], round: u8, half: u64) -> u64 {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/pwtoken/v2");
    h.update(key);
    h.update(&[round]);
    h.update(&half.to_be_bytes());
    be6(&h.finalize().as_bytes()[0..6]) & MASK48
}

fn feistel_enc(mut l: u64, mut r: u64, key: &[u8]) -> (u64, u64) {
    for i in 0..ROUNDS {
        let nr = l ^ round_f(key, i, r);
        l = r;
        r = nr & MASK48;
    }
    (l, r)
}
fn feistel_dec(mut l: u64, mut r: u64, key: &[u8]) -> (u64, u64) {
    for i in (0..ROUNDS).rev() {
        let nl = r ^ round_f(key, i, l);
        r = l;
        l = nl & MASK48;
    }
    (l, r)
}

/// Encode (userId, creation-seconds, 6-byte fingerprint) into a 16-char password.
pub fn encode_password(id: u16, secs: u32, fp6: &[u8; 6], key: &[u8]) -> String {
    let mut payload = [0u8; 12];
    payload[0..2].copy_from_slice(&id.to_be_bytes());
    payload[2..6].copy_from_slice(&secs.to_be_bytes());
    payload[6..12].copy_from_slice(fp6);
    let (l, r) = feistel_enc(be6(&payload[0..6]), be6(&payload[6..12]), key);
    let mut out = [0u8; 12];
    out[0..6].copy_from_slice(&to_be6(l));
    out[6..12].copy_from_slice(&to_be6(r));
    URL_SAFE_NO_PAD.encode(out) // 12 bytes → 16 chars
}

/// Reverse a password back to (userId, creation-seconds, fingerprint).
pub fn decode_password(token: &str, key: &[u8]) -> Option<(u16, u32, [u8; 6])> {
    let raw = URL_SAFE_NO_PAD.decode(token.as_bytes()).ok()?;
    if raw.len() != 12 {
        return None;
    }
    let (l, r) = feistel_dec(be6(&raw[0..6]), be6(&raw[6..12]), key);
    let mut p = [0u8; 12];
    p[0..6].copy_from_slice(&to_be6(l));
    p[6..12].copy_from_slice(&to_be6(r));
    let id = u16::from_be_bytes([p[0], p[1]]);
    let secs = u32::from_be_bytes([p[2], p[3], p[4], p[5]]);
    let mut fp = [0u8; 6];
    fp.copy_from_slice(&p[6..12]);
    Some((id, secs, fp))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Deterministic fingerprint of the four identity fields — the seed-chain root.
pub fn identity_root(pan: &str, aadhaar: &str, dob: &str, username: &str) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/identity-root/v1");
    for part in [pan.trim().to_uppercase().as_str(), aadhaar.trim(), dob.trim(), username.trim()] {
        h.update(part.as_bytes());
        h.update(b"|");
    }
    *h.finalize().as_bytes()
}

pub fn root_hex(root: &[u8; 32]) -> String {
    hex(root)
}

/// The 6-byte fingerprint embedded in the password (bytes 0..6 of the root).
pub fn fingerprint6(root: &[u8; 32]) -> [u8; 6] {
    let mut fp = [0u8; 6];
    fp.copy_from_slice(&root[0..6]);
    fp
}

/// Genesis Citizen-UID seed (login #0) anchored at the identity root.
pub fn seed_genesis(root_hex: &str) -> String {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/seed/v1");
    h.update(root_hex.as_bytes());
    h.update(&0u64.to_be_bytes());
    hex(h.finalize().as_bytes())
}

/// Next seed in the hash chain: seed_n = H(prev_seed ‖ n). Tamper-evident.
pub fn seed_next(prev_hex: &str, n: u64) -> String {
    let mut h = blake3::Hasher::new();
    h.update(b"janmat/seed/v1");
    h.update(prev_hex.as_bytes());
    h.update(&n.to_be_bytes());
    hex(h.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_roundtrips_and_is_16_chars() {
        let key = b"test-secret";
        let fp = [1u8, 2, 3, 4, 5, 6];
        for &(id, secs) in &[(1u16, 1_700_000_000u32), (65535, 0), (42, 2_000_000_000)] {
            let p = encode_password(id, secs, &fp, key);
            assert_eq!(p.len(), 16, "password must be 16 chars, got {p}");
            assert_eq!(decode_password(&p, key), Some((id, secs, fp)));
        }
    }

    #[test]
    fn fingerprint_binds_all_four_fields() {
        let a = identity_root("ABCDE1234F", "123456789012", "1998-04-12", "rohan_k");
        let b = identity_root("ABCDE1234F", "123456789012", "1998-04-12", "rohan_x"); // diff username
        let c = identity_root("ABCDE1234F", "999999999999", "1998-04-12", "rohan_k"); // diff aadhaar
        assert_ne!(fingerprint6(&a), fingerprint6(&b));
        assert_ne!(fingerprint6(&a), fingerprint6(&c));
    }

    #[test]
    fn wrong_key_does_not_recover() {
        let fp = [9u8; 6];
        let p = encode_password(7, 1_700_000_000, &fp, b"key-a");
        assert_ne!(decode_password(&p, b"key-b"), Some((7, 1_700_000_000, fp)));
    }

    #[test]
    fn seed_chain_advances() {
        let root = root_hex(&identity_root("ABCDE1234F", "123456789012", "1998-04-12", "rohan_k"));
        let s0 = seed_genesis(&root);
        let s1 = seed_next(&s0, 1);
        assert_ne!(s0, s1);
    }
}
