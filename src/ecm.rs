use crate::algs::{is_prime, is_square};
use crate::consts::PRIMES as primes;
use crate::traits::{Factor, Factorizer, Factors};
use num::bigint::BigInt;
use num::ToPrimitive;
use std::collections::HashMap;

pub struct ECM {
    n: BigInt,
}
impl ECM {
    pub fn new(n: BigInt) -> Self {
        Self { n }
    }
}

impl Factorizer for ECM {
    fn factorize(&self) -> Option<Factors> {
        factorize(&self.n)
    }
}

fn factorize(n: &BigInt) -> Option<Factors> {
    let mut n = n.clone();
    let mut rng = rand::thread_rng();
    let low = 2u64.to_bigint().unwrap();
    let high = n.clone();
    let l = 20000;

    loop {
        let a = rng.gen_bigint_range(&low, &high);
        let x0 = rng.gen_bigint_range(&low, &high);
        let y0 = rng.gen_bigint_range(&low, &high);
        let g = Point { x: x0, y: y0, z: 1 };
        let b = (y0.modpow(2, n) - x0.modpow(3, n) - a * x0 % n + 2 * n) % n;
        let e = EllipticCurve { a, b, n };

        for p in primes {
            let mp = p.clone();
            while mp * p <= l {
                mp *= p;
            }
            g = mp * g;
            // TODO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;

    #[test]
    fn test_ECM() {
        let n = BigInt::from(64712443855127040u64);
        let ff = ECM::new(n.clone());
        let f = ff.factorize().unwrap();
        assert_eq!(f.n(), n);
    }
}
