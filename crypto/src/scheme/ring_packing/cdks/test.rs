use crate::{
    core::{
        lwe::test::LweParam,
        rlwe::{test::RlweParam, RlweCiphertext},
    },
    scheme::ring_packing::cdks::{self, packing_key_gen, prepare_packing_key, CdksKey, CdksParam},
    util::rng::StdLweRng,
};
use itertools::{izip, Itertools};
use phantom_zone_math::{
    decomposer::DecompositionParam,
    distribution::Sampler,
    distribution::{Gaussian, Ternary},
    modulus::{Modulus, ModulusOps, Native, Prime},
    ring::{PrimeRing, RingOps},
};
use rand::SeedableRng;

fn test_param(modulus: impl Into<Modulus>) -> CdksParam {
    let ring_size = 1024;
    CdksParam {
        modulus: modulus.into(),
        ring_size,
        sk_distribution: Ternary.into(),
        noise_distribution: Gaussian(3.19).into(),
        auto_decomposition_param: DecompositionParam {
            log_base: 17,
            level: 1,
        },
    }
}

impl From<CdksParam> for RlweParam {
    fn from(param: CdksParam) -> Self {
        RlweParam {
            message_modulus: 4,
            ciphertext_modulus: param.modulus,
            ring_size: param.ring_size,
            sk_distribution: param.sk_distribution,
            noise_distribution: param.noise_distribution,
            u_distribution: Ternary.into(),
            ks_decomposition_param: param.auto_decomposition_param,
        }
    }
}

impl From<CdksParam> for LweParam {
    fn from(param: CdksParam) -> Self {
        RlweParam::from(param).to_lwe()
    }
}

#[test]
fn pack_lwes() {
    fn run<R: RingOps>(modulus: impl Into<Modulus>) {
        let mut rng = StdLweRng::from_entropy();
        let param = test_param(modulus);
        let rlwe = RlweParam::from(param).build::<R>();
        let ring = rlwe.ring();
        let ring_size = ring.ring_size();
        let lwe = LweParam::from(param).build::<R>();

        let sk = rlwe.sk_gen(&mut rng);
        let decrypt = |ct: &_| rlwe.decode(&rlwe.decrypt(&sk.automorphism(2 * ring_size - 1), ct));
        let packing_key = {
            let mut packing_key = CdksKey::allocate(param);
            packing_key_gen(ring, &mut packing_key, &sk, &mut rng);
            let mut packing_key_prep = CdksKey::allocate_eval(param, ring.eval_size());
            prepare_packing_key(ring, &mut packing_key_prep, &packing_key);
            packing_key_prep
        };
        let ms = rlwe.message_ring().sample_uniform_vec(ring_size, &mut rng);
        let cts = ms
            .iter()
            .map(|m| lwe.sk_encrypt(&sk.clone().into(), lwe.encode(*m), &mut rng))
            .collect_vec();
        let mut ct = RlweCiphertext::allocate(ring_size);
        for k in 0..=ring_size {
            let ell = k.next_power_of_two().ilog2() as usize;
            cdks::pack_lwes(ring, &mut ct, &packing_key, &cts[..k]);
            izip!(&ms[..k], decrypt(&ct).into_iter().step_by(ring_size >> ell))
                .for_each(|(a, b)| assert!(*a == b));
        }
    }

    run::<PrimeRing>(Prime::gen(54, 11));
}

#[test]
fn pack_lwes_ms() {
    fn run<M: ModulusOps, R: RingOps>(
        lwe_modulus: impl Into<Modulus>,
        modulus: impl Into<Modulus>,
    ) {
        let mut rng = StdLweRng::from_entropy();
        let lwe_param = test_param(lwe_modulus);
        let lwe = LweParam::from(lwe_param).build::<M>();
        let param = test_param(modulus);
        let rlwe = RlweParam::from(param).build::<R>();
        let ring = rlwe.ring();
        let ring_size = ring.ring_size();
        let mod_lwe = lwe.modulus();

        let sk = rlwe.sk_gen(&mut rng);
        let decrypt = |ct: &_| rlwe.decode(&rlwe.decrypt(&sk.automorphism(2 * ring_size - 1), ct));
        let packing_key = {
            let mut packing_key = CdksKey::allocate(param);
            packing_key_gen(ring, &mut packing_key, &sk, &mut rng);
            let mut packing_key_prep = CdksKey::allocate_eval(param, ring.eval_size());
            prepare_packing_key(ring, &mut packing_key_prep, &packing_key);
            packing_key_prep
        };
        let ms = rlwe.message_ring().sample_uniform_vec(ring_size, &mut rng);
        let cts = ms
            .iter()
            .map(|m| lwe.sk_encrypt(&sk.clone().into(), lwe.encode(*m), &mut rng))
            .collect_vec();
        let mut ct = RlweCiphertext::allocate(ring_size);
        for k in 0..=ring_size {
            let ell = k.next_power_of_two().ilog2() as usize;
            cdks::pack_lwes_ms(mod_lwe, ring, &mut ct, &packing_key, &cts[..k]);
            izip!(&ms[..k], decrypt(&ct).into_iter().step_by(ring_size >> ell))
                .for_each(|(a, b)| assert!(*a == b));
        }
    }

    run::<Native, PrimeRing>(Native::native(), Prime::gen(61, 11));
}
