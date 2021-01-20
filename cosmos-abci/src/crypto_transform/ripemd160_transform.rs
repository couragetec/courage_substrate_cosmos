use ripemd160::{Digest, Ripemd160};
use sp_std::vec::Vec;

pub fn get_ripemd160_value(from: &[u8]) -> Vec<u8> {
    let mut digest = Ripemd160::new();
    digest.update(from);
    let value = digest.finalize();
    value.clone().to_vec()
}
