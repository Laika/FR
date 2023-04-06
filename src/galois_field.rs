use crate::bi;
use num::bigint::{BigInt, Sign};
use num::Integer;
use num_traits::{One, Zero};
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Default, Hash)]
pub struct GaloisField {
    pub p: BigInt,
    pub value: BigInt,
}
impl GaloisField {
    #[allow(non_snake_case)]
    pub fn GaloisField(p: &BigInt) -> Self {
        Self {
            p: p.clone(),
            value: BigInt::zero(),
        }
    }

    pub fn new(&self, v: &BigInt) -> Self {
        let p = &self.p;
        if &BigInt::zero() <= v && v < p {
            return Self {
                p: self.p.clone(),
                value: v.clone(),
            };
        }
        match v.sign() {
            Sign::Plus => Self {
                p: p.clone(),
                value: (v % p).clone(),
            },
            Sign::NoSign => Self {
                p: p.clone(),
                value: v.clone(),
            },
            Sign::Minus => Self {
                p: p.clone(),
                value: ((-v + p - &BigInt::one()) / p * p + v) % p,
            },
        }
    }
    pub fn p(&self) -> BigInt {
        self.p.clone()
    }

    pub fn zero(&self) -> Self {
        self.new(&BigInt::zero())
    }
    pub fn one(&self) -> Self {
        self.new(&BigInt::one())
    }

    pub fn inv(&self) -> Option<Self> {
        let res = self.value.extended_gcd(&self.p);
        if self.value == bi!("599", 10) {
            println!("{self} {res:?}");
        }
        if self.value < bi!("1000", 10) {
            println!("{self}");
        }
        if res.gcd == BigInt::one() {
            Some(self.new(&res.x))
        } else {
            None
        }
    }

    pub fn pow(&self, e: &BigInt) -> Option<Self> {
        match e.sign() {
            Sign::Minus => Self {
                p: self.p.clone(),
                value: self.value.modpow(&-e, &self.p),
            }
            .inv(),
            _ => Some(Self {
                p: self.p.clone(),
                value: self.value.modpow(&e, &self.p),
            }),
        }
    }
}

impl Display for GaloisField {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for GaloisField {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.value)
    }
}

impl PartialEq for GaloisField {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value && self.p == rhs.p
    }
}

impl Neg for GaloisField {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            value: self.p.clone() - self.value,
            p: self.p,
        }
    }
}

impl Add for GaloisField {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        let p = self.p;
        let mut value = self.value + rhs.value;
        if value >= p.clone() {
            value -= p.clone();
        }

        Self { p, value }
    }
}

impl Sub for GaloisField {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for GaloisField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.p, rhs.p);
        let p = self.p;
        let value = self.value * rhs.value % p.clone();

        Self { p, value }
    }
}

impl Div for GaloisField {
    type Output = Option<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        let rinv = rhs.inv()?;
        Some(self * rinv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bi;
    use num::bigint::BigInt;
    use num_bigint::ToBigInt;
    #[test]

    fn test_gf() {
        let p = bi!(
            "ffc1b7ccb0ce84ef5717b481d3dac3a061a6045e385b319e28154b9a2edfc7bb",
            16
        );
        let a = bi!(
            "13d2a791a7afc7bad0bdb8eaa49c5a34907b689c4208ce107abb404dbec9f146",
            16
        );
        let b = bi!(
            "474975c4ec852b4fff65ec2d149580f9601cf8a299dd5c15a112a28cdac448a9",
            16
        );

        let f = GaloisField::GaloisField(&p);

        let x: BigInt = -BigInt::one();
        let y: BigInt = p.clone() - BigInt::one();
        assert_eq!(f.new(&x), f.new(&y));

        let x: BigInt = -8917389123_i64.to_bigint().unwrap();
        let y: BigInt = (p.clone() - 8917389123_i64).to_bigint().unwrap();
        assert_eq!(f.new(&x), f.new(&y));

        let x: BigInt = BigInt::zero();
        let y: BigInt = p.clone();
        assert_eq!(f.new(&x), f.new(&y));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let z: BigInt = bi!(
            "5b1c1d569434f30ad023a517b931db2df098613edbe62a261bcde2da998e39ef",
            16
        );

        assert_eq!(x + y, f.new(&z));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let z: BigInt = bi!(
            "5b1c1d569434f30ad023a517b931db2df098613edbe62a261bcde2da998e39ef",
            16
        );
        assert_eq!(x + y, f.new(&z));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let z: BigInt = bi!(
            "cc4ae9996bf9215a286f813f63e19cdb92047457e086a39901bde95b12e57058",
            16
        );
        assert_eq!(x - y, f.new(&z));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let z: BigInt = bi!(
            "83f586495da900cdee6da208c9720d437398a9b8b6ca067d52ec0e1f7cdf029c",
            16
        );
        assert_eq!(x * y, f.new(&z));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let z: BigInt = bi!(
            "ed30030ae80f2255c2acf73d01c3c9a41302928d248c7ceef9e95231b821effe",
            16
        );

        let x: GaloisField = f.new(&a);
        let y: &BigInt = &b;
        let z: BigInt = bi!(
            "20410ee973c22010125f6a3e8f0b369f9dabb3fb439f4f5478fc91102c5061e3",
            16
        );

        let x: GaloisField = f.new(&a);
        let y: &BigInt = &b;
        let z: BigInt = bi!(
            "93983616857e67ce830aa4af9c2dfd67f2e52dbd6dda4b0cb43c3ae9c56d064b",
            16
        );
        assert_eq!(x.pow(&-y).unwrap(), f.new(&z));

        let x: GaloisField = f.new(&a);
        let y: GaloisField = f.new(&b);
        let n1: &BigInt = &-BigInt::one();

        assert_eq!(x.pow(&n1), x.inv());
        assert_eq!(y.pow(&n1), y.inv());
    }
}
