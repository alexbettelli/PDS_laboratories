
use std::env::args;
use std::process::exit;
mod cli;
mod io;

fn main() {
    let args: Vec<String> = args().collect();

    let parsed_csv = match cli::parse_csv(args) {
        Ok(csv) => csv,
        Err(e) => {
            eprintln!("Error {e}");
            exit(1)
        }
    };

    let csv_data = match io::read_csv(parsed_csv.file_path.as_str()) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error {e}");
            exit(2)
        }
    };
    io::print_head(&csv_data, parsed_csv.head);
}
