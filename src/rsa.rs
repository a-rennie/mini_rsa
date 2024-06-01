use crate::prime_gen;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use prime_gen::prime_with_bits;

// Uses the Euclidean algorithm
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    let largest = std::cmp::max(a, b);
    let smallest = std::cmp::min(a, b);
    if largest == smallest || smallest == &BigUint::from(0u8) {
        return largest.clone();
    }
    gcd(smallest, &(largest % smallest))
}

fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    (a * b) / gcd(a, b)
}

fn totient(p: &BigUint, q: &BigUint) -> BigUint {
    lcm(&(p - BigUint::one()), &(q - BigUint::one()))
}

pub struct PublicKey {
    n: BigUint,
    e: BigUint,
}

impl PublicKey {
    pub fn encrypt(&self, message: Vec<u8>) -> Vec<BigUint> {
        message
            .into_iter()
            .map(|x| BigUint::from(x).modpow(&self.e, &self.n))
            .collect()
    }

    pub fn encrypt_str(&self, message: &str) -> Vec<BigUint> {
        self.encrypt(message.as_bytes().into())
    }
}

struct PrivateKey {
    d: BigUint,
}
pub struct RsaKeyPair {
    public: PublicKey,
    private: PrivateKey,
}

impl RsaKeyPair {
    pub fn new(bits: u64) -> RsaKeyPair {
        let p: BigUint = prime_with_bits(bits);
        let q: BigUint = prime_with_bits(bits);
        let n = &p * &q;
        let totient = totient(&p, &q);
        let e = BigUint::from(65537_u32);
        let d = e.modinv(&totient).unwrap();
        let public = PublicKey { n, e };

        let private = PrivateKey { d };

        RsaKeyPair { public, private }
    }

    pub fn decrypt(&self, message: Vec<BigUint>) -> Vec<u8> {
        message
            .into_iter()
            .map(|x| {
                x.modpow(&self.private.d, &self.public.n)
                    .to_u8()
                    .expect("Invalid message")
            })
            .collect()
    }

    pub fn decrypt_to_str(&self, message: Vec<BigUint>) -> String {
        String::from_utf8(self.decrypt(message)).expect("Invalid message")
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }
}
