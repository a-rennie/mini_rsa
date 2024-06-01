use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::thread_rng;

const MILLER_RABIN_ITERATIONS: u8 = 8;

pub fn prime_with_bits(bits: u64) -> BigUint {
    let mut rng = thread_rng();
    let mut prime = rng.gen_biguint(bits);
    while !is_prime(&prime) {
        prime = rng.gen_biguint(bits)
    }
    prime
}

fn is_prime(n: &BigUint) -> bool {
    let zero = BigUint::ZERO;
    let one = BigUint::one();
    let two = &one + &one;
    let three = &one + &two;

    // Handle base cases
    if n == &two {
        return true;
    }
    if n == &one || n == &three {
        return false;
    }
    if n % &two == zero {
        return false;
    }

    // Find d such that d * 2^r = n-1
    let mut d = n - one;
    while &d % &two == zero {
        d /= &two
    }

    for _ in 0..MILLER_RABIN_ITERATIONS {
        if !miller_rabin_test(&d, n) {
            return false;
        }
    }

    true
}

fn miller_rabin_test(d: &BigUint, n: &BigUint) -> bool {
    let one = BigUint::one();
    let two = &one + &one;

    let mut d = d.clone();
    let mut rng = thread_rng();
    let a = rng.gen_biguint_range(&two, &(n - &two));
    let mut x = a.modpow(&d, n);

    if x == BigUint::from(1_u8) || x == n - &one {
        return true;
    }

    while d != n - &one {
        x = x.pow(2) % n;
        d *= &two;

        if x == one {
            return false;
        }
        if x == n - &one {
            return true;
        }
    }
    false
}
