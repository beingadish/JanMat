#[derive(Debug, Clone)]
pub struct ZkReceipt {
    pub circuit_id: &'static str,
    pub proof_bytes: Vec<u8>,
}

pub fn verify_receipt(receipt: &ZkReceipt) -> bool {
    !receipt.proof_bytes.is_empty() && !receipt.circuit_id.is_empty()
}
