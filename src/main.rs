use crate::traits::Factorizer;
use num::BigInt;

mod algs;
mod cli;
mod consts;
mod ecm;
mod elliptic_curve;
mod fermat;
mod gf;
mod mac;
mod traits;
mod trial;

fn main() {
    let cli = cli::parse();
    let n_str: String = cli.n.trim().to_string();
    let n: BigInt = BigInt::parse_bytes(n_str.as_bytes(), 10).unwrap();
    let algorithm: &str = cli.algorithm.as_str();

    let factors = match algorithm {
        "trial" => trial::Trial::new(n.clone()).factorize(),
        "fermat" => fermat::Fermat::new(n.clone()).factorize(),
        _ => None,
    }
    .unwrap();

    println!("n = {factors}");
}
