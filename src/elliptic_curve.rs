use crate::galois_field::GaloisField;
use num::bigint::{BigInt};
use num::Integer;
use num_traits::{One, Zero};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Fn, FnMut, FnOnce, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    a: GaloisField,
    b: GaloisField,
    f: GaloisField,
}

impl EllipticCurve {
    pub fn new(f: &GaloisField, a: &BigInt, b: &BigInt) -> EllipticCurve {
        let (a, b) = (f.new(a), f.new(b));
        let f = f.clone();
        EllipticCurve { a, b, f }
    }

    pub fn new_point(&self, x: &BigInt, y: &BigInt) -> Point {
        let (x, y) = (self.f.new(x), self.f.new(y));
        let lhs = x.pow(&BigInt::from(3u32)).unwrap() + x.clone() * self.a.clone() + self.b.clone();
        let rhs = y.pow(&BigInt::from(2u32)).unwrap();
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
    x: GaloisField,
    y: GaloisField,
    z: GaloisField,
    curve: EllipticCurve,
    n: GaloisField,
}
impl Point {
    pub fn xy(&self) -> (BigInt, BigInt) {
        if self.z.clone() == self.curve.f.zero() {
            return (BigInt::zero(), BigInt::zero()); // TODO: Should be changed to an appropriate value?
        } else {
            (
                (self.x.clone() / self.z.clone().pow(&BigInt::from(2u32)).unwrap())
                    .unwrap()
                    .value,
                (self.y.clone() / self.z.clone().pow(&BigInt::from(3u32)).unwrap())
                    .unwrap()
                    .value,
            )
        }
    }

    pub fn x(&self) -> BigInt {
        self.xy().0
    }
    pub fn y(&self) -> BigInt {
        self.xy().1
    }
    pub fn z(&self) -> BigInt {
        self.z.value.clone()
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
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Self::Output {
        jacobian_add(self, rhs)
        //   affine_add(self, rhs)
    }
}

fn affine_add(p1: Point, p2: Point) -> Option<Point> {
    assert_eq!(p1.curve.a, p2.curve.a);
    assert_eq!(p1.curve.b, p2.curve.b);
    assert_eq!(p1.curve.f, p2.curve.f);

    let p = p1.curve.f.p.clone();
    let f = GaloisField::GaloisField(&p);
    if p1 == p1.curve.o() {
        return Some(p2);
    }
    if p2 == p1.curve.o() {
        return Some(p1);
    }

    let (x1, x2) = (p1.x, p2.x);
    let (y1, y2) = (p1.y, p2.y);

    let lambda = ((y2 - y1.clone()) / (x2.clone() - x1.clone()))?;
    let x3 = lambda.pow(&BigInt::from(2u64))? - x1.clone() - x2;
    let y3 = lambda * (x1 - x3.clone()) - y1;

    Some(Point {
        x: x3,
        y: y3,
        z: f.new(&BigInt::one()),
        curve: p1.curve,
        n: p1.n,
    })
}

fn jacobian_add(p1: Point, p2: Point) -> Option<Point> {
    assert_eq!(p1.curve.a, p2.curve.a);
    assert_eq!(p1.curve.b, p2.curve.b);
    assert_eq!(p1.curve.f.p, p2.curve.f.p);
    assert_eq!(p1.curve.f.value, p2.curve.f.value);
    let p = p1.curve.f.p.clone();
    let f = GaloisField::GaloisField(&p);
    if p1 == p1.curve.o() {
        return Some(p2);
    }
    if p2 == p1.curve.o() {
        return Some(p1);
    }
    let (x1, x2) = (p1.x, p2.x);
    let (y1, y2) = (p1.y, p2.y);
    let (z1, z2) = (p1.z, p2.z);

    let (u1, u2) = (
        x1 * z2.pow(&BigInt::from(2u32))?,
        x2 * z1.pow(&BigInt::from(2u32))?,
    );
    let (s1, s2) = (
        y1.clone() * z2.pow(&BigInt::from(3u32))?,
        y2.clone() * z1.pow(&BigInt::from(3u32))?,
    );
    let h = u2.clone() - u1.clone();
    let r = (s2.clone() - s1.clone()) * f.new(&BigInt::from(2u32));
    let i = h.pow(&BigInt::from(2u32))? * f.new(&BigInt::from(4u32));
    let (j1, j2) = (i.clone() * h.clone(), i.clone() * u1.clone());
    let x3 = r.pow(&BigInt::from(2u32))? - j1.clone() - j2.clone() * f.new(&BigInt::from(2u32));
    let y3 = (j2.clone() - x3.clone()) * r.clone()
        - s1.clone() * j1.clone() * f.new(&BigInt::from(2u32));
    let z3 = h.clone() * z1.clone() * z2 * f.new(&BigInt::from(2u32));

    let finity: bool = z3.clone() != p1.curve.f.zero();
    if finity {
        let x3 = (x3.clone() / z3.clone().pow(&BigInt::from(2u32))?)?;
        let y3 = (y3.clone() / z3.clone().pow(&BigInt::from(3u32))?)?;
        let z3 = p1.curve.f.new(&BigInt::from(finity as i32));
        Some(Point {
            x: x3,
            y: y3,
            z: z3,
            curve: p1.curve,
            n: p1.n,
        })
    } else {
        Some(p1.curve.o())
    }
}

impl Mul<Point> for BigInt {
    type Output = Option<Point>;
    fn mul(self, rhs: Point) -> Self::Output {
        scalar_mul(self, rhs)
    }
}

fn scalar_mul(k: BigInt, p: Point) -> Option<Point> {
    let mut p0: Point = p.clone();
    let mut p: Point = p.curve.o();
    let mut k: BigInt = k.clone();

    while k > BigInt::zero() {
        if k.clone() & BigInt::one() == BigInt::one() {
            p = (p + p0.clone())?;
        }
        p0 = double(p0.clone())?;
        k >>= 1u32;
    }
    Some(p)
}

pub fn scalar_mul_for_factorization(k: BigInt, p: Point) -> Option<BigInt> {
    let mut p0: Point = p.clone();
    let p: Point = p.curve.o();
    let mut k: BigInt = k.clone();

    while k > BigInt::zero() {
        if k.clone() & BigInt::one() == BigInt::one() {
            let q = p.clone() + p0.clone();
            match q {
                Some(_) => return None,
                None => {
                    return Some(p.curve.f.p.gcd(&(p.x - p0.x).value));
                }
            };
        }
        p0 = double(p0.clone())?;
        k >>= 1u32;
    }
    None
}

fn double(pp: Point) -> Option<Point> {
    let (a, p) = (pp.curve.a.clone(), pp.curve.f.p.clone());
    let f = GaloisField::GaloisField(&p);
    let (x1, y1, z1) = (pp.x, pp.y, pp.z);
    let s = ((x1.clone() + y1.pow(&BigInt::from(2u32))?).pow(&BigInt::from(2u32))?
        - x1.pow(&BigInt::from(2u32))?
        - y1.pow(&BigInt::from(4u32))?)
        * f.new(&BigInt::from(2u32));
    let m = (x1.pow(&BigInt::from(2u32))?) * f.new(&BigInt::from(3u32))
        + (z1.pow(&BigInt::from(4u32))?) * a;
    let x3 = m.pow(&BigInt::from(2u32))? - s.clone() * f.new(&BigInt::from(2u32));
    let y3 =
        (s.clone() - x3.clone()) * m - (y1.pow(&BigInt::from(4u32))?) * f.new(&BigInt::from(8u32));
    let z3 = y1 * z1 * f.new(&BigInt::from(2u32));

    let finity: bool = z3.clone() != pp.curve.f.zero();
    if finity {
        let x3 = (x3.clone() / z3.clone().pow(&BigInt::from(2u32))?)?;
        let y3 = (y3.clone() / z3.clone().pow(&BigInt::from(3u32))?)?;
        let z3 = pp.curve.f.new(&BigInt::from(finity as i32));
        Some(Point {
            x: x3,
            y: y3,
            z: z3,
            curve: pp.curve,
            n: pp.n,
        })
    } else {
        Some(pp.curve.o())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;
    #[test]

    fn test_elliptic_curve() {
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
        let k = BigInt::from(32u32);
        let e = EllipticCurve::new(&f, &a, &b);

        let x1 = bi!(
            "de2067f34a264b2f9e5a3764d70fcfea97ec50d1a5f41cd4267b95c600c58b88",
            16
        );
        let y1 = bi!(
            "469a63d604a4fd1f0125ea35eb64b590b996bf4cc7117a13ab106b090fb3404e",
            16
        );

        let x2 = bi!(
            "a48213309ba2021c3c72ebf26b7ad76531f616f644d4c3214dfa3e1ae82f35b7",
            16
        );
        let y2 = bi!(
            "6fc896d6ee86f5172bf36b33d85a6fb82712ec5e8fedc84b0e6262b7545b9a9d",
            16
        );

        let x3 = bi!(
            "cd7ab1c4a9d632f10df44921843ff4a5e7f5a6201cc81712b692d9f59a3e7ff4",
            16
        );
        let y3 = bi!(
            "f6cd5040024520de26231babe98526bdad85fd9645544a6c942bcfbe770aa921",
            16
        );

        let P = e.new_point(&x1, &y1);
        let Q = e.new_point(&x2, &y2);
        let R = e.new_point(&x3, &y3);
        assert_eq!((P.clone() + Q.clone()).unwrap(), R);

        assert_eq!((Q.clone() + Q.curve.o()).unwrap(), Q.clone());
        assert_eq!((Q.curve.o() + Q.clone()).unwrap(), Q.clone());

        assert_eq!((BigInt::from(2u32) * Q.curve.o()).unwrap(), Q.curve.o());

        let x3 = bi!(
            "ff339cace0a8015a7f693252f3f810e6f04a427d1b0cef16020325be952166c1",
            16
        );
        let y3 = bi!(
            "1a8e4912c5b1f36c475cd375ec3ac797a1fc491d4f7584ffd1234d301205d00e",
            16
        );
        let R = e.new_point(&x3, &y3);
        assert_eq!((k * P.clone()).unwrap(), R);
    }
}
