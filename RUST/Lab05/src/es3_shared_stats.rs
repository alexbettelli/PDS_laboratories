#![allow(dead_code)]
//! Esercizio 3 - Statistiche concorrenti con `RwLock`
//!
//! Implementare la struct `SharedStats` che incapsula un `RwLock<Stats>`
//! e fornisce metodi per aggiornare e interrogare le statistiche.
//!
//! Vedi il file `Lab5.md` per la specifica completa.

pub struct SharedStats {
}

impl SharedStats {
    pub fn new() -> Self {
        todo!("Implementare new")
    }

    /// Aggiunge un campione, aggiornando count, sum, min e max.
    pub fn add_sample(&self, _value: f64) {
        todo!("Implementare add_sample")
    }

    pub fn count(&self) -> usize {
        todo!("Implementare count")
    }

    /// Restituisce la media dei campioni inseriti, oppure `None` se non ce ne sono.
    pub fn mean(&self) -> Option<f64> {
        todo!("Implementare mean")
    }

    pub fn min(&self) -> Option<f64> {
        todo!("Implementare min")
    }

    pub fn max(&self) -> Option<f64> {
        todo!("Implementare max")
    }

    /// Restituisce in un'unica operazione `(count, mean, min, max)`,
    /// garantendo che i quattro valori siano coerenti tra loro.
    pub fn snapshot(&self) -> Option<(usize, f64, f64, f64)> {
        todo!("Implementare snapshot")
    }
}

impl Default for SharedStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn stats_iniziali_vuote() {
        let s = SharedStats::new();
        assert_eq!(s.count(), 0);
        assert!(s.mean().is_none());
        assert!(s.min().is_none());
        assert!(s.max().is_none());
        assert!(s.snapshot().is_none());
    }

    #[test]
    fn singolo_campione() {
        let s = SharedStats::new();
        s.add_sample(3.0);
        assert_eq!(s.count(), 1);
        assert_eq!(s.mean(), Some(3.0));
        assert_eq!(s.min(), Some(3.0));
        assert_eq!(s.max(), Some(3.0));
    }

    #[test]
    fn piu_campioni_sequenziali() {
        let s = SharedStats::new();
        for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
            s.add_sample(v);
        }
        assert_eq!(s.count(), 5);
        assert_eq!(s.mean(), Some(3.0));
        assert_eq!(s.min(), Some(1.0));
        assert_eq!(s.max(), Some(5.0));
    }

    #[test]
    fn snapshot_coerente() {
        let s = SharedStats::new();
        s.add_sample(10.0);
        s.add_sample(20.0);
        let snap = s.snapshot().unwrap();
        assert_eq!(snap.0, 2);
        assert_eq!(snap.1, 15.0);
        assert_eq!(snap.2, 10.0);
        assert_eq!(snap.3, 20.0);
    }

    #[test]
    fn aggiornamento_concorrente() {
        let s = Arc::new(SharedStats::new());
        let mut handles = vec![];
        // 4 thread, ciascuno inserisce 1000 campioni.
        for _ in 0..4 {
            let s = Arc::clone(&s);
            handles.push(thread::spawn(move || {
                for v in 1..=1000 {
                    s.add_sample(v as f64);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(s.count(), 4000);
        assert_eq!(s.min(), Some(1.0));
        assert_eq!(s.max(), Some(1000.0));
    }

    #[test]
    fn valori_negativi_e_misti() {
        let s = SharedStats::new();
        for v in [-3.0, 1.0, -10.0, 5.0, 0.0] {
            s.add_sample(v);
        }
        assert_eq!(s.count(), 5);
        assert_eq!(s.min(), Some(-10.0));
        assert_eq!(s.max(), Some(5.0));
        // Media: (-3 + 1 - 10 + 5 + 0) / 5 = -7 / 5 = -1.4
        let media = s.mean().unwrap();
        assert!((media - (-1.4)).abs() < 1e-12);
    }

    #[test]
    fn snapshot_e_metodi_singoli_coerenti_in_assenza_di_scrittori() {
        // In assenza di scrittori concorrenti, i valori restituiti dai
        // singoli getter devono coincidere con quelli del snapshot.
        let s = SharedStats::new();
        for v in [2.0_f64, 4.0, 6.0, 8.0] {
            s.add_sample(v);
        }
        let snap = s.snapshot().unwrap();
        assert_eq!(snap.0, s.count());
        assert!((snap.1 - s.mean().unwrap()).abs() < 1e-12);
        assert_eq!(snap.2, s.min().unwrap());
        assert_eq!(snap.3, s.max().unwrap());
    }

    #[test]
    fn snapshot_internamente_coerente_sotto_scrittori() {
        // Anche con tanti scrittori concorrenti, ciascun snapshot deve
        // restituire valori internamente coerenti: min <= mean <= max
        // e count > 0 implica gli altri tre campi finiti.
        let s = Arc::new(SharedStats::new());
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let mut writers = vec![];
        for t in 0..4 {
            let s = Arc::clone(&s);
            let stop = Arc::clone(&stop);
            writers.push(thread::spawn(move || {
                let mut i = 0u64;
                while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                    s.add_sample((t * 1000 + (i % 1000)) as f64);
                    i += 1;
                }
            }));
        }

        // Si aggiunge almeno un campione per garantire snapshot != None.
        s.add_sample(0.0);
        for _ in 0..2_000 {
            let snap = s.snapshot().expect("snapshot non deve essere None");
            assert!(snap.0 >= 1);
            assert!(snap.2 <= snap.1 + 1e-9, "min > mean: {:?}", snap);
            assert!(snap.1 <= snap.3 + 1e-9, "mean > max: {:?}", snap);
        }
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        for w in writers {
            w.join().unwrap();
        }
    }

    #[test]
    fn molti_lettori_concorrenti() {
        // Verifica che molti lettori in parallelo non si blocchino tra loro
        // (RwLock permette letture multiple). Test funzionale: tutti i
        // lettori vedono lo stesso conteggio finale.
        let s = Arc::new(SharedStats::new());
        for v in 1..=100 {
            s.add_sample(v as f64);
        }
        let mut handles = vec![];
        for _ in 0..16 {
            let s = Arc::clone(&s);
            handles.push(thread::spawn(move || {
                let mut last = 0usize;
                for _ in 0..1_000 {
                    last = s.count();
                }
                last
            }));
        }
        for h in handles {
            assert_eq!(h.join().unwrap(), 100);
        }
    }
}
