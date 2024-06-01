mod ecc;
mod prime_gen;
mod rsa;

use crate::ecc::{double_ec_point, scalar_mult, EccKeyPair};
use crate::prime_gen::prime_with_bits;
use crate::rsa::RsaKeyPair;
use num_bigint::{BigInt, BigUint};
use std::str::FromStr;

fn main() {
    // let prime = prime_with_bits(512);
    // println!("{prime}")
    let scalar = scalar_mult(&BigInt::from(9_u8), &BigUint::from(0x12_u8), &(BigInt::from(2_u8).pow(255) - BigInt::from(19_u8)));
    println!("{scalar}")

    // let modulus = BigInt::from(2u8).pow(255) - BigInt::from(19u8);
    // let x = BigInt::from(9u8);
    // println!("{}", scalar_mult(&x, &BigInt::from(3u8), &modulus));
    // panic!();
    // let y = BigUint::from_str(
    //     "14781619447589544791020593568409986887264606134616475288964881837755586237401",
    // )
    // .unwrap();
    // let l = ((BigUint::from(3u8) * x.pow(2)
    //     + BigUint::from(2u8) * BigUint::from(486662u32) * &x
    //     + BigUint::from(1u8))
    //     * (&y + &y).modinv(&modulus).unwrap())
    //     % &modulus;
    // let x2 = l.modpow(&BigUint::from(2u8), &modulus) - BigUint::from(486662u32) - &x - &x;
    // println!("{x2}");
    //
    // let x = BigUint::from_str("9").unwrap();
    // let z = BigUint::from(1u8);
    // let x2 =
    //     (x.modpow(&BigUint::from(2u8), &modulus) - z.pow(2)).modpow(&BigUint::from(2u8), &modulus);
    // let z2 = BigUint::from(4u8)
    //     * &x
    //     * &z
    //     * (x.modpow(&BigUint::from(2u8), &modulus)
    //         + BigUint::from(486662u32) * &x * &z
    //         + z.modpow(&BigUint::from(2u8), &modulus));
    // let point = (&x2 * (&z2 % &modulus).modinv(&modulus).unwrap()) % &modulus;
    // println!("{point}, {x2}, {z2}");
    //
    // let x = BigUint::from_str("9").unwrap();
    // let z = BigUint::from(1u8);
    // let point = double_ec_point(&x, &z, &modulus);
    // let (x2, z2) = &point;
    // let point = (x2 * (z2 % &modulus).modinv(&modulus).unwrap()) % &modulus;
    // println!("{point:?}");
    // let point = double_ec_point(x2, z2, &modulus);
    // let (x2, z2) = &point;
    // let point = (x2 * (z2 % &modulus).modinv(&modulus).unwrap()) % &modulus;
    // println!("{point:?}")
    //
    // // let point = double_ec_point(&BigUint::from(9_u32), &BigUint::from(1_u32), &(BigUint::from(2u8).pow(255) - BigUint::from(19u8)));
    // // let x = point.0 * point.1.modinv(&(BigUint::from(2u8).pow(255) - BigUint::from(19u8))).unwrap();
    // // println!("{:?}", (x));
    // //
    // // let key_pair = RsaKeyPair::new(1024);
    // // let message = "This is a test message";
    // // let encrypted = key_pair.public_key().encrypt_str(message);
    // // let decrypted = key_pair.decrypt_to_str(encrypted.clone());
    // // println!("{message}\n{encrypted:?}\n{decrypted}")
}
