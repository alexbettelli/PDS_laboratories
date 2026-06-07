#![allow(dead_code)]
//! Esercizio 3 - Pipeline a stadi con canali `mpsc`
//!
//! Realizzare una pipeline a tre stadi (Generator -> Transformer -> Collector)
//! che comunicano tra loro tramite canali `std::sync::mpsc`.
//!
//! Vedi il file `Lab6.md` per la specifica completa.

/// Esegue la pipeline producendo i numeri da 1 a `n`, calcolandone il quadrato
/// nel secondo stadio e raccogliendoli nel terzo stadio.
///
/// L'ordine degli elementi nel vettore restituito deve coincidere con quello
/// di produzione, ovvero `[1, 4, 9, 16, ..., n*n]`.
pub fn run_pipeline(_n: i64) -> Vec<i64> {
    todo!("Implementare run_pipeline")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_zero_elementi() {
        let v = run_pipeline(0);
        assert!(v.is_empty());
    }

    #[test]
    fn pipeline_pochi_elementi() {
        let v = run_pipeline(5);
        assert_eq!(v, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn pipeline_molti_elementi() {
        let n = 1000;
        let v = run_pipeline(n);
        assert_eq!(v.len(), n as usize);
        for (i, x) in v.iter().enumerate() {
            let k = (i + 1) as i64;
            assert_eq!(*x, k * k);
        }
    }
}
