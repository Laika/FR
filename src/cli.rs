use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = "trial")]
    pub algorithm: String,

    #[clap(short, long, default_value = "expr")]
    pub format: String,
    // expr, python, json, c
    #[clap(short, long, default_value_t = -1)]
    pub jobs: i8,

    pub n: String,
}

pub fn parse() -> Cli {
    let args = Cli::parse();
    args
}
