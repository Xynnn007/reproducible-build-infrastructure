pub mod error;
pub mod simple;

// These traits are for a simple KV-store for artifact ID -> manifest
pub trait KVStore {
    fn insert(&self, k: String, v: String) -> Result<(), error::Error>;
    fn delete(&self, k: String) -> Result<(), error::Error>;
    fn search(&self, k: String) -> Result<String, error::Error>;
}

pub fn new(ttype: &str) -> Result<Box<dyn KVStore + Sync + Send>, error::Error> {

    // TODO : Add more KV-db here, who has trait KVStore
    
    match ttype {
        "simple" => Ok(Box::new(simple::SimpleKV::new())),
        _ => Err(error::Error::new(error::Kind::ElementNotFound))
    }
}