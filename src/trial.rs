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
    while !is_prime(n.clone()) {
        for x in primes {
            if n.clone() % x.clone() == BigInt::from(0u64) {
                factors.add(BigInt::from(x.clone()));
                n /= x;
                break;
            }
        }
    }
    factors.add(n.clone());

    Some(factors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;

    #[test]
    fn test_trial() {
        let n = BigInt::parse_bytes("895649414291294604941588381871244924626104121562042227318384494381723497514540860474803494041479529".as_bytes(), 10).unwrap();
        let ff = Trial::new(n);
        let f = ff.factorize().unwrap();
        println!("{f:?}");
        //        assert_eq!(
        //            f[0].pe(),
        //            BigInt::parse_bytes(
        //                "29927402397991286489627904551843385490310576382227".as_bytes(),
        //                10
        //            )
        //            .unwrap()
        //        );
        //        assert_eq!(
        //            f[1].pe(),
        //            BigInt::parse_bytes(
        //                "29927402397991286489627837734179186385188296382227".as_bytes(),
        //                10
        //            )
        //            .unwrap()
        //        );
    }
}
