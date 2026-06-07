#![allow(dead_code)]
//! Esercizio 2 - Canale MPSC implementato a mano
//!
//! Implementare a mano un canale di comunicazione asincrono unbounded con
//! semantica multi-producer single-consumer, analogo a `std::sync::mpsc`.
//!
//! Vedi il file `Lab6.md` per la specifica completa.

/// Errore restituito da `Sender::send` quando il `Receiver` è stato distrutto.
/// Contiene l'elemento che non è stato possibile consegnare.
#[derive(Debug)]
pub struct SendError<T>(pub T);

/// Errore restituito da `Receiver::recv` quando il canale è vuoto
/// e tutti i `Sender` sono stati distrutti.
#[derive(Debug, PartialEq, Eq)]
pub struct RecvError;

/// Errore restituito da `Receiver::try_recv`.
#[derive(Debug, PartialEq, Eq)]
pub enum TryRecvError {
    /// Il canale è vuoto al momento ma esiste ancora almeno un `Sender`.
    Empty,
    /// Tutti i `Sender` sono stati distrutti e il canale è vuoto.
    Disconnected,
}

/// Estremo di invio del canale. Più produttori possono possedere un
/// proprio `Sender` clonandolo dal `Sender` originale.
pub struct Sender<T> {
    // TODO: definire i campi.
    _marker: std::marker::PhantomData<T>,
}

/// Estremo di ricezione del canale. Esiste un solo `Receiver` per canale.
pub struct Receiver<T> {
    // TODO: definire i campi.
    _marker: std::marker::PhantomData<T>,
}

/// Crea una nuova coppia `(Sender, Receiver)`.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    todo!("Implementare la funzione channel")
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!("Implementare la clonazione di Sender")
    }
}

impl<T> Sender<T> {
    /// Inserisce `item` nel canale. Non si blocca mai per motivi di capacità.
    /// Restituisce `Err(SendError(item))` se il `Receiver` è stato distrutto.
    pub fn send(&self, _item: T) -> Result<(), SendError<T>> {
        todo!("Implementare Sender::send")
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!("Implementare il drop per Sender")    
    }
}

impl<T> Receiver<T> {
    /// Estrae il prossimo elemento dal canale.
    /// Si blocca senza consumare CPU se il canale è vuoto ma esiste
    /// ancora almeno un `Sender`. Restituisce `Err(RecvError)` se il
    /// canale è vuoto e tutti i `Sender` sono stati distrutti.
    pub fn recv(&self) -> Result<T, RecvError> {
        todo!("Implementare Receiver::recv")
    }

    /// Variante non bloccante di `recv`.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        todo!("Implementare Receiver::try_recv")
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        todo!("Implementare il drop per Receiver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn send_e_recv_singolo_thread() {
        let (tx, rx) = channel::<i32>();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        tx.send(3).unwrap();
        assert_eq!(rx.recv(), Ok(1));
        assert_eq!(rx.recv(), Ok(2));
        assert_eq!(rx.recv(), Ok(3));
    }

    #[test]
    fn try_recv_su_canale_vuoto() {
        let (tx, rx) = channel::<i32>();
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        tx.send(42).unwrap();
        assert_eq!(rx.try_recv(), Ok(42));
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
    }

    #[test]
    fn recv_si_blocca_finche_arriva_un_messaggio() {
        let (tx, rx) = channel::<i32>();
        let ricevuti = Arc::new(AtomicUsize::new(0));

        let r2 = Arc::clone(&ricevuti);
        let consumer = thread::spawn(move || {
            let v = rx.recv().unwrap();
            assert_eq!(v, 99);
            r2.fetch_add(1, Ordering::SeqCst);
        });

        // Diamo tempo al consumatore di mettersi in attesa.
        thread::sleep(Duration::from_millis(100));
        assert_eq!(ricevuti.load(Ordering::SeqCst), 0);

        tx.send(99).unwrap();
        consumer.join().unwrap();
        assert_eq!(ricevuti.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn recv_restituisce_errore_quando_tutti_i_sender_droppati() {
        let (tx, rx) = channel::<i32>();
        drop(tx);
        assert_eq!(rx.recv(), Err(RecvError));
        assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
    }

    #[test]
    fn recv_si_sblocca_alla_chiusura_dei_sender() {
        // Il Receiver è bloccato su recv quando l'ultimo Sender viene droppato:
        // recv deve sbloccarsi e restituire RecvError.
        let (tx, rx) = channel::<i32>();
        let sbloccato = Arc::new(AtomicUsize::new(0));

        let s2 = Arc::clone(&sbloccato);
        let consumer = thread::spawn(move || {
            let r = rx.recv();
            assert_eq!(r, Err(RecvError));
            s2.fetch_add(1, Ordering::SeqCst);
        });

        thread::sleep(Duration::from_millis(100));
        assert_eq!(sbloccato.load(Ordering::SeqCst), 0);

        drop(tx);
        consumer.join().unwrap();
        assert_eq!(sbloccato.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn messaggi_residui_letti_anche_dopo_chiusura() {
        // Se i Sender vengono droppati ma ci sono ancora messaggi in coda,
        // il Receiver deve poterli leggere; solo dopo riceverà RecvError.
        let (tx, rx) = channel::<i32>();
        tx.send(10).unwrap();
        tx.send(20).unwrap();
        drop(tx);

        assert_eq!(rx.recv(), Ok(10));
        assert_eq!(rx.recv(), Ok(20));
        assert_eq!(rx.recv(), Err(RecvError));
    }

    #[test]
    fn send_fallisce_se_receiver_droppato() {
        let (tx, rx) = channel::<i32>();
        drop(rx);
        match tx.send(1) {
            Err(SendError(v)) => assert_eq!(v, 1),
            Ok(_) => panic!("send avrebbe dovuto fallire dopo il drop del Receiver"),
        }
    }

    #[test]
    fn sender_clonabile_multi_producer() {
        let (tx, rx) = channel::<i32>();
        let n_prod = 5;
        let per_prod = 100;

        let mut handles = vec![];
        for _ in 0..n_prod {
            let tx = tx.clone();
            handles.push(thread::spawn(move || {
                for i in 0..per_prod {
                    tx.send(i).unwrap();
                }
            }));
        }
        // Rilasciamo l'originale: ne restano `n_prod` cloni.
        drop(tx);

        let mut ricevuti = 0usize;
        while let Ok(_) = rx.recv() {
            ricevuti += 1;
        }
        assert_eq!(ricevuti, n_prod * per_prod as usize);
    }

    #[test]
    fn ordine_fifo_da_singolo_sender() {
        // Da un singolo Sender, i messaggi devono arrivare nello stesso ordine
        // in cui sono stati inviati.
        let (tx, rx) = channel::<i32>();
        let producer = thread::spawn(move || {
            for i in 0..1000 {
                tx.send(i).unwrap();
            }
        });

        let mut atteso = 0;
        while let Ok(v) = rx.recv() {
            assert_eq!(v, atteso);
            atteso += 1;
        }
        assert_eq!(atteso, 1000);
        producer.join().unwrap();
    }

    #[test]
    fn drop_canale_senza_ricezioni_non_blocca() {
        // Costruire ed eliminare canali senza usarli non deve causare
        // attese o blocchi.
        for _ in 0..100 {
            let (_tx, _rx) = channel::<i32>();
        }
    }
}