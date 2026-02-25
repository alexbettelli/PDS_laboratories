pub struct ParsedCsv {
    pub file_path: String,
    pub head : usize,
}
pub fn parse_csv(args: Vec<String>) -> Result<ParsedCsv, String>{
    if args.len() < 2 {
        return Err("È obbligatoria la presenza di un file csv come parametro".to_string());
    }

    let file_path = (&args[1]).to_string();
    let mut head = 10;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--head" => {
                i += 1;
                if i >= args.len() {
                    return Err("Necessario un parametro numerico per il comando --head. Utilizzare il formato [--head <N>]".to_string())
                }
                match args[i].parse::<isize>() {
                    Ok(h) => if h < 0 { return Err("Valore numerico maggiore di 0 necessario per N".to_string()) } else { head = h as usize },
                    Err(e) => {return Err(format!("N non è un intero. Erroe {e}").to_string())}
                };
            }
            _ => return Err("Parametro non riconosciuto. Utilizzare il formato [--head <N>]".to_string()),
        }
        i += 1;
    }
    Ok( ParsedCsv {file_path, head})

}