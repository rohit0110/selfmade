use crate::resp::RespValue;
use crate::store::Store;

pub fn handle(resp_value: RespValue, store: &mut Store) -> RespValue {
    match resp_value {                                                                                           
        RespValue::Array(elements) => {
            match &elements[0] {                                                                                 
                RespValue::BulkString(Some(cmd)) => match cmd.as_str() {
                    "PING" => RespValue::SimpleString(String::from("PONG")),                                     
                    "GET" => {
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
                        return store.set(key, val);
                    },
                    _ => RespValue::Error(String::from("ERR unknown command")),                                  
                },                                                                                               
                _ => RespValue::Error(String::from("ERR expected bulk string")),
            }                                                                                                    
        },      
        _ => RespValue::Error(String::from("ERR expected array")),                                               
    }           
}