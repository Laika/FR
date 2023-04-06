use crate::consts::PRIMES as primes;
use crate::elliptic_curve::{scalar_mul_for_factorization, EllipticCurve};
use crate::galois_field::GaloisField;
use crate::traits::{Factorizer, Factors};
use num::bigint::BigInt;
use num_bigint::RandBigInt;

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
    let n = n.clone();
    let mut rng = rand::thread_rng();
    let low = BigInt::from(2);
    let high = n.clone();
    let l = 242u64;

    loop {
        let f = GaloisField::GaloisField(&n);
        let x0 = f.new(&rng.gen_bigint_range(&low, &high));
        let y0 = f.new(&rng.gen_bigint_range(&low, &high));
        let a = f.new(&rng.gen_bigint_range(&low, &high));
        let b = y0.clone().pow(&BigInt::from(2))?
            - x0.clone().pow(&BigInt::from(3))?
            - a.clone() * x0.clone();
        let e = EllipticCurve::new(&f, &a.value, &b.value);
        let g = e.new_point(&x0.value, &y0.value);

        for p in primes {
            let mut m: u64 = p.clone();
            //let mut pg; // Previous g
            let cg = (BigInt::from(2) * g.clone())?; // Current g

            while m.clone() * p.clone() <= l {
                m *= p.clone();
            }

            let xg = scalar_mul_for_factorization(BigInt::from(m), g.clone());
            println!("{p} | {xg:?}");
            //for k in 1..=m {
            //    pg = cg.clone();
            //    cg = (cg.clone() + g.clone())?;
            //    println!("{k}/{m}");
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
    use crate::bi;
    use num::bigint::BigInt;

    #[test]
    fn test_ecm() {
        //let n = bi!("121439531096594251777", 10);
        //let n = bi!("455839", 10);
        let n = bi!("835791", 10);
        let ff = ECM::new(n);
        let f = ff.factorize();
        println!("f: {f:?}");
    }
}
