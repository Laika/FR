use crate::traits::Factorizer;
use num::BigInt;

mod algs;
mod fermat;
mod traits;

// fn ecm_factorize(n: i128) -> Vec<i128> {}
//
// fn is_prime(n: BigUint) -> bool {}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let res = algs::mod_pow(34241, 243513, 3124123);
    println!("{res:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigInt;
    #[test]

    fn test_fermat() {
        let n = BigInt::parse_bytes("895649414291294604941588381871244924626104121562042227318384494381723497514540860474803494041479529".as_bytes(), 10).unwrap();
        let ff = fermat::Fermat::new(n);
        let f = ff.factorize().unwrap();
        assert_eq!(
            f[0].pe(),
            BigInt::parse_bytes(
                "29927402397991286489627904551843385490310576382227".as_bytes(),
                10
            )
            .unwrap()
        );
        assert_eq!(
            f[1].pe(),
            BigInt::parse_bytes(
                "29927402397991286489627837734179186385188296382227".as_bytes(),
                10
            )
            .unwrap()
        );
    }
}
