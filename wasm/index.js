const pz = require('./pkg/phantom_zone_wasm.js')

const param = new pz.FhewParam({
    modulus: { "Prime": 18014398509404161n },
    ring_size: 2048,
    sk_distribution: "Ternary",
    noise_distribution: { "Gaussian": 3.19 },
    auto_decomposition_param: {
        log_base: 24,
        level: 1,
    },
    rlwe_by_rgsw_decomposition_param: {
        log_base: 17,
        level_a: 1,
        level_b: 1,
    },
    lwe_modulus: { PowerOfTwo: 16 },
    lwe_dimension: 620,
    lwe_sk_distribution: "Ternary",
    lwe_noise_distribution: { "Gaussian": 3.19 },
    lwe_ks_decomposition_param: {
        log_base: 1,
        level: 13,
    },
    q: 2048,
    g: 5,
    w: 10,
    u_distribution: "Ternary",
    rgsw_by_rgsw_decomposition_param: {
        log_base: 6,
        level_a: 7,
        level_b: 6,
    },
    total_shares: 4,
})
const crs = new pz.FhewCrs({ seed: new Uint8Array(32) })
const shareIdx = 0
const sk = pz.RlweSecretKey.sample(param.ring_size, { "Gaussian": 3.19 }, new Uint8Array(32))
const pk = sk.pk()
const skKs = pz.LweSecretKey.sample(param.lwe_dimension, "Ternary", new Uint8Array(32))
const bsKeyShare = pz.FhewBootstrappingKeyShare.generate(param, crs, shareIdx, sk, pk, skKs, new Uint8Array(32))

// console.log(sk.toBytes())
// console.log(pz.LweSecretKey.fromBytes(skKs.toBytes()))
const bytes = bsKeyShare.toBytes()
console.log(Buffer.from(bytes, 0, bytes.length).toString('hex') == Buffer.from(pz.FhewBootstrappingKeyShare.fromBytes(bytes).toBytes(), 0, bytes.length).toString('hex'))
// console.log((new pz.RlweSecretKey(sk.toObject())).toObject())
