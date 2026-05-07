use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::resp::RespValue;

pub struct Store {
    store: HashMap<String,StoreValue>
}

struct StoreValue {
    value: String,
    expires_at: Option<Instant>
}

impl StoreValue {
    fn new(value: String) -> Self {
        Self {
            value,
            expires_at: None
        }
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    pub fn get(&mut self,key: &str) -> RespValue {
        if self.store.contains_key(key) {
            if self.has_expired(key) {
                self.store.remove(key);
                return RespValue::BulkString(None);
            }
            return RespValue::BulkString(Some(self.store.get(key).unwrap().value.clone()));
        }
        else {
            return RespValue::BulkString(None);
        }
    }

    pub fn mget(&mut self, keys: Vec<&str>) -> RespValue {
        let mut resp = vec![];
        for key in keys {
            if self.store.contains_key(key) {
                if self.has_expired(key) {
                    self.store.remove(key);
                    resp.push(RespValue::BulkString(None));
                } else {
                    resp.push(RespValue::BulkString(Some(self.store.get(key).unwrap().value.clone())));
                }
            }
            else {
                resp.push(RespValue::BulkString(None));
            }
        }
        return RespValue::Array(resp);
    }

    pub fn set(&mut self, key: &str, val: &str) -> RespValue {
        self.store.insert(key.to_string(), StoreValue::new(val.to_string()));
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

    pub fn exists(&mut self, key: &str) -> RespValue {
        if self.store.contains_key(key) {
            if self.has_expired(key) {
                self.store.remove(key);
                return RespValue::Integer(0);
            }
            return RespValue::Integer(1);
        } else {
            return RespValue::Integer(0);
        }
    } 

    pub fn incr(&mut self, key: &str) -> RespValue {
        if self.store.contains_key(key) && !self.has_expired(key) {
            let val = match self.store.get(key).unwrap().value.clone().parse::<i64>() {
                Ok(n) => n+1,
                Err(_) => return RespValue::Error(String::from("VALUE IS NOT A NUMBER"))
            };
            self.store.insert(key.to_string(), StoreValue::new(val.to_string()));
            return RespValue::Integer(val);
        } else {
            self.store.insert(key.to_string(), StoreValue::new(1.to_string()));
            return RespValue::Integer(1);
        }
    }

    pub fn expire(&mut self, key: &str, seconds: i64) -> RespValue {
        if self.store.contains_key(key) && !self.has_expired(key) {
            if let Some(store_value) = self.store.get_mut(key) {
                store_value.expires_at = Some(Instant::now() + Duration::from_secs(seconds as u64));
                return RespValue::Integer(1);
            } else {
                return RespValue::Error(String::from("Issue with setting expiry"));
            }
        } else {
            return RespValue::Integer(0);
        }
    }  

    pub fn ttl(&mut self, key:&str) -> RespValue {
        if self.store.contains_key(key) && !self.has_expired(key) {
            let expires_at = match self.store.get(key).unwrap().expires_at {
                Some(time) => time,
                None => return RespValue::Integer(-1),
            };
            return RespValue::Integer((expires_at.checked_duration_since(Instant::now())
            .unwrap_or_default()
            .as_secs_f64()
            .ceil()
            ) as i64);
        } else {
            if self.store.contains_key(key) {
                self.store.remove(key);
            }
            return RespValue::Integer(-2);
        }
    }

    fn has_expired(&self, key: &str) -> bool {
        match self.store.get(key) {
            Some(val) => match val.expires_at {
                Some(time) => Instant::now() > time,
                None => return false
            },
            None => return false
        }
    }
}
