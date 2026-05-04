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

    pub fn delete(&mut self, key: &str) -> RespValue {
        if self.store.contains_key(key) {
            self.store.remove(key);
            return RespValue::Integer(1);
        } else {
            return RespValue::Integer(0);
        }
    }

    pub fn exists(&self, key: &str) -> RespValue {
        if self.store.contains_key(key) {
            return RespValue::Integer(1);
        } else {
            return RespValue::Integer(0);
        }
    } 

    pub fn incr(&mut self, key: &str) -> RespValue {
        if self.store.contains_key(key) {
            let mut val = match self.store.get(key).clone().unwrap().parse::<i64>() {
                Ok(n) => n+1,
                Err(_) => return RespValue::Error(String::from("VALUE IS NOT A NUMBER"))
            };
            self.store.insert(key.to_string(), val.to_string());
            return RespValue::Integer(val);
        } else {
            self.store.insert(key.to_string(), 1.to_string());
            return RespValue::Integer(1);
        }
    }
}
