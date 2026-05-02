use std::collections::HashMap;
use crate::resp::RespValue;

pub struct Store {
    store: HashMap<String,String>
}

impl Store {
    pub fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub fn get(&self,key: &str) -> RespValue {
        if self.store.contains_key(key) {
             return RespValue::BulkString(Some(self.store.get(key).unwrap().clone()));
        }
        else {
            return RespValue::BulkString(None);
        }
    }

    pub fn set(&mut self, key: &str, val: &str) -> RespValue {
        self.store.insert(key.to_string(), val.to_string());
        return RespValue::SimpleString(String::from("OK"));
    }
}
