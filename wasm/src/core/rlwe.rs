use crate::util::{
    distribution::SecretDistribution,
    impl_from_to_bytes,
    rng::{DefaultRng, DefaultSeed},
};
use phantom_zone_crypto::core::rlwe::{
    RlwePublicKeyOwned, RlwePublicKeyView, RlweSecretKeyOwned, RlweSecretKeyView,
};
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RlweSecretKey(RlweSecretKeyOwned<i32>);

#[wasm_bindgen]
impl RlweSecretKey {
    #[wasm_bindgen]
    pub fn sample(
        ring_size: u32,
        sk_distribution: SecretDistribution,
        seed: DefaultSeed,
    ) -> Result<RlweSecretKey, String> {
        Ok(Self(RlweSecretKeyOwned::sample(
            ring_size as _,
            sk_distribution.into(),
            DefaultRng::from_seed(seed.into()),
        )))
    }

    #[wasm_bindgen]
    pub fn pk(&self) -> Result<RlwePublicKey, String> {
        Ok(RlwePublicKey(RlwePublicKeyOwned::allocate(
            self.0.ring_size(),
        )))
    }
}

impl<'a> From<&'a RlweSecretKey> for RlweSecretKeyView<'a, i32> {
    fn from(value: &'a RlweSecretKey) -> Self {
        value.0.as_view()
    }
}

impl_from_to_bytes!(RlweSecretKey);

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RlwePublicKey(RlwePublicKeyOwned<u64>);

impl<'a> From<&'a RlwePublicKey> for RlwePublicKeyView<'a, u64> {
    fn from(value: &'a RlwePublicKey) -> Self {
        value.0.as_view()
    }
}

impl_from_to_bytes!(RlwePublicKey);
