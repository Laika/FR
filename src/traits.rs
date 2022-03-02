use num::BigInt;

#[derive(Debug, Clone)]
pub struct Factor {
    p: BigInt,
    e: u32,
}

impl Factor {
    pub fn new(prime: BigInt, exp: u32) -> Self {
        Self { p: prime, e: exp }
    }
    pub fn pe(&self) -> BigInt {
        let p = self.p.clone();
        p.pow(self.e)
    }
}

pub trait Factorizer {
    fn factorize(&self) -> Option<Vec<Factor>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigInt;

    #[test]
    fn test_factor() {
        let p = BigInt::parse_bytes("189237912873428934298749324".as_bytes(), 10).unwrap();
        let f = Factor::new(p, 5);
        assert_eq!(f.pe(), BigInt::parse_bytes("242683778199500834259490681761793674473122071277452196998027316391426303843158647336337348864204968088977503838097970496463304346624".as_bytes(), 10).unwrap());
    }
}
