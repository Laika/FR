use crate::algs::{is_prime, is_square};
use crate::consts::PRIMES as primes;
use crate::traits::{Factor, Factorizer, Factors};
use num::bigint::BigInt;
use num::ToPrimitive;
use std::collections::HashMap;

pub struct Trial {
    n: BigInt,
}
impl Trial {
    pub fn new(n: BigInt) -> Self {
        Self { n }
    }
}

impl Factorizer for Trial {
    fn factorize(&self) -> Option<Factors> {
        factorize(&self.n)
    }
}

fn factorize(n: &BigInt) -> Option<Factors> {
    let mut n = n.clone();

    let mut factors = Factors::new(None);
    while n > BigInt::from(1u64) && !is_prime(&n) {
        for x in primes {
            if n.clone() % x.clone() == BigInt::from(0u64) {
                factors.add(BigInt::from(x.clone()));
                n /= x;
                break;
            }
        }
    }
    if n > BigInt::from(1u64) {
        factors.add(n.clone());
    }

    Some(factors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;

    #[test]
    fn test_trial() {
        let n = BigInt::from(64712443855127040u64);
        let ff = Trial::new(n.clone());
        let f = ff.factorize().unwrap();
        assert_eq!(f.n(), n);
    }
}
