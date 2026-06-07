#![allow(dead_code)]
//! Esercizio 5 - Fan-out / fan-in con `crossbeam::channel`
//!
//! Calcola in parallelo, su una grande lista di numeri, una funzione costosa
//! (simulata da una somma di cifre con un piccolo `thread::sleep`), usando
//! canali MPMC di `crossbeam_channel` per distribuire i task ai worker e
//! raccogliere i risultati in un thread aggregator.
//!
//! Vedi il file `Lab6.md` per la specifica completa.

/// Calcola, per ciascun numero in `input`, la somma delle sue cifre in base 10,
/// usando `n_workers` thread che consumano da un canale crossbeam MPMC.
///
/// L'ordine del vettore restituito non è specificato.
pub fn fan_out_fan_in(_input: Vec<i64>, _n_workers: usize) -> Vec<(i64, u32)> {
    todo!("Implementare fan_out_fan_in")
}

pub fn somma_cifre(x: i64) -> u32 {
    let mut n = x.unsigned_abs();
    let mut s: u32 = 0;
    while n > 0 {
        s += (n % 10) as u32;
        n /= 10;
    }
    // Piccolo sleep per simulare un calcolo costoso.
    std::thread::sleep(std::time::Duration::from_micros(100));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn input_vuoto() {
        let v = fan_out_fan_in(vec![], 4);
        assert!(v.is_empty());
    }

    #[test]
    fn risultati_corretti_pochi_elementi() {
        let input = vec![0, 5, 12, 99, 123];
        let v = fan_out_fan_in(input.clone(), 2);
        assert_eq!(v.len(), input.len());
        let map: HashMap<i64, u32> = v.into_iter().collect();
        assert_eq!(map[&0], 0);
        assert_eq!(map[&5], 5);
        assert_eq!(map[&12], 3);
        assert_eq!(map[&99], 18);
        assert_eq!(map[&123], 6);
    }

    #[test]
    fn risultati_corretti_molti_elementi() {
        let input: Vec<i64> = (0..500).collect();
        let v = fan_out_fan_in(input.clone(), 8);
        assert_eq!(v.len(), input.len());

        let map: HashMap<i64, u32> = v.into_iter().collect();
        for x in &input {
            let mut atteso: u32 = 0;
            let mut n = x.unsigned_abs();
            while n > 0 {
                atteso += (n % 10) as u32;
                n /= 10;
            }
            assert_eq!(map.get(x).copied(), Some(atteso), "x = {x}");
        }
    }

    #[test]
    fn nessun_elemento_duplicato_o_perso() {
        // Anche con molti worker, ogni input deve essere processato
        // esattamente una volta.
        let input: Vec<i64> = (1..=1000).collect();
        let v = fan_out_fan_in(input.clone(), 16);
        assert_eq!(v.len(), input.len());

        let mut chiavi: Vec<i64> = v.iter().map(|(k, _)| *k).collect();
        chiavi.sort();
        assert_eq!(chiavi, input);
    }
}
