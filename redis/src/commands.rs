use crate::resp::RespValue;

 pub fn handle(resp_value: RespValue) -> RespValue {
      match resp_value {                                                                                           
          RespValue::Array(elements) => {
              match &elements[0] {                                                                                 
                  RespValue::BulkString(cmd) => match cmd.as_str() {
                      "PING" => RespValue::SimpleString(String::from("PONG")),                                     
                      "GET" => todo!(),                                                                            
                      "SET" => todo!(),
                      _ => RespValue::Error(String::from("ERR unknown command")),                                  
                  },                                                                                               
                  _ => RespValue::Error(String::from("ERR expected bulk string")),
              }                                                                                                    
          },      
          _ => RespValue::Error(String::from("ERR expected array")),                                               
      }           
  }