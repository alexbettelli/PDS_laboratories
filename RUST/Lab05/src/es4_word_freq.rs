#![allow(dead_code)]
//! Esercizio 4 - Word frequency multi-file (mini map-reduce)
//!
//! Implementare la funzione `word_frequencies` che, dato un elenco di percorsi
//! a file di testo, calcola la frequenza globale di ciascuna parola
//! (case-insensitive) sfruttando il parallelismo tra file.
//!
//! Implementare inoltre la funzione `top_k` che estrae le `k` parole più
//! frequenti.
//!
//! Vedi il file `Lab5.md` per la specifica completa.

use std::collections::HashMap;

/// Calcola la frequenza globale delle parole presenti in tutti i file passati.
///
/// Ogni file è elaborato da un thread distinto, che costruisce una mappa locale
/// e la fonde una sola volta nella mappa globale condivisa.
pub fn word_frequencies(_paths: Vec<String>) -> HashMap<String, usize> {
    todo!("Implementare word_frequencies")
}

/// Restituisce le `k` parole più frequenti, ordinate per frequenza decrescente.
/// In caso di parità, ordine alfabetico crescente.
pub fn top_k(_freqs: &HashMap<String, usize>, _k: usize) -> Vec<(String, usize)> {
    todo!("Implementare top_k")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    /// Crea un file temporaneo con il contenuto indicato e ne restituisce il percorso.
    fn crea_file_temp(nome: &str, contenuto: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("es4_{}_{}.txt", std::process::id(), nome));
        let mut f = File::create(&path).expect("creazione file di test fallita");
        f.write_all(contenuto.as_bytes())
            .expect("scrittura fallita");
        path
    }

    #[test]
    fn frequenze_singolo_file() {
        let p = crea_file_temp("singolo", "Ciao ciao mondo");
        let paths = vec![p.to_string_lossy().to_string()];
        let f = word_frequencies(paths);
        assert_eq!(f.get("ciao"), Some(&2));
        assert_eq!(f.get("mondo"), Some(&1));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn frequenze_piu_file() {
        let p1 = crea_file_temp("a", "rust è bello rust");
        let p2 = crea_file_temp("b", "Rust va veloce");
        let paths = vec![
            p1.to_string_lossy().to_string(),
            p2.to_string_lossy().to_string(),
        ];
        let f = word_frequencies(paths);
        assert_eq!(f.get("rust"), Some(&3));
        assert_eq!(f.get("bello"), Some(&1));
        assert_eq!(f.get("veloce"), Some(&1));
        let _ = std::fs::remove_file(&p1);
        let _ = std::fs::remove_file(&p2);
    }

    #[test]
    fn file_inesistente_non_interrompe() {
        let p = crea_file_temp("ok", "uno due tre");
        let paths = vec![
            "/percorso/che/non/esiste/davvero.txt".to_string(),
            p.to_string_lossy().to_string(),
        ];
        let f = word_frequencies(paths);
        assert_eq!(f.get("uno"), Some(&1));
        assert_eq!(f.get("due"), Some(&1));
        assert_eq!(f.get("tre"), Some(&1));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn punteggiatura_e_case() {
        let p = crea_file_temp("punct", "Hello, world! HELLO world.");
        let paths = vec![p.to_string_lossy().to_string()];
        let f = word_frequencies(paths);
        assert_eq!(f.get("hello"), Some(&2));
        assert_eq!(f.get("world"), Some(&2));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn top_k_ordinamento() {
        let mut m = HashMap::new();
        m.insert("alfa".to_string(), 5);
        m.insert("beta".to_string(), 5);
        m.insert("gamma".to_string(), 10);
        m.insert("delta".to_string(), 1);

        let top = top_k(&m, 3);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0], ("gamma".to_string(), 10));
        // Parità a 5: ordine alfabetico crescente => alfa prima di beta.
        assert_eq!(top[1], ("alfa".to_string(), 5));
        assert_eq!(top[2], ("beta".to_string(), 5));
    }

    #[test]
    fn top_k_piu_grande_della_mappa() {
        let mut m = HashMap::new();
        m.insert("uno".to_string(), 1);
        m.insert("due".to_string(), 2);
        let top = top_k(&m, 10);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "due");
    }

    #[test]
    fn lista_di_path_vuota() {
        let f = word_frequencies(vec![]);
        assert!(f.is_empty());
    }

    #[test]
    fn file_vuoto() {
        let p = crea_file_temp("vuoto", "");
        let paths = vec![p.to_string_lossy().to_string()];
        let f = word_frequencies(paths);
        assert!(f.is_empty());
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn tutti_i_file_inesistenti() {
        // Nessun file leggibile: il programma non deve panicare e deve
        // restituire una mappa vuota.
        let paths = vec![
            "/percorso/inesistente/a.txt".to_string(),
            "/percorso/inesistente/b.txt".to_string(),
        ];
        let f = word_frequencies(paths);
        assert!(f.is_empty());
    }

    #[test]
    fn frequenze_su_piu_righe() {
        let p = crea_file_temp(
            "multi_riga",
            "alfa beta\nbeta gamma\nalfa\n",
        );
        let paths = vec![p.to_string_lossy().to_string()];
        let f = word_frequencies(paths);
        assert_eq!(f.get("alfa"), Some(&2));
        assert_eq!(f.get("beta"), Some(&2));
        assert_eq!(f.get("gamma"), Some(&1));
        assert_eq!(f.len(), 3);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn parole_alfanumeriche() {
        // I caratteri non alfanumerici fanno da separatore: "abc123" è
        // un'unica parola, "abc-123" diventa due.
        let p = crea_file_temp("alnum", "abc123 abc-123\nabc123");
        let paths = vec![p.to_string_lossy().to_string()];
        let f = word_frequencies(paths);
        assert_eq!(f.get("abc123"), Some(&2));
        assert_eq!(f.get("abc"), Some(&1));
        assert_eq!(f.get("123"), Some(&1));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn top_k_zero() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 1);
        m.insert("b".to_string(), 2);
        assert!(top_k(&m, 0).is_empty());
    }

    #[test]
    fn top_k_su_mappa_vuota() {
        let m: HashMap<String, usize> = HashMap::new();
        assert!(top_k(&m, 5).is_empty());
    }

    #[test]
    fn top_k_ordine_decrescente_completo() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 3);
        m.insert("b".to_string(), 7);
        m.insert("c".to_string(), 1);
        m.insert("d".to_string(), 5);
        let top = top_k(&m, 4);
        let counts: Vec<usize> = top.iter().map(|(_, c)| *c).collect();
        // Devono essere ordinate strettamente in modo decrescente.
        for w in counts.windows(2) {
            assert!(w[0] >= w[1]);
        }
        assert_eq!(top[0], ("b".to_string(), 7));
        assert_eq!(top[3], ("c".to_string(), 1));
    }

    #[test]
    fn fusione_mappa_globale_e_parallelismo() {
        // Stress: molti file di piccola dimensione, ognuno elaborato in
        // parallelo. La somma globale per parola deve essere coerente con
        // la somma sequenziale.
        let mut paths = Vec::new();
        for i in 0..16 {
            let contenuto = format!("alfa beta gamma\nalfa alfa\n{} delta", i);
            paths.push(crea_file_temp(&format!("parallel_{}", i), &contenuto));
        }
        let path_strings: Vec<String> = paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        let f = word_frequencies(path_strings);
        // Ogni file contribuisce con: alfa x3, beta x1, gamma x1, delta x1.
        assert_eq!(f.get("alfa"), Some(&(3 * 16)));
        assert_eq!(f.get("beta"), Some(&16));
        assert_eq!(f.get("gamma"), Some(&16));
        assert_eq!(f.get("delta"), Some(&16));
        for p in paths {
            let _ = std::fs::remove_file(&p);
        }
    }
}
