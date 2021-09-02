use crate::cache::KVStore;
use super::error::{Error,Kind};
use dashmap::DashMap;

pub struct SimpleKV {
    m: DashMap<String, String>,
}

impl SimpleKV {
    pub fn new() -> SimpleKV {
        SimpleKV {
            m: DashMap::new()
        }
    }
}

impl KVStore for SimpleKV {
    fn insert(&self, k: String, v: String) -> Result<(), Error> {
        println!("[SimpleKV] Insert {} -> {}", k, v);
        if self.m.contains_key(&k) {
            return Err(Error::new(Kind::ElementDuplicate))
        }
        self.m.insert(k, v); 
        Ok(())
    }

    fn delete(&self, s: String) -> Result<(), Error> {
        println!("[SimpleKV] delete key {}", s);
        match self.m.remove(&s) {
            None => Err(Error::new(Kind::ElementNotFound)),
            _ => Ok(())
        }
    }

    fn search(&self, k: String) -> Result<String, Error> {
        println!("[SimpleKV] search key {}", k);
        match self.m.get(&k) {
            Some(s) => Ok(s.value().clone()),
            None => Err(Error::new(Kind::ElementNotFound))
        }
    }
}