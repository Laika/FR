use crate::traits::Factorizer;
use num::BigInt;

mod algs;
mod consts;
mod elliptic_curve;
mod fermat;
mod gf;
mod traits;
mod trial;

fn main() {
    let mut n_str: String = String::new();
    std::io::stdin().read_line(&mut n_str).unwrap();
    let n = BigInt::parse_bytes(n_str.trim().as_bytes(), 10).unwrap();

    let fermat = fermat::Fermat::new(n.clone());
    let res_fermat = fermat.factorize().unwrap();
    println!("Factorized by Fermat's method: {res_fermat}");

    let trial = trial::Trial::new(n.clone());
    let res_trial = trial.factorize().unwrap();
    println!("Factorized by    trial method: {res_trial}");
}
