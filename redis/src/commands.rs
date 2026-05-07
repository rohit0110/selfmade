use crate::resp::RespValue;
use crate::store::Store;
use std::sync::{Arc,Mutex};

pub fn handle(resp_value: RespValue, store: Arc<Mutex<Store>>) -> RespValue {
    match resp_value {                                                                                           
        RespValue::Array(elements) => {
            match &elements[0] {                                                                                 
                RespValue::BulkString(Some(cmd)) => match cmd.as_str() {
                    "PING" => RespValue::SimpleString(String::from("PONG")),                                     
                    "GET" => {
                        let mut store = store.lock().unwrap();
                        if let RespValue::BulkString(Some(key)) = &elements[1] {
                            return store.get(key);
                        } else {
                            return RespValue::Error(String::from("COULDNT GET"));
                        }
                    },                                                                            
                    "SET" => {
                        let key = match &elements[1] {
                            RespValue::BulkString(Some(key)) => key,
                            _ => return RespValue::Error(String::from("NO KEY FOUND"))
                        };
                        let val = match &elements[2] {
                            RespValue::BulkString(Some(val)) => val,
                             _ => return RespValue::Error(String::from("NO VALUE FOUND"))
                        };
                        let mut store = store.lock().unwrap();
                        return store.set(key, val);
                    },
                    "DEL" => {
                        let mut store = store.lock().unwrap();
                        match &elements[1] {
                            RespValue::BulkString(Some(key)) => return store.delete(key),
                            _ => RespValue::Error(String::from("NO KEY PROVIDED"))
                        }
                    },
                    "EXISTS" => {
                        let mut store = store.lock().unwrap();
                        match &elements[1] {
                            RespValue::BulkString(Some(key)) => return store.exists(key),
                            _ => RespValue::Error(String::from("NO KEY PROVIDED"))
                        }
                    },
                    "INCR" => {
                        let mut store = store.lock().unwrap();
                        match &elements[1] {
                            RespValue::BulkString(Some(key)) => return store.incr(key),
                            _ => RespValue::Error(String::from("NO KEY PROVIDED")),
                        }
                    },
                    "MGET" => {
                        let mut keys = vec![];
                        for ele in &elements[1..] {
                            keys.push(match ele {
                                RespValue::BulkString(Some(key)) => key.as_str(),
                                _ => continue
                            });
                        }
                        let mut store = store.lock().unwrap();
                        return store.mget(keys);
                    },
                    "EXPIRE" => {
                        let mut store = store.lock().unwrap();
                        match &elements[1] {
                            RespValue::BulkString(Some(key)) => match &elements[2] {
                                RespValue::BulkString(Some(ttl)) => return store.expire(key, match ttl.parse::<i64>() {
                                    Some(val) => val,
                                    _ => return RespValue::Error(String::from("TIME PROVIDED IS NOT A NUMBER")),
                                }), //what if not i64 parsed?
                                _ => return RespValue::Error(String::from("NO TTL PROVIDED")),
                            },
                            _ => RespValue::Error(String::from("NO KEY PROVIDED"))
                        }
                    },
                    "TTL" => {
                        let mut store = store.lock().unwrap();
                        match &elements[1] {
                            RespValue::BulkString(Some(key)) => return store.ttl(key),
                            _ => return RespValue::Error(String::from("NO KEY PROVIDED"))
                        }
                    }
                    _ => RespValue::Error(String::from("ERR unknown command")),                                  
                },                                                                                               
                _ => RespValue::Error(String::from("ERR expected bulk string")),
            }                                                                                                    
        },      
        _ => RespValue::Error(String::from("ERR expected array")),                                               
    }           
}