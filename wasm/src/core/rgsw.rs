use crate::util::impl_constructor_from_object;
use phantom_zone_crypto::core::rgsw;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RgswDecompositionParam {
    pub log_base: u32,
    pub level_a: u32,
    pub level_b: u32,
}

impl From<RgswDecompositionParam> for rgsw::RgswDecompositionParam {
    fn from(value: RgswDecompositionParam) -> Self {
        Self {
            log_base: value.log_base as _,
            level_a: value.level_a as _,
            level_b: value.level_b as _,
        }
    }
}

impl_constructor_from_object!(RgswDecompositionParam);
