use crate::util::{
    distribution::SecretDistribution,
    impl_constructor_from_object, impl_from_to_bytes,
    rng::{DefaultRng, DefaultSeed},
};
use phantom_zone_crypto::core::lwe::{LweSecretKeyOwned, LweSecretKeyView};
use phantom_zone_math::decomposer;
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DecompositionParam {
    pub log_base: u32,
    pub level: u32,
}

impl From<DecompositionParam> for decomposer::DecompositionParam {
    fn from(value: DecompositionParam) -> Self {
        Self {
            log_base: value.log_base as _,
            level: value.level as _,
        }
    }
}

impl_constructor_from_object!(DecompositionParam);

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LweSecretKey(LweSecretKeyOwned<i32>);

#[wasm_bindgen]
impl LweSecretKey {
    #[wasm_bindgen]
    pub fn sample(
        dimension: u32,
        sk_distribution: SecretDistribution,
        seed: DefaultSeed,
    ) -> Result<LweSecretKey, String> {
        Ok(Self(LweSecretKeyOwned::sample(
            dimension as _,
            sk_distribution.into(),
            DefaultRng::from_seed(seed.into()),
        )))
    }
}

impl<'a> From<&'a LweSecretKey> for LweSecretKeyView<'a, i32> {
    fn from(value: &'a LweSecretKey) -> Self {
        value.0.as_view()
    }
}

impl_from_to_bytes!(LweSecretKey);
