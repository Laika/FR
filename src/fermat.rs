use crate::algs::is_square;
use crate::traits::{Factor, Factorizer, Factors};
use num::bigint::BigInt;
use std::collections::HashMap;

pub struct Fermat {
    n: BigInt,
}
impl Fermat {
    pub fn new(n: BigInt) -> Self {
        Self { n }
    }
}

impl Factorizer for Fermat {
    fn factorize(&self) -> Option<Factors> {
        factorize(&self.n)
    }
}

fn factorize(n: &BigInt) -> Option<Factors> {
    let mut x: BigInt = n.sqrt() + 1u32;
    let mut y2 = x.clone() * x.clone() - n.clone();
    while !is_square(&y2) {
        x += 1u32;
        y2 = x.clone() * x.clone() - n;
    }
    let a = x.clone() + y2.sqrt();
    let b = x.clone() - y2.sqrt();

    let mut f: HashMap<BigInt, u32> = HashMap::new();
    f.insert(a, 1);
    f.insert(b, 1);
    let res = Factors::new(Some(f));
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;
    #[test]

    fn test_fermat() {
        let n = BigInt::parse_bytes("895649414291294604941588381871244924626104121562042227318384494381723497514540860474803494041479529".as_bytes(), 10).unwrap();
        let ff = Fermat::new(n);
        let factors = ff.factorize().unwrap().get_factors();
        let primes = Vec::from_iter(factors.keys());
        assert_eq!(
            *primes.clone()[0],
            BigInt::parse_bytes(
                "29927402397991286489627904551843385490310576382227".as_bytes(),
                10
            )
            .unwrap()
        );
        assert_eq!(
            *primes[1],
            BigInt::parse_bytes(
                "29927402397991286489627837734179186385188296382227".as_bytes(),
                10
            )
            .unwrap()
        );
    }
}
