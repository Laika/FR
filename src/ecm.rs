use crate::algs::{is_prime, is_square};
use crate::consts::PRIMES as primes;
use crate::elliptic_curve::EllipticCurve;
use crate::gf::GF;
use crate::traits::{Factor, Factorizer, Factors};
use num::bigint::BigInt;
use num::Integer;
use num::ToPrimitive;
use num_bigint::RandBigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;

pub struct ECM {
    n: BigInt,
}
impl ECM {
    pub fn new(n: &BigInt) -> Self {
        Self { n: n.clone() }
    }
}

impl Factorizer for ECM {
    fn factorize(&self) -> Option<Factors> {
        factorize(&self.n)
    }
}

fn factorize(n: &BigInt) -> Option<Factors> {
    let n = n.clone();
    let mut rng = rand::thread_rng();
    let low = BigInt::from(2u64);
    let high = n.clone();
    let l = 20000_u64;

    loop {
        let f = GF::GF(&n);
        let x0 = f.new(&rng.gen_bigint_range(&low, &high));
        let y0 = f.new(&rng.gen_bigint_range(&low, &high));
        let a = f.new(&rng.gen_bigint_range(&low, &high));
        let b = y0.clone().pow(&BigInt::from(2u32))?
            - x0.clone().pow(&BigInt::from(3u32))?
            - a.clone() * x0.clone();
        let e = EllipticCurve::new(&f, &a.value, &b.value);
        let g = e.new_point(&x0.value, &y0.value);
        println!("G: {g}");

        for p in primes {
            let mut mp: u64 = p.clone();
            //let mut pg; // Previous g
            let mut cg = (BigInt::from(2u32) * g.clone())?; // Current g

            while mp.clone() * p.clone() <= l {
                mp *= p.clone();
            }

            let xg = BigInt::from(mp) * g.clone();
            let res = match xg.clone() {
                Some(_) => None,
                None => Some("Found"),
            };
            println!("{res:?}");
            println!("xg: {xg:?}");
            //for k in 1..=mp {
            //    pg = cg.clone();
            //    cg = (cg.clone() + g.clone())?;
            //    println!("{k}/{mp}");
            //    if cg == e.o() {
            //        let m = pg.x() - g.x();
            //        let q = n.gcd(&m);
            //        println!("{k} | pg: {pg}  cg: {cg}  m: {m}  found: {q}    n: {n}");
            //        if q != BigInt::one() && q != n {
            //            println!("FOUND!!!!: q: {q}");
            //            break;
            //        }
            //    }
            //}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;

    #[test]
    fn test_ECM() {
        let n = BigInt::parse_bytes("121439531096594251777".as_bytes(), 10).unwrap();
        // let n = BigInt::from(187_u32);
        let ff = ECM::new(&n);
        let f = ff.factorize().unwrap();
        println!("f: {f:?}");
        assert_eq!(f.n(), n);
    }
}
