use anyhow::anyhow;
use anyhow::Context;
use std::cmp::max;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


#[derive(Clone, Debug)]
pub enum Field {
    Integer(i32),
    Float(f64),
    Text(String),
}

impl Field {
    fn from_string(s: &str) -> Self {
        if let Ok(i) = s.parse::<i32>() {
            Field::Integer(i)
        } else if let Ok(f) = s.parse::<f64>() {
            Field::Float(f)
        } else {
            Field::Text(s.to_string())
        }
    }
    fn type_name(&self) -> &'static str {
        match self {
            Field::Integer(_) => "Integer",
            Field::Float(_) => "Float",
            Field::Text(_) => "Text",
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Float(n) => write!(f, "{n:?}"),
            Self::Text(t) => write!(f, "{t}"),
        }
    }
}
#[derive(Debug)]
pub struct CsvFile {
    headers: Vec<String>,
    rows: Vec<Vec<Field>>,
}
impl CsvFile {
    fn parse_line(
        row_num: usize,
        line: String,
        headers: &[String],
        types: Option<&Vec<Field>>,
    )-> Result< Vec<Field>, anyhow::Error>{

        let fields: Vec<Field> = line.split(',').map(|s| Field::from_string(s)).collect();
        if fields.len() != headers.len() {
            return Err(anyhow!(
                "La riga {} ha un numero di campi diverso dagli header: attesi {}, trovati {}",
                row_num,
                headers.len(),
                fields.len()
            ));
        }else if let Some(types) = types {
            for (i, (field, ty)) in fields.iter().zip(types).enumerate() {
                use Field::*;
                match (field, ty) {
                    (Integer(_), Integer(_)) | (Float(_), Float(_)) | (Text(_), Text(_)) => (),
                    _ => {
                        return Err(anyhow!(
                            "Mismatched type for field #{}({}) at line {}, expected {}, but got {}",
                            i + 1,
                            headers[i],
                            row_num,
                            ty.type_name(),
                            field.type_name()
                        ));
                    }
                }
            }
            Ok(fields)
        }else {
            Ok(fields)
        }

        
    }
    pub fn from_file(path: &Path) -> Result<Self, anyhow::Error> {
        let f = File::open(path).with_context(|| format!("Cannot find file {:?}", path))?;
        let mut lines = BufReader::new(f).lines().enumerate();
        let mut errors = vec![];
        if let Some((_, r)) = lines.next() {
            let s = r.with_context(|| format!("Cannot read from file {:?}", path))?;
            let headers: Vec<String> = s.split(',').map(|s| s.to_string()).collect();
            let mut fields = vec![];
            let mut types: Option<Vec<Field>> = None;
            for (n, r) in lines {
                let s =
                    r.with_context(|| format!("Cannot read line {} in file {:?}", n + 1, path))?;
                let v1 = match Self::parse_line(n + 1, s, &headers, types.as_ref()) {
                    Ok(v) => v,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };
                if types.is_none() {
                    types = Some(v1.clone());
                }
                fields.push(v1);
            }
            if errors.is_empty() {
                Ok(Self {
                    headers,
                    rows: fields,
                })
            } else {
                Err(anyhow!(
                    "Failed to parse file {:?} with {} error(s):\n{}",
                    path,
                    errors.len(),
                    errors
                        .into_iter()
                        .map(|e| format!("- {e}"))
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
            }
        } else {
            Ok(Self {
                headers: vec![],
                rows: vec![],
            })
        }
    }
}

impl Display for CsvFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut cols = vec![];
        let mut sizes = vec![];
        for (i, header) in self.headers.iter().enumerate() {
            let mut col = vec![header.clone()];
            let mut size = header.chars().count();
            for fs in self.rows.iter() {
                let s = format!("{}", fs[i]);
                size = max(size, s.chars().count());
                col.push(s);
            }
            sizes.push(size);
            cols.push(col);
        }
        if cols.is_empty() {
            write!(f, "[ Empty csv file ]")
        } else {
            for i in 0..cols[0].len() {
                for j in 0..sizes.len() {
                    let col = &cols[j];
                    let size = sizes[j];
                    if j != 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{:>size$}", col[i])?;
                }
                if i < cols[0].len() - 1 {
                    writeln!(f)?;
                }
                if i == 0 {
                    for j in 0..sizes.len() {
                        if j != 0 {
                            write!(f, "+")?;
                        }
                        let size = sizes[j] + if j == 0 { 1 } else { 2 };
                        write!(f, "{:-<size$}", "")?;
                    }
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }
}
