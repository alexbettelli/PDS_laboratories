use std::path::Path;

use clap::Parser;

mod cli;
mod csv;

fn main() {
    let args = match cli::Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error parsing arguments: {}", err);
            std::process::exit(1);
        }
    };

    let csv = match csv::CsvFile::from_file(Path::new(&args.input)) {
        Ok(csv) => csv,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(5);
        }
    };
    println!("{}", csv);
}
