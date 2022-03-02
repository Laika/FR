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

pub fn isqrt(n: &BigInt) -> BigInt {
    let n = n.clone();
    if n.clone() < 2u64.to_bigint().unwrap() {
        return n.clone();
    }
    let mut shift: u64 = 2u64;
    while (n.clone() >> shift) > Zero::zero() {
        shift += 2u64;
    }

    let mut result: BigInt = Zero::zero();
    while shift >= 0u64 {
        result = result.clone() << 1u32;
        let r = result.clone() + 1u32;
        if r.clone() * r.clone() <= (n.clone() >> shift) {
            result = r;
        }
        if shift >= 2u64 {
            shift -= 2u64;
        } else {
            return result;
        }
    }
    result
}

pub fn is_square(n: &BigInt) -> bool {
    let n = n.clone();
    let x = isqrt(&n);

    x.clone() * x == n
}
