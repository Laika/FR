use crate::algs::{is_prime, is_square};
use crate::traits::{Factor, Factorizer, Factors};
use num::bigint::BigInt;
use num::ToPrimitive;
use num::Zero;
use std::collections::HashMap;
use std::io::{stdout, Write};

#[derive(Debug)]
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
    let m = n.sqrt().to_u64().unwrap();

    let mut factors = Factors::new(None);
    for i in 2..=m {
        while n.clone() % i.clone() == BigInt::zero() {
            factors.add(BigInt::from(i.clone()));
            n /= i;
        }
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
