use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CsvData {
    pub rows: usize,
    pub lines: Vec<String>,
}
pub fn read_csv(file_path: &str) -> Result<CsvData, String>{
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Impossibile aprire il file '{}': {}", file_path, e)),
    };

    let mut lines = Vec::new();
    let reader =  BufReader::new(file).lines();

    let rows = lines.len();
    for (line_number,line) in reader.enumerate() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => return Err(format!("Errore nella lettura della riga {line_number}: {e}").to_string())
        }
    }
    Ok(CsvData{rows, lines})
}
pub fn print_head(data: &CsvData, head:usize) {
    println!("rows: {}", data.rows);
    println!("Head ({head}):");
    for i in 0..head{
        println!("{}", data.lines[i]);
    }
}
