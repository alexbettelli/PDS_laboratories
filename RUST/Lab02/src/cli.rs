use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub input: String,
    #[clap(long, default_value = "10")]
    pub head: usize,
}
