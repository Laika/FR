use crate::gf::GF;
use num::bigint::{BigInt, Sign};
use num::Integer;
use num::ToPrimitive;
use num_bigint::{RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use std::ops::{Add, Div, Fn, FnMut, FnOnce, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    a: GF,
    b: GF,
    f: GF,
}

#[derive(Debug, Clone)]
pub struct Point {
    x: GF,
    y: GF,
    z: GF,
    curve: EllipticCurve,
    n: GF,
}
impl EllipticCurve {
    pub fn new(f: &GF, a: &BigInt, b: &BigInt) -> EllipticCurve {
        let (a, b) = (f.new(a), f.new(b));
        let f = f.clone();
        EllipticCurve { a, b, f }
    }

    pub fn new_point(&self, x: &BigInt, y: &BigInt) -> Point {
        let (x, y) = (self.f.new(x), self.f.new(y));
        Point {
            x,
            y,
            z: self.f.one(),
            curve: self.clone(),
            n: self.f.new(&BigInt::from(-1i32)),
        }
    }

    #[allow(non_snake_name)]
    pub fn O(&self) -> Point {
        Point {
            x: self.f.new(&BigInt::one()),
            y: self.f.new(&BigInt::one()),
            z: self.f.new(&BigInt::zero()),
            curve: self.clone(),
            n: self.f.new(&BigInt::from(-1i32)),
        }
    }
}

impl Point {
    pub fn xy(&self) -> (GF, GF) {
        (self.x.clone(), self.y.clone())
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert_eq!(self.curve.a, rhs.curve.a);
        assert_eq!(self.curve.b, rhs.curve.b);
        assert_eq!(self.curve.f.p, rhs.curve.f.p);
        assert_eq!(self.curve.f.value, rhs.curve.f.value);
        let p = self.curve.f.p.clone();
        let f = GF::GF(&p);
        let (x1, x2) = (self.x, rhs.x);
        let (y1, y2) = (self.y, rhs.y);
        let (z1, z2) = (self.z, rhs.z);

        let (u1, u2) = (
            x1 * z2.pow(&BigInt::from(2u32)),
            x2 * z1.pow(&BigInt::from(2u32)),
        );
        let (s1, s2) = (
            y1 * z2.pow(&BigInt::from(3u32)),
            y2 * z1.pow(&BigInt::from(3u32)),
        );
        let h = u2.clone() - u1.clone();
        let r = (s2 - s1.clone()) * f.new(&BigInt::from(2u32));
        let i = h.pow(&BigInt::from(2u32)) * f.new(&BigInt::from(4u32));
        let (j1, j2) = (i.clone() * h.clone(), i.clone() * u1);
        let x3 = r.pow(&BigInt::from(2u32)) - j1.clone() - j2.clone() * f.new(&BigInt::from(2u32));
        let y3 = r * (j2 - x3.clone()) - s1 * j1 * f.new(&BigInt::from(2u32));
        let z3 = h * z1.pow(&BigInt::from(2u32)) * f.new(&BigInt::from(2u32));

        Self {
            x: x3,
            y: y3,
            z: z3,
            curve: self.curve,
            n: self.n,
        }
    }
}

fn scalar_mul(k: BigInt, P: Point) -> Point {
    let mut P: Point = P.clone();
    let mut Q: Point = P.curve.O();
    let mut k: BigInt = k.clone();

    while k > BigInt::zero() {
        if k.clone() & BigInt::one() == BigInt::one() {
            P = P + Q.clone();
        }
        Q = double(Q.clone());
        k >>= 1u32;
    }
    P
}

fn double(P: Point) -> Point {
    let (a, p) = (P.curve.a.clone(), P.curve.f.p.clone());
    let f = GF::GF(&p);
    let (x1, y1, z1) = (P.x, P.y, P.z);
    let s = (x1.clone() + y1.pow(&BigInt::from(2u32))).pow(&BigInt::from(2u32))
        * f.new(&BigInt::from(2u32))
        - x1.pow(&BigInt::from(2u32))
        - y1.pow(&BigInt::from(4u32));
    let m =
        x1.pow(&BigInt::from(2u32)) * f.new(&BigInt::from(3u32)) + z1.pow(&BigInt::from(4u32)) * a;
    let x3 = m.pow(&BigInt::from(2u32)) - f.new(&BigInt::from(2u32)) * s.clone();
    let y3 =
        (s.clone() - x3.clone()) * m - y1.pow(&BigInt::from(4u32)) * f.new(&BigInt::from(8u32));
    let z3 = y1 * z1 * f.new(&BigInt::from(2u32));
    Point {
        x: x3,
        y: y3,
        z: z3,
        curve: P.curve,
        n: P.n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;
    #[test]

    fn test_elliptic_curve() {
        let p = BigInt::parse_bytes(
            "ffc1b7ccb0ce84ef5717b481d3dac3a061a6045e385b319e28154b9a2edfc7bb".as_bytes(),
            16,
        )
        .unwrap();
        let a = BigInt::parse_bytes(
            "13d2a791a7afc7bad0bdb8eaa49c5a34907b689c4208ce107abb404dbec9f146".as_bytes(),
            16,
        )
        .unwrap();
        let b = BigInt::parse_bytes(
            "474975c4ec852b4fff65ec2d149580f9601cf8a299dd5c15a112a28cdac448a9".as_bytes(),
            16,
        )
        .unwrap();
        let x1 = BigInt::parse_bytes(
            "de2067f34a264b2f9e5a3764d70fcfea97ec50d1a5f41cd4267b95c600c58b88".as_bytes(),
            16,
        )
        .unwrap();
        let y1 = BigInt::parse_bytes(
            "469a63d604a4fd1f0125ea35eb64b590b996bf4cc7117a13ab106b090fb3404e".as_bytes(),
            16,
        )
        .unwrap();

        let x2 = BigInt::parse_bytes(
            "a48213309ba2021c3c72ebf26b7ad76531f616f644d4c3214dfa3e1ae82f35b7".as_bytes(),
            16,
        )
        .unwrap();
        let y2 = BigInt::parse_bytes(
            "156202e5ba08124e81f9220eb6d22f975b3ea313d651b175b93ccc0750c93ca5".as_bytes(),
            16,
        )
        .unwrap();

        let f = GF::GF(&p);
        let e = EllipticCurve::new(&f, &a, &b);
        let P = e.new_point(&x1, &y1);
        let Q = e.new_point(&x2, &y2);
        let R = P + Q;

        println!("{R:?}");
    }
}
