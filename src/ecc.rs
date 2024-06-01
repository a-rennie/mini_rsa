use num_bigint::BigInt;
use num_bigint::{BigUint, RandBigInt};
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
    let largest = std::cmp::max(
        x.modpow(&BigUint::from(2u8), modulus),
        z.modpow(&BigUint::from(2u8), modulus),
    );
    let smallest = std::cmp::min(
        x.modpow(&BigUint::from(2u8), modulus),
        z.modpow(&BigUint::from(2u8), modulus),
    );
    let x2 = (largest - smallest).modpow(&BigUint::from(2u8), modulus);
    let z2 = BigUint::from(4u8)
        * x
        * z
        * (x.modpow(&BigUint::from(2u8), modulus)
            + BigUint::from(486662_u32) * x * z
            + z.modpow(&BigUint::from(2u8), modulus));
    (x2, z2)
}

pub fn scalar_mult(x: &BigInt, s: &BigUint, modulus: &BigInt) -> BigInt {
    let mut clamped = s.to_bytes_le();
    clamped.resize(32, 0);
    let mut a = BigInt::one();
    let mut b = x.clone();
    let mut c = BigInt::ZERO;
    let mut d = BigInt::one();
    let mut e;
    let mut f;

    for i in (0..254).rev() {
        let bit = (clamped[i >> 3] >> (i & 7)) & 1;
        if bit == 1 {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut c, &mut d);
        }
        e = &a+&c;
        a = &a - &c;
        c = &b + &d;
        b = &b - &d;
        d = &e * &e;
        f = &a * &a;
        a = &c * &a;
        c = &b * &e;
        e = &a + &c;
        a = &a - &c;
        b = &a * &a;
        c = &d - &f;
        a = &c * &BigInt::from(121665_u32);
        a = &a + &d;
        c = &c * &a;
        a = &d * &f;
        d = &b * x;
        b = &e * &e;
        if bit == 1 {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut c, &mut d);
        }

        a = &a % modulus;
        b = &b % modulus;
        c = &c % modulus;
        d = &d % modulus;
        e = &a % modulus;
        f = &f % modulus;


    }

    (a * c.modinv(modulus).unwrap()) % modulus

}
