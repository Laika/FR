use clap::{ValueEnum, Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, value_enum, default_value_t = Algorithm::Trial)]
    pub algorithm: Algorithm,

    #[clap(short, long, value_enum, default_value_t = OutputFormat::Expr)]
    pub output_format: OutputFormat,

    #[clap(short, long, default_value_t = -1)]
    pub jobs: i8,

    pub n: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algorithm {
    Auto,
    Trial,
    ECM,
    Fermat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Expr,
    FlatList,
    List,
    Json,
}

pub fn parse() -> Cli {
    let args = Cli::parse();
    args
}
