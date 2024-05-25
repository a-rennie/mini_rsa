use num_primes::{BigUint, RandBigInt};
use num_traits::One;
use rand::thread_rng;

pub struct EccKeyPair {
    pub public: BigUint,
    pub private: BigUint,
}

// X25519 (y^2 = x^3 + 486662x^2 + x)
impl EccKeyPair {
    pub fn new(bits: u64) -> EccKeyPair {
        let mut rng = thread_rng();
        let mut private = rng.gen_biguint(bits);
        let modulus = BigUint::from(2u8).pow(255) - BigUint::from(19u8);
        todo!()
    }
}

pub fn double_ec_point(x: &BigUint, z: &BigUint, modulus: &BigUint) -> (BigUint, BigUint) {
    // (x, z)
    let largest = std::cmp::max(x.modpow(&BigUint::from(2u8), modulus), z.modpow(&BigUint::from(2u8), modulus));
    let smallest = std::cmp::min(x.modpow(&BigUint::from(2u8), modulus), z.modpow(&BigUint::from(2u8), modulus));
    let x2 = (largest - smallest).modpow(&BigUint::from(2u8), modulus);
    let z2 = BigUint::from(4u8) * x * z * (x.modpow(&BigUint::from(2u8), modulus) + BigUint::from(486662_u32) * x * z + z.modpow(&BigUint::from(2u8), modulus));
    (x2, z2)
}