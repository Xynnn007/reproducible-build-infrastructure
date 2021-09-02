pub mod error;
pub mod in_toto;
use std::{collections::HashMap};
use crate::rvps_handlers::error::{Kind::TypeNotFound, Error};

// (file) -> Result
type Callback = fn(&Vec<u8>) -> Result<String, Box<dyn std::error::Error>>;

// rvps_type -> Callback
pub struct RvpsHandler {
    handlers: HashMap<String, Callback>,
}

impl RvpsHandler {
    pub fn add_callback(&mut self, name: &str, func: Callback) {
        self.handlers.insert(name.to_string(), func);
    }

    pub fn call(&self, name:&str, file: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        let callback = self.handlers.get(name);
        match callback {
            None => Err(Box::new(Error::new(TypeNotFound))),
            Some(f) => f(file),
        }
    }

    pub fn new() -> RvpsHandler {
        let mut r = RvpsHandler {
            handlers : HashMap::<String, Callback>::new(),
        };

        // TODO : register more types of RVPS here use 'add_callback'
        
        r.add_callback("in-toto", in_toto::handler);
        r
    }
}

impl std::fmt::Debug for RvpsHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Handler")
    }
}
