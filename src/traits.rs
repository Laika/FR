use num::BigInt;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::ops::Index;

// Factorizer
pub trait Factorizer {
    fn factorize(&self) -> Option<Factors>;
}

// Factor
#[derive(Debug, Clone, Eq)]
pub struct Factor {
    p: BigInt,
    e: u32,
}

impl Factor {
    pub fn new(prime: BigInt, exp: u32) -> Self {
        Self { p: prime, e: exp }
    }
    pub fn q(&self) -> BigInt {
        let p = self.p.clone();
        p.pow(self.e)
    }
    pub fn pe(&self) -> (BigInt, u32) {
        (self.p.clone(), self.e.clone())
    }
}
impl Display for Factor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.e == 1u32 {
            write!(f, "{}^{}", self.p, self.e)
        } else {
            write!(f, "{}", self.p)
        }
    }
}
impl Ord for Factor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p.cmp(&other.p)
    }
}
impl PartialOrd for Factor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.p.cmp(&other.p))
    }
}
impl PartialEq for Factor {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

// Factors
#[derive(Debug, Clone)]
pub struct Factors {
    factors: HashMap<BigInt, u32>,
}
impl Factors {
    pub fn new(factors: Option<HashMap<BigInt, u32>>) -> Self {
        match factors {
            Some(fs) => Self { factors: fs },
            _ => Self {
                factors: HashMap::new(),
            },
        }
    }
    pub fn n(&self) -> BigInt {
        self.factors.iter().map(|(p, e)| p.pow(*e)).product()
    }

    pub fn add(&mut self, p: BigInt) {
        if self.factors.contains_key(&p) {
            if let Some(e) = self.factors.get_mut(&p) {
                *e += 1;
            }
        } else {
            self.factors.insert(p, 1);
        }
    }
    pub fn get_factors(&self) -> HashMap<BigInt, u32> {
        self.factors.clone()
    }
    pub fn get_factors_vector(&self) -> Vec<Factor> {
        let mut v = Vec::<Factor>::new();
        for (p, e) in self.factors.iter() {
            v.push(Factor {
                p: p.clone(),
                e: e.clone(),
            })
        }
        v.sort();

        v
    }
}
impl Display for Factors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut fs: Vec<(&BigInt, &u32)> = Vec::from_iter(self.factors.iter());
        fs.sort_by(|a, b| a.0.cmp(&b.0));
        write!(
            f,
            "{}",
            fs.iter()
                .map(|(p, e)| {
                    if **e != 1_u32 {
                        format!("{p}^{e}")
                    } else {
                        format!("{p}")
                    }
                })
                .collect::<Vec<_>>()
                .join(" * ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigInt;

    #[test]
    fn test_factor() {
        let p = BigInt::parse_bytes("189237912873428934298749324".as_bytes(), 10).unwrap();
        let f = Factor::new(p, 5);
        assert_eq!(f.q(), BigInt::parse_bytes("242683778199500834259490681761793674473122071277452196998027316391426303843158647336337348864204968088977503838097970496463304346624".as_bytes(), 10).unwrap());
    }
}
