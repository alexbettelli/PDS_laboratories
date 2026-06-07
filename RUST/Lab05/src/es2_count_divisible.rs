#![allow(dead_code)]
//! Esercizio 2 - Conteggio parallelo con stato condiviso
//!
//! Implementare la funzione `count_divisible` che conta quanti elementi di
//! `data` sono divisibili per `k`, usando `n_threads` thread che cooperano
//! aggiornando un contatore condiviso `Arc<Mutex<usize>>`.
//!
//! Vedi il file `Lab5.md` per la specifica completa.

/// Restituisce il numero di elementi di `data` divisibili per `k`,
/// usando `n_threads` thread.
pub fn count_divisible(_data: Vec<u32>, _k: u32, _n_threads: usize) -> usize {
    todo!("Implementare count_divisible")
}

/// Restituisce il numero di elementi di `data` divisibili per `k`,
/// usando `n_threads` thread che aggiornano un contatore atomico condiviso.
pub fn count_divisible_atomic(_data: Vec<u32>, _k: u32, _n_threads: usize) -> usize {
    todo!("Implementare count_divisible_atomic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nessun_divisibile() {
        let data = vec![1, 3, 5, 7, 9];
        assert_eq!(count_divisible(data, 2, 4), 0);
    }

    #[test]
    fn tutti_divisibili() {
        let data = vec![6, 12, 18, 24];
        assert_eq!(count_divisible(data, 3, 2), 4);
    }

    #[test]
    fn divisibili_per_uno() {
        let data: Vec<u32> = (1..=100).collect();
        assert_eq!(count_divisible(data, 1, 4), 100);
    }

    #[test]
    fn vettore_grande() {
        let data: Vec<u32> = (1..=10_000).collect();
        let atteso = data.iter().filter(|&&x| x % 7 == 0).count();
        assert_eq!(count_divisible(data, 7, 5), atteso);
    }

    #[test]
    fn vettore_vuoto() {
        let data: Vec<u32> = vec![];
        assert_eq!(count_divisible(data, 3, 4), 0);
    }

    #[test]
    fn singolo_thread() {
        let data: Vec<u32> = (1..=100).collect();
        let atteso = data.iter().filter(|&&x| x % 5 == 0).count();
        assert_eq!(count_divisible(data, 5, 1), atteso);
    }

    #[test]
    fn piu_thread_che_elementi() {
        let data = vec![2, 4, 7];
        // 7 non è divisibile per 2; 2 e 4 sì.
        assert_eq!(count_divisible(data, 2, 16), 2);
    }

    // ----- Test per la versione atomica -----

    #[test]
    fn atomic_nessun_divisibile() {
        let data = vec![1, 3, 5, 7, 9];
        assert_eq!(count_divisible_atomic(data, 2, 4), 0);
    }

    #[test]
    fn atomic_tutti_divisibili() {
        let data = vec![6, 12, 18, 24];
        assert_eq!(count_divisible_atomic(data, 3, 2), 4);
    }

    #[test]
    fn atomic_vettore_vuoto() {
        let data: Vec<u32> = vec![];
        assert_eq!(count_divisible_atomic(data, 3, 4), 0);
    }

    #[test]
    fn atomic_vettore_grande() {
        let data: Vec<u32> = (1..=10_000).collect();
        let atteso = data.iter().filter(|&&x| x % 7 == 0).count();
        assert_eq!(count_divisible_atomic(data, 7, 5), atteso);
    }

    #[test]
    fn coerenza_mutex_atomic() {
        // Le due implementazioni devono restituire esattamente lo stesso
        // risultato per qualunque combinazione di input.
        let data: Vec<u32> = (0..20_000).map(|i| (i * 13 + 7) % 1000).collect();
        for k in [1u32, 2, 3, 5, 7, 11, 100] {
            for nt in [1usize, 2, 4, 8] {
                let a = count_divisible(data.clone(), k, nt);
                let b = count_divisible_atomic(data.clone(), k, nt);
                assert_eq!(a, b, "divergenza per k={}, n_threads={}", k, nt);
            }
        }
    }

    #[test]
    fn nessuna_perdita_di_aggiornamenti() {
        // Stress test: molti thread, vettore grande, k = 1 (tutti divisibili).
        // Il conteggio finale deve essere esattamente data.len() in entrambe
        // le implementazioni: una eventuale race condition farebbe perdere
        // aggiornamenti.
        let data: Vec<u32> = (1..=50_000).collect();
        assert_eq!(count_divisible(data.clone(), 1, 16), 50_000);
        assert_eq!(count_divisible_atomic(data, 1, 16), 50_000);
    }
}
