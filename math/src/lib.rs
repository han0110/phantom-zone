pub mod decomposer;
pub mod distribution;
pub mod modulus;
pub mod poly;
pub mod ring;
pub mod util;

pub mod prelude {
    pub use crate::{
        decomposer::{Decomposer, DecompositionParam},
        distribution::{DistributionSized, Gaussian, Sampler, Ternary},
        modulus::{
            ElemFrom, ElemOps, ElemTo, Modulus, ModulusOps, Native, NonNativePowerOfTwo, Prime,
        },
        ring::{
            NativeRing, NoisyNativeRing, NoisyNonNativePowerOfTwoRing, NoisyPrimeRing,
            NonNativePowerOfTwoRing, PrimeRing, RingOps,
        },
        util::compact::Compact,
    };
}

pub static mut TIMER: [std::time::Duration; 8] = [std::time::Duration::new(0, 0); 8];

pub fn timer() -> [std::time::Duration; 8] {
    unsafe { TIMER }
}

pub fn reset_timer() {
    unsafe { TIMER = Default::default() }
}

pub fn add_time_ks(sss: std::time::Instant) {
    unsafe { TIMER[0] += sss.elapsed() }
}

pub fn add_time_br(sss: std::time::Instant) {
    unsafe { TIMER[1] += sss.elapsed() }
}

pub fn add_time_rgsw(sss: std::time::Instant) {
    unsafe { TIMER[2] += sss.elapsed() }
}

pub fn add_time_auto(sss: std::time::Instant) {
    unsafe { TIMER[3] += sss.elapsed() }
}

pub fn add_time_forw(sss: std::time::Instant) {
    unsafe { TIMER[4] += sss.elapsed() }
}

pub fn add_time_back(sss: std::time::Instant) {
    unsafe { TIMER[5] += sss.elapsed() }
}

pub fn add_time_eval(sss: std::time::Instant) {
    unsafe { TIMER[6] += sss.elapsed() }
}

pub fn add_time_deco(sss: std::time::Instant) {
    unsafe { TIMER[7] += sss.elapsed() }
}
