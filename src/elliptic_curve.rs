use crate::gf::GF;
use num::bigint::{BigInt, Sign};
use num::Integer;
use num::ToPrimitive;
use num_bigint::{RandBigInt, ToBigInt};
use num_traits::{One, Zero};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Fn, FnMut, FnOnce, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    a: GF,
    b: GF,
    f: GF,
}

impl EllipticCurve {
    pub fn new(f: &GF, a: &BigInt, b: &BigInt) -> EllipticCurve {
        let (a, b) = (f.new(a), f.new(b));
        let f = f.clone();
        EllipticCurve { a, b, f }
    }

    pub fn new_point(&self, x: &BigInt, y: &BigInt) -> Point {
        let (x, y) = (self.f.new(x), self.f.new(y));
        let lhs = x.pow(&BigInt::from(3u32)) + x.clone() * self.a.clone() + self.b.clone();
        let rhs = y.pow(&BigInt::from(2u32));
        assert_eq!(lhs, rhs);
        Point {
            x,
            y,
            z: self.f.one(),
            curve: self.clone(),
            n: self.f.new(&BigInt::from(-1i32)),
        }
    }

    pub fn o(&self) -> Point {
        Point {
            x: self.f.one(),
            y: self.f.one(),
            z: self.f.zero(),
            curve: self.clone(),
            n: self.f.new(&BigInt::from(-1i32)),
        }
    }
}

impl PartialEq for EllipticCurve {
    fn eq(&self, rhs: &Self) -> bool {
        self.a == rhs.a && self.b == rhs.b && self.f == rhs.f
    }
}

impl Display for EllipticCurve {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "a = {}\nb = {}\nField: {}", self.a, self.b, self.f.p)
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    x: GF,
    y: GF,
    z: GF,
    curve: EllipticCurve,
    n: GF,
}
impl Point {
    pub fn xy(&self) -> (GF, GF) {
        if self.z.clone() == self.curve.f.zero() {
            return (self.curve.f.zero(), self.curve.f.zero());
        } else {
            (
                self.x.clone() / self.z.clone().pow(&BigInt::from(2u32)),
                self.y.clone() / self.z.clone().pow(&BigInt::from(3u32)),
            )
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "({} : {} : {})",
            self.x.clone(),
            self.y.clone(),
            self.z.clone(),
        )
    }
}

impl PartialEq for Point {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z && self.curve == rhs.curve
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
        if self == self.curve.o() {
            return rhs;
        }
        if rhs == self.curve.o() {
            return self;
        }
        let (x1, x2) = (self.x, rhs.x);
        let (y1, y2) = (self.y, rhs.y);
        let (z1, z2) = (self.z, rhs.z);

        let (u1, u2) = (
            x1 * z2.pow(&BigInt::from(2u32)),
            x2 * z1.pow(&BigInt::from(2u32)),
        );
        let (s1, s2) = (
            y1.clone() * z2.pow(&BigInt::from(3u32)),
            y2.clone() * z1.pow(&BigInt::from(3u32)),
        );
        let h = u2.clone() - u1.clone();
        let r = (s2.clone() - s1.clone()) * f.new(&BigInt::from(2u32));
        let i = h.pow(&BigInt::from(2u32)) * f.new(&BigInt::from(4u32));
        let (j1, j2) = (i.clone() * h.clone(), i.clone() * u1.clone());
        let x3 = r.pow(&BigInt::from(2u32)) - j1.clone() - j2.clone() * f.new(&BigInt::from(2u32));
        let y3 = (j2.clone() - x3.clone()) * r.clone()
            - s1.clone() * j1.clone() * f.new(&BigInt::from(2u32));
        let z3 = h.clone() * z1.clone() * z2 * f.new(&BigInt::from(2u32));

        let finity: bool = z3.clone() != self.curve.f.zero();
        if !finity {
            self.curve.o()
        } else {
            let x3 = x3.clone() / z3.clone().pow(&BigInt::from(2u32));
            let y3 = y3.clone() / z3.clone().pow(&BigInt::from(3u32));
            let z3 = self.curve.f.new(&BigInt::from(finity as i32));
            Self {
                x: x3,
                y: y3,
                z: z3,
                curve: self.curve,
                n: self.n,
            }
        }
    }
}

impl Mul<Point> for BigInt {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        scalar_mul(self, rhs)
    }
}

fn scalar_mul(k: BigInt, P: Point) -> Point {
    let mut p: Point = P.curve.o();
    let mut p0: Point = P.clone();
    let mut k: BigInt = k.clone();

    while k > BigInt::zero() {
        println!("P  {p}");
        println!("p0 {p0}");
        println!("k  {k}");
        if k.clone() & BigInt::one() == BigInt::one() {
            println!("--===========");
            p = p + p0.clone();
        }
        p0 = double(p0.clone());
        k >>= 1u32;
    }
    p
}

fn double(pp: Point) -> Point {
    let (a, p) = (pp.curve.a.clone(), pp.curve.f.p.clone());
    let f = GF::GF(&p);
    let (x1, y1, z1) = (pp.x, pp.y, pp.z);
    let s = ((x1.clone() + y1.pow(&BigInt::from(2u32))).pow(&BigInt::from(2u32))
        - x1.pow(&BigInt::from(2u32))
        - y1.pow(&BigInt::from(4u32)))
        * f.new(&BigInt::from(2u32));
    println!("{s}");
    let m = (x1.pow(&BigInt::from(2u32))) * f.new(&BigInt::from(3u32))
        + (z1.pow(&BigInt::from(4u32))) * a;
    let x3 = m.pow(&BigInt::from(2u32)) - s.clone() * f.new(&BigInt::from(2u32));
    let y3 =
        (s.clone() - x3.clone()) * m - (y1.pow(&BigInt::from(4u32))) * f.new(&BigInt::from(8u32));
    let z3 = y1 * z1 * f.new(&BigInt::from(2u32));

    let finity: bool = z3.clone() != pp.curve.f.zero();
    if !finity {
        pp.curve.o()
    } else {
        let x3 = x3.clone() / z3.clone().pow(&BigInt::from(2u32));
        let y3 = y3.clone() / z3.clone().pow(&BigInt::from(3u32));
        let z3 = pp.curve.f.new(&BigInt::from(finity as i32));
        Point {
            x: x3,
            y: y3,
            z: z3,
            curve: pp.curve,
            n: pp.n,
        }
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

        let f = GF::GF(&p);
        let k = BigInt::from(32u32);
        let e = EllipticCurve::new(&f, &a, &b);

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
            "6fc896d6ee86f5172bf36b33d85a6fb82712ec5e8fedc84b0e6262b7545b9a9d".as_bytes(),
            16,
        )
        .unwrap();

        let x3 = BigInt::parse_bytes(
            "cd7ab1c4a9d632f10df44921843ff4a5e7f5a6201cc81712b692d9f59a3e7ff4".as_bytes(),
            16,
        )
        .unwrap();
        let y3 = BigInt::parse_bytes(
            "f6cd5040024520de26231babe98526bdad85fd9645544a6c942bcfbe770aa921".as_bytes(),
            16,
        )
        .unwrap();

        let P = e.new_point(&x1, &y1);
        let Q = e.new_point(&x2, &y2);
        let R = e.new_point(&x3, &y3);
        assert_eq!(P.clone() + Q.clone(), R);

        assert_eq!(Q.clone() + Q.curve.o(), Q.clone());
        assert_eq!(Q.curve.o() + Q.clone(), Q.clone());

        assert_eq!(BigInt::from(2u32) * Q.curve.o(), Q.curve.o());

        let x3 = BigInt::parse_bytes(
            "ff339cace0a8015a7f693252f3f810e6f04a427d1b0cef16020325be952166c1".as_bytes(),
            16,
        )
        .unwrap();
        let y3 = BigInt::parse_bytes(
            "1a8e4912c5b1f36c475cd375ec3ac797a1fc491d4f7584ffd1234d301205d00e".as_bytes(),
            16,
        )
        .unwrap();
        let R = e.new_point(&x3, &y3);
        let Q = k.clone() * P.clone();
        println!("target {R}");
        println!("k*P    {Q}");
        assert_eq!(k * P.clone(), R);
    }
}
