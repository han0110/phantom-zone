use crate::util::impl_from_into_wasm_abi;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

pub type DefaultRng = ChaCha20Rng;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DefaultSeed([u8; 32]);

impl From<DefaultSeed> for [u8; 32] {
    fn from(value: DefaultSeed) -> Self {
        value.0
    }
}

impl_from_into_wasm_abi!(DefaultSeed);
