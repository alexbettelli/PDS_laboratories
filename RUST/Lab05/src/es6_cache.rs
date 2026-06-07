#![allow(dead_code)]
//! Esercizio 6 - Cache condivisa con politica di scadenza
//!
//! Implementare la struct generica `Cache<V>` che associa chiavi `String` a
//! valori di tipo `V`, con scadenza per TTL e accesso thread-safe basato
//! su `RwLock`.
//!
//! Vedi il file `Lab5.md` per la specifica completa.

use std::time::Duration;

/// Cache thread-safe con scadenza basata su TTL.
pub struct Cache<V> {
    _marker: std::marker::PhantomData<V>, // Rimuovere quando si aggiungono i campi reali, questo serve solo per far compilare la struct vuota.
}

impl<V> Cache<V>
where
    V: Clone,
{
    pub fn new(_ttl: Duration) -> Self {
        todo!("Implementare new")
    }

    /// Inserisce o sovrascrive la voce associata a `key`,
    /// aggiornandone l'istante di inserimento.
    pub fn insert(&self, _key: String, _value: V) {
        todo!("Implementare insert")
    }

    /// Restituisce il valore associato a `key` se presente e non scaduto.
    /// Una voce scaduta letta tramite `get` deve essere rimossa (lazy eviction).
    pub fn get(&self, _key: &str) -> Option<V> {
        todo!("Implementare get")
    }

    /// Numero di voci attualmente presenti (incluse eventuali scadute non ancora rimosse).
    pub fn len(&self) -> usize {
        todo!("Implementare len")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Implementare is_empty")
    }

    /// Rimuove tutte le voci scadute. Restituisce il numero di voci eliminate.
    pub fn purge_expired(&self) -> usize {
        todo!("Implementare purge_expired")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn insert_e_get() {
        let c: Cache<String> = Cache::new(Duration::from_secs(60));
        c.insert("a".to_string(), "uno".to_string());
        c.insert("b".to_string(), "due".to_string());
        assert_eq!(c.get("a"), Some("uno".to_string()));
        assert_eq!(c.get("b"), Some("due".to_string()));
        assert_eq!(c.get("c"), None);
    }

    #[test]
    fn insert_sovrascrive() {
        let c: Cache<i32> = Cache::new(Duration::from_secs(60));
        c.insert("k".to_string(), 1);
        c.insert("k".to_string(), 2);
        assert_eq!(c.get("k"), Some(2));
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn voce_scaduta_non_visibile() {
        let c: Cache<i32> = Cache::new(Duration::from_millis(50));
        c.insert("x".to_string(), 42);
        thread::sleep(Duration::from_millis(120));
        assert_eq!(c.get("x"), None);
    }

    #[test]
    fn lazy_eviction_su_get() {
        let c: Cache<i32> = Cache::new(Duration::from_millis(50));
        c.insert("x".to_string(), 1);
        assert_eq!(c.len(), 1);
        thread::sleep(Duration::from_millis(120));
        // get su voce scaduta deve rimuoverla
        assert_eq!(c.get("x"), None);
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn purge_expired_rimuove_solo_scadute() {
        let c: Cache<i32> = Cache::new(Duration::from_millis(80));
        c.insert("vecchia".to_string(), 1);
        thread::sleep(Duration::from_millis(120));
        c.insert("nuova".to_string(), 2);
        let rimosse = c.purge_expired();
        assert_eq!(rimosse, 1);
        assert_eq!(c.len(), 1);
        assert_eq!(c.get("nuova"), Some(2));
        assert_eq!(c.get("vecchia"), None);
    }

    #[test]
    fn uso_concorrente() {
        let c: Arc<Cache<i32>> = Arc::new(Cache::new(Duration::from_secs(5)));
        let mut handles = vec![];

        // Scrittori
        for t in 0..4 {
            let c = Arc::clone(&c);
            handles.push(thread::spawn(move || {
                for i in 0..100 {
                    c.insert(format!("t{}_k{}", t, i), i);
                }
            }));
        }
        // Lettori
        for _ in 0..4 {
            let c = Arc::clone(&c);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    let _ = c.get("t0_k0");
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.len(), 400);
    }

    #[test]
    fn get_su_chiave_inesistente() {
        let c: Cache<i32> = Cache::new(Duration::from_secs(60));
        assert_eq!(c.get("non_esiste"), None);
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn cache_appena_creata_e_vuota() {
        let c: Cache<i32> = Cache::new(Duration::from_secs(60));
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
    }

    #[test]
    fn purge_expired_su_cache_vuota() {
        let c: Cache<i32> = Cache::new(Duration::from_millis(10));
        assert_eq!(c.purge_expired(), 0);
    }

    #[test]
    fn purge_expired_senza_voci_scadute() {
        let c: Cache<i32> = Cache::new(Duration::from_secs(60));
        c.insert("a".to_string(), 1);
        c.insert("b".to_string(), 2);
        assert_eq!(c.purge_expired(), 0);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn purge_expired_rimuove_tutte_le_voci_scadute() {
        let c: Cache<i32> = Cache::new(Duration::from_millis(50));
        c.insert("a".to_string(), 1);
        c.insert("b".to_string(), 2);
        c.insert("c".to_string(), 3);
        thread::sleep(Duration::from_millis(120));
        assert_eq!(c.purge_expired(), 3);
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn insert_aggiorna_listante_di_inserimento() {
        // Reinserire una chiave già scaduta deve renderla nuovamente
        // visibile, perché insert aggiorna l'istante di inserimento.
        let c: Cache<i32> = Cache::new(Duration::from_millis(50));
        c.insert("k".to_string(), 1);
        thread::sleep(Duration::from_millis(120));
        // La voce è scaduta, ma non ancora rimossa.
        c.insert("k".to_string(), 2);
        assert_eq!(c.get("k"), Some(2));
    }

    #[test]
    fn voci_diverse_scadenze_indipendenti() {
        // Due voci inserite in momenti diversi scadono in modo indipendente.
        let c: Cache<i32> = Cache::new(Duration::from_millis(80));
        c.insert("vecchia".to_string(), 1);
        thread::sleep(Duration::from_millis(60));
        c.insert("recente".to_string(), 2);
        // A ~60ms: entrambe ancora valide.
        assert_eq!(c.get("vecchia"), Some(1));
        assert_eq!(c.get("recente"), Some(2));
        // A ~110ms (rispetto al primo insert): la prima è scaduta, la
        // seconda no (è stata inserita ~50ms fa).
        thread::sleep(Duration::from_millis(50));
        assert_eq!(c.get("vecchia"), None);
        assert_eq!(c.get("recente"), Some(2));
    }

    #[test]
    fn cache_generica_con_tipi_diversi() {
        // V deve essere generico: la cache deve funzionare con tipi
        // diversi (Vec<u8>, String, struct ecc.) purché siano Clone.
        let c: Cache<Vec<u8>> = Cache::new(Duration::from_secs(60));
        c.insert("payload".to_string(), vec![1, 2, 3]);
        assert_eq!(c.get("payload"), Some(vec![1, 2, 3]));

        #[derive(Clone, Debug, PartialEq)]
        struct Persona {
            nome: String,
            eta: u32,
        }
        let c2: Cache<Persona> = Cache::new(Duration::from_secs(60));
        let p = Persona { nome: "Mario".to_string(), eta: 30 };
        c2.insert("user".to_string(), p.clone());
        assert_eq!(c2.get("user"), Some(p));
    }

    #[test]
    fn lettori_concorrenti_multipli() {
        // Verifica che molti lettori in parallelo possano accedere alla
        // cache senza serializzarsi (sfrutto del lock di lettura).
        let c: Arc<Cache<i32>> = Arc::new(Cache::new(Duration::from_secs(60)));
        for i in 0..50 {
            c.insert(format!("k{}", i), i);
        }
        let mut handles = vec![];
        for _ in 0..16 {
            let c = Arc::clone(&c);
            handles.push(thread::spawn(move || {
                let mut hits = 0;
                for i in 0..50 {
                    if c.get(&format!("k{}", i)).is_some() {
                        hits += 1;
                    }
                }
                hits
            }));
        }
        for h in handles {
            assert_eq!(h.join().unwrap(), 50);
        }
        assert_eq!(c.len(), 50);
    }
}