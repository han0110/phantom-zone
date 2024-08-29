use crate::util::impl_from_into_wasm_abi;
use phantom_zone_crypto::util::distribution;
use phantom_zone_math::distribution::{Gaussian, Ternary};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SecretDistribution {
    Gaussian(f32),
    Ternary,
}

impl From<SecretDistribution> for distribution::SecretDistribution {
    fn from(value: SecretDistribution) -> Self {
        match value {
            SecretDistribution::Gaussian(std_dev) => Gaussian(std_dev as _).into(),
            SecretDistribution::Ternary => Ternary.into(),
        }
    }
}

impl_from_into_wasm_abi!(SecretDistribution);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NoiseDistribution {
    Gaussian(f32),
}

impl From<NoiseDistribution> for distribution::NoiseDistribution {
    fn from(value: NoiseDistribution) -> Self {
        match value {
            NoiseDistribution::Gaussian(std_dev) => Gaussian(std_dev as _).into(),
        }
    }
}

impl_from_into_wasm_abi!(NoiseDistribution);
