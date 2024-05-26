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

pub fn scalar_mult(x: &BigUint, s: &BigUint, modulus: &BigUint) -> BigUint {
    let mut a = BigUint::from(1_u8);
    let mut b = x.clone();
    let mut c = BigUint::ZERO;
    let mut d = BigUint::from(1_u8);
    let (mut e, mut f);
    let mut clamped = s.to_bytes_le();
    clamped.resize(32, 0);
    clamped[0] &= 0xf8;
    clamped[31] = (clamped[31] & 0x7f) | 0x40;
    let mut bit = 0;
    for i in 254..=0 {
        bit = (clamped[i >> 3] >> (i & 7)) & 1;
        if bit == 1 {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut c, &mut d);
        }
        e = &a + &c;
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
        a = &c * BigUint::from(0xDB41_u16);
        a = &a + &d;
        c = &c * &a;
        a = &d * &f;
        d = &b * x;
        b = &e * &e;
        if bit == 1 {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut c, &mut d);
        }
    }
    println!("{c}");
    c = c.modinv(modulus).unwrap();
    a = &a*&c;
    a
}
