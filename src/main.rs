use crate::cli::{Algorithm, OutputFormat};
use crate::traits::Factorizer;
use num::BigInt;

mod algs;
mod cli;
mod consts;
mod ecm;
mod elliptic_curve;
mod fermat;
mod galois_field;
mod mac;
mod traits;
mod trial;

fn main() {
    let cli = cli::parse();
    let n_str: String = cli.n.trim().to_string();
    let n: BigInt = BigInt::parse_bytes(n_str.as_bytes(), 10).unwrap();

    let factors = match cli.algorithm {
        Algorithm::Trial => trial::Trial::new(n).factorize(),
        Algorithm::Fermat => fermat::Fermat::new(n).factorize(),
        Algorithm::ECM => ecm::ECM::new(n).factorize(),
        _ => None,
    }.unwrap();

    let result = match cli.output_format {
        OutputFormat::List => factors.get_factors_list(),
        OutputFormat::FlatList => factors.get_factors_flat_list(),
        OutputFormat::Expr => factors.get_factors_expr(),
        _ => format!("{factors}"),
    };

    println!("n = {result}");
}
