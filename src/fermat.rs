use crate::algs::is_square;
use crate::traits::{Factor, Factorizer};
use num::bigint::BigInt;

pub struct Fermat {
    n: BigInt,
}
impl Fermat {
    pub fn new(n: BigInt) -> Self {
        Self { n }
    }
}

impl Factorizer for Fermat {
    fn factorize(&self) -> Option<Vec<Factor>> {
        factorize(&self.n)
    }
}

fn factorize(n: &BigInt) -> Option<Vec<Factor>> {
    let mut x = n.sqrt();
    let mut y2 = x.clone() * x.clone() - n.clone();
    while !is_square(&y2) {
        x += 1u32;
        y2 = x.clone() * x.clone() - n;
    }
    let a = x.clone() + y2.sqrt();
    let b = x.clone() - y2.sqrt();

    let p1 = Factor::new(a, 1);
    let p2 = Factor::new(b, 1);

    Some(vec![p1, p2])
}
