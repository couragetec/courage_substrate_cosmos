use sp_std::vec::Vec;
use sha2::{Digest, Sha256};

pub fn get_sha_value(from: &[u8]) -> Vec<u8> {
    let mut digest = Sha256::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}