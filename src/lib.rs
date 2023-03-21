use log::info;
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const DIFFICULTY_PREFIX: &str = "00";

pub fn calculate_hash(
    id: u64,
    timestamp: i64,
    previous_hash: &str,
    data: &str,
    nonce: u64,
) -> Vec<u8> {
    let data: Value = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

pub fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");
    let mut nonce: u64 = 0;

    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash: Vec<u8> = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash: String = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
