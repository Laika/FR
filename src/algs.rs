use num::bigint::{BigInt, ToBigInt};
use num::traits::Zero;

pub fn mod_pow(b: u128, e: u128, m: u128) -> u128 {
    let mut s = 1;
    let mut t = b;
    let mut exp = e;

    while exp > 0 {
        if exp & 1 == 1 {
            s = s * t % m;
        }
        t = t * t % m;
        exp >>= 1;
        println!("{s}, {t}, {exp}");
    }
    s
}

pub fn is_quadratic(a: u128, p: u128) -> bool {
    mod_pow(a, (p - 1) / 2, p) == 1
}

pub fn is_square(n: &BigInt) -> bool {
    let n = n.clone();
    let x = n.sqrt();

    x.clone() * x == n
}