use crate::util::impl_from_into_wasm_abi;
use phantom_zone_math::modulus::{self, Native, NonNativePowerOfTwo, Prime};
use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Modulus {
    #[serde(deserialize_with = "deserialize_power_of_two")]
    PowerOfTwo(usize),
    #[serde(deserialize_with = "deserialize_prime")]
    Prime(u64),
}

impl From<Modulus> for modulus::Modulus {
    fn from(value: Modulus) -> Self {
        match value {
            Modulus::PowerOfTwo(64) => Native::native().into(),
            Modulus::PowerOfTwo(bits) => NonNativePowerOfTwo::new(bits).into(),
            Modulus::Prime(q) => Prime::new(q).into(),
        }
    }
}

impl_from_into_wasm_abi!(Modulus);

fn deserialize_power_of_two<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
    u32::deserialize(deserializer).and_then(|bits| {
        (bits <= 64).then_some(bits as _).ok_or_else(|| {
            let msg = format!("invalid `PowerOfTwo` bits {bits}, expected at most 64");
            Error::custom(msg)
        })
    })
}

fn deserialize_prime<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    u128::deserialize(deserializer).and_then(|q| {
        (q < 1u128 << 64).then_some(q as _).ok_or_else(|| {
            let msg = format!("invalid `Prime` {q}, expected at most 2^64-1");
            Error::custom(msg)
        })
    })
}
