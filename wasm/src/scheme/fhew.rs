use crate::{
    core::{
        lwe::{DecompositionParam, LweSecretKey},
        rgsw::RgswDecompositionParam,
        rlwe::{RlwePublicKey, RlweSecretKey},
    },
    util::{
        distribution::{NoiseDistribution, SecretDistribution},
        impl_constructor_from_object,
        modulus::Modulus,
        rng::{DefaultRng, DefaultSeed},
    },
};
use phantom_zone_crypto::scheme::blind_rotation::lmkcdey::{
    interactive::{
        bs_key_share_gen, LmkcdeyInteractiveCrs, LmkcdeyInteractiveParam, LmkcdeyKeyShareCompact,
        LmkcdeyKeyShareOwned,
    },
    LmkcdeyParam,
};
use phantom_zone_math::{
    modulus::{self, NonNativePowerOfTwo},
    ring::{NativeRing, NonNativePowerOfTwoRing, PrimeRing, RingOps},
};
use rand_chacha::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FhewParam {
    // Rlwe param
    pub modulus: Modulus,
    pub ring_size: u32,
    pub sk_distribution: SecretDistribution,
    pub noise_distribution: NoiseDistribution,
    pub auto_decomposition_param: DecompositionParam,
    pub rlwe_by_rgsw_decomposition_param: RgswDecompositionParam,
    // Lwe param
    pub lwe_modulus: Modulus,
    pub lwe_dimension: u32,
    pub lwe_sk_distribution: SecretDistribution,
    pub lwe_noise_distribution: NoiseDistribution,
    pub lwe_ks_decomposition_param: DecompositionParam,
    // Blind rotation param
    pub q: u32,
    pub g: u32,
    pub w: u32,
    // Multi-party param
    pub u_distribution: SecretDistribution,
    pub rgsw_by_rgsw_decomposition_param: RgswDecompositionParam,
    pub total_shares: u32,
}

impl From<FhewParam> for LmkcdeyInteractiveParam {
    fn from(value: FhewParam) -> Self {
        Self {
            param: LmkcdeyParam {
                modulus: value.modulus.into(),
                ring_size: value.ring_size as _,
                sk_distribution: value.sk_distribution.into(),
                noise_distribution: value.noise_distribution.into(),
                auto_decomposition_param: value.auto_decomposition_param.into(),
                rlwe_by_rgsw_decomposition_param: value.rlwe_by_rgsw_decomposition_param.into(),
                lwe_modulus: value.lwe_modulus.into(),
                lwe_dimension: value.lwe_dimension as _,
                lwe_sk_distribution: value.lwe_sk_distribution.into(),
                lwe_noise_distribution: value.lwe_noise_distribution.into(),
                lwe_ks_decomposition_param: value.lwe_ks_decomposition_param.into(),
                q: value.q as _,
                g: value.g as _,
                w: value.w as _,
            },
            u_distribution: value.u_distribution.into(),
            rgsw_by_rgsw_decomposition_param: value.rgsw_by_rgsw_decomposition_param.into(),
            total_shares: value.total_shares as _,
        }
    }
}

impl_constructor_from_object!(FhewParam);

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FhewCrs(LmkcdeyInteractiveCrs<DefaultRng>);

impl From<FhewCrs> for LmkcdeyInteractiveCrs<DefaultRng> {
    fn from(value: FhewCrs) -> Self {
        value.0
    }
}

impl_constructor_from_object!(FhewCrs);

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FhewBootstrappingKeyShare(LmkcdeyKeyShareOwned<u64, u64, DefaultRng>);

#[wasm_bindgen]
impl FhewBootstrappingKeyShare {
    #[wasm_bindgen]
    pub fn generate(
        param: &FhewParam,
        crs: &FhewCrs,
        share_idx: u32,
        sk: &RlweSecretKey,
        pk: &RlwePublicKey,
        sk_ks: &LweSecretKey,
        seed: DefaultSeed,
    ) -> Result<FhewBootstrappingKeyShare, String> {
        let param = LmkcdeyInteractiveParam::from(*param);
        let crs = LmkcdeyInteractiveCrs::from(*crs);
        let mod_ks = NonNativePowerOfTwo::try_from(param.lwe_modulus)
            .map_err(|_| "Non `PowerOfTwo` LWE modulus is not supported")?;
        let mut bs_key_share = LmkcdeyKeyShareOwned::allocate(param, crs, share_idx as _);
        match param.modulus {
            modulus::Modulus::Native(_) => {
                let ring = <NativeRing as RingOps>::new(param.modulus, param.ring_size);
                let mut scratch = ring.allocate_scratch(2, 3, 0);
                bs_key_share_gen(
                    &ring,
                    &mod_ks,
                    &mut bs_key_share,
                    sk,
                    pk,
                    sk_ks,
                    scratch.borrow_mut(),
                    &mut DefaultRng::from_seed(seed.into()),
                )
            }
            modulus::Modulus::NonNativePowerOfTwo(_) => {
                let ring =
                    <NonNativePowerOfTwoRing as RingOps>::new(param.modulus, param.ring_size);
                let mut scratch = ring.allocate_scratch(2, 3, 0);
                bs_key_share_gen(
                    &ring,
                    &mod_ks,
                    &mut bs_key_share,
                    sk,
                    pk,
                    sk_ks,
                    scratch.borrow_mut(),
                    &mut DefaultRng::from_seed(seed.into()),
                )
            }
            modulus::Modulus::Prime(_) => {
                let ring = <PrimeRing as RingOps>::new(param.modulus, param.ring_size);
                let mut scratch = ring.allocate_scratch(2, 3, 0);
                bs_key_share_gen(
                    &ring,
                    &mod_ks,
                    &mut bs_key_share,
                    sk,
                    pk,
                    sk_ks,
                    scratch.borrow_mut(),
                    &mut DefaultRng::from_seed(seed.into()),
                )
            }
        }
        Ok(Self(bs_key_share))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<FhewBootstrappingKeyShare, String> {
        let compact: LmkcdeyKeyShareCompact<DefaultRng> =
            bincode::deserialize(bytes).map_err(|err| err.to_string())?;
        let mod_ks = NonNativePowerOfTwo::try_from(compact.param().lwe_modulus).unwrap();
        match compact.param().modulus {
            modulus::Modulus::Native(_) => {
                let ring = <NativeRing as RingOps>::new(
                    compact.param().modulus,
                    compact.param().ring_size,
                );
                Ok(Self(compact.uncompact(&ring, &mod_ks)))
            }
            modulus::Modulus::NonNativePowerOfTwo(_) => {
                let ring = <NonNativePowerOfTwoRing as RingOps>::new(
                    compact.param().modulus,
                    compact.param().ring_size,
                );
                Ok(Self(compact.uncompact(&ring, &mod_ks)))
            }
            modulus::Modulus::Prime(_) => {
                let ring =
                    <PrimeRing as RingOps>::new(compact.param().modulus, compact.param().ring_size);
                Ok(Self(compact.uncompact(&ring, &mod_ks)))
            }
        }
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> wasm_bindgen::JsValue {
        let mod_ks = NonNativePowerOfTwo::try_from(self.0.param().lwe_modulus).unwrap();
        let compact = match self.0.param().modulus {
            modulus::Modulus::Native(_) => {
                let ring =
                    <NativeRing as RingOps>::new(self.0.param().modulus, self.0.param().ring_size);
                self.0.compact(&ring, &mod_ks)
            }
            modulus::Modulus::NonNativePowerOfTwo(_) => {
                let ring = <NonNativePowerOfTwoRing as RingOps>::new(
                    self.0.param().modulus,
                    self.0.param().ring_size,
                );
                self.0.compact(&ring, &mod_ks)
            }
            modulus::Modulus::Prime(_) => {
                let ring =
                    <PrimeRing as RingOps>::new(self.0.param().modulus, self.0.param().ring_size);
                self.0.compact(&ring, &mod_ks)
            }
        };
        bincode::serialize(&compact).unwrap().into()
    }
}
