#![allow(dead_code)]
//! Esercizio 1 - Barriera di sincronizzazione
//!
//! Implementare una barriera di sincronizzazione riusabile. Il metodo `wait`
//! deve bloccare il thread chiamante finché tutti gli `n` partecipanti non
//! hanno raggiunto la barriera. L'attesa deve essere passiva (nessun polling).
//!
//! Vedi il file `Lab6.md` per la specifica completa.

/// Barriera di sincronizzazione riusabile per `n` thread.
pub struct MyBarrier {
    // TODO: definire i campi.
}

impl MyBarrier {
    /// Crea una nuova barriera per `n` partecipanti.
    pub fn new(_n: usize) -> Self {
        todo!("Implementare MyBarrier::new")
    }

    /// Blocca il thread chiamante finché tutti gli `n` partecipanti
    /// non hanno chiamato a loro volta `wait()`.
    pub fn wait(&self) {
        todo!("Implementare MyBarrier::wait")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn barriera_singolo_thread() {
        // n = 1 => wait() ritorna subito.
        let b = MyBarrier::new(1);
        b.wait();
    }

    #[test]
    fn barriera_sblocca_tutti() {
        let n = 4;
        let b = Arc::new(MyBarrier::new(n));
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        for _ in 0..n {
            let b = Arc::clone(&b);
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                // Tutti incrementano prima della barriera, attendono, poi controllano.
                c.fetch_add(1, Ordering::SeqCst);
                b.wait();
                // Dopo la barriera tutti devono vedere n incrementi.
                assert_eq!(c.load(Ordering::SeqCst), n);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn barriera_blocca_finche_non_arrivano_tutti() {
        let n = 3;
        let b = Arc::new(MyBarrier::new(n));
        let arrivati_dopo = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        // Due thread arrivano subito.
        for _ in 0..n - 1 {
            let b = Arc::clone(&b);
            let a = Arc::clone(&arrivati_dopo);
            handles.push(thread::spawn(move || {
                b.wait();
                a.fetch_add(1, Ordering::SeqCst);
            }));
        }
        // Diamo tempo ai due thread di mettersi in attesa.
        thread::sleep(Duration::from_millis(100));
        assert_eq!(arrivati_dopo.load(Ordering::SeqCst), 0);

        // Il terzo thread sblocca tutti.
        let b2 = Arc::clone(&b);
        let a2 = Arc::clone(&arrivati_dopo);
        handles.push(thread::spawn(move || {
            b2.wait();
            a2.fetch_add(1, Ordering::SeqCst);
        }));

        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(arrivati_dopo.load(Ordering::SeqCst), n);
    }

    #[test]
fn barriera_riusabile() {
        let (tx, rx) = std::sync::mpsc::channel::<usize>();
        let n = 3;
        let giri = 5;
        let b = Arc::new(MyBarrier::new(n));
        let counter = Arc::new(AtomicUsize::new(0));

        let mut handles = vec![];
        for _ in 0..n {
            let b = Arc::clone(&b);
            let c = Arc::clone(&counter);
            let tx = tx.clone();
            handles.push(thread::spawn(move || {
                for g in 1..giri+1 {
                    c.fetch_add(1, Ordering::SeqCst);
                    b.wait();
                    tx.send(g).unwrap();
                }
            }));
        }
        drop(tx);
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::SeqCst), n * giri);
        let mut giro = 0 as usize;
        while let Ok(g) = rx.recv() {
            assert!(g >= giro, "Giro ricevuto {g} ma ci aspettavamo un numero non maggiore di {giro}");
            giro = g;
        }
    }
}
