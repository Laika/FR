use num::bigint::BigInt;
use num::Integer;
use num::ToPrimitive;
use num_bigint::{RandBigInt, ToBigInt};
use num_traits::One;

pub fn mod_pow(b: BigInt, e: u64, m: BigInt) -> BigInt {
    let mut s: BigInt = BigInt::from(1u64);
    let mut t = b;
    let mut exp = e;
    let mo = m.clone();

    while exp > 0 {
        if exp & 1 == 1 {
            s = s * t.clone() % mo.clone();
        }
        t = t.clone() * t.clone() % mo.clone();
        exp >>= 1;
    }
    s
}

pub fn inv(a: BigInt, p: BigInt) -> BigInt {
    let res = a.extended_gcd(&p);
    res.x % p
}

pub fn is_quadratic(a: BigInt, p: BigInt) -> bool {
    let e: BigInt = (p.clone() - 1u64) >> 1u64;
    a.modpow(&e, &p) == BigInt::one()
}

pub fn is_square(n: &BigInt) -> bool {
    let n = n.clone();
    let x = n.sqrt();

    x.clone() * x.clone() == n
}

pub fn miller_rabin(n: &BigInt) -> bool {
    if *n <= BigInt::one() {
        return false;
    }
    let mut m: BigInt = n - 1u64;
    let mut k: u64 = 0;
    while m.is_even() {
        k += 1u64;
        m >>= 1u64;
    }

    let threshold = 2;
    for _ in 0..threshold {
        if !internal_test(n, &m, &k) {
            return false;
        }
    }
    true
}
fn internal_test(n: &BigInt, m: &BigInt, k: &u64) -> bool {
    let mut rng = rand::thread_rng();
    let low = 2u64.to_bigint().unwrap();
    let high = n.clone();
    let a = rng.gen_bigint_range(&low, &high);

    let mut b = a.modpow(&m, &n);
    for _ in 0..*k {
        if b.clone() % n.clone() == n.clone() - 1u64 {
            return true;
        }
        b = b.modpow(&2u64.to_bigint().unwrap(), n);
    }
    false
}

pub fn is_prime(n: &BigInt) -> bool {
    miller_rabin(n)
}
