use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use async_recursion::async_recursion; 

pub enum RespValue {
    SimpleString(String), //Non binary safe strings, starts with +, ends with CRLF, \r\n
    Error(String), // starts with -
    Integer(i64), // starts with :
    BulkString(Option<String>), //starts with $, binary safe strings, Option to handle Null cases
    Array(Vec<RespValue>), //starts with *
}

#[async_recursion]
pub async fn resp_parser(mut reader: &mut BufReader<TcpStream>) -> Option<RespValue> {
    let mut line: String = String::new();
    if reader.read_line(&mut line).await.unwrap() == 0 {
        return None;
    } else {
        match line.chars().nth(0).unwrap() {
            '+' => return Some(RespValue::SimpleString(line[1..].trim_end_matches("\r\n").to_string())),
            '-' => return Some(RespValue::Error(line[1..].trim_end_matches("\r\n").to_string())),
            ':' => return Some(RespValue::Integer(line[1..].trim_end_matches("\r\n").parse::<i64>().expect("NOT AN INT"))),
            '$' => {
                let chars = line[1..].trim_end_matches("\r\n").parse::<usize>().expect("NOT AN INT FOR SIZE BULK STRING");
                let mut buf = vec![0u8;chars];
                reader.read_exact(&mut buf).await.unwrap();
                let mut crlf = String::new();
                reader.read_line(&mut crlf).await; //READ WASTE CHRACTERS \r\n
                return Some(RespValue::BulkString(Some(String::from_utf8(buf).unwrap())));
            },
            '*' => {
                let items = line[1..].trim_end_matches("\r\n").parse::<i64>().expect("NOT AN INT FOR SIZE ARRAY");
                let mut elements :Vec<RespValue> = vec![];
                for _ in 0..items {
                    match resp_parser(&mut *reader).await {
                        Some(parsed_resp) => elements.push(parsed_resp),
                        None => return Some(RespValue::Error(String::from("IDK GANG SUMN BROKE IF IT REACHES HERE")))
                    }
                }
                return Some(RespValue::Array(elements));
            },
            _ => return Some(RespValue::Error(String::from("NOT A VALID FIRST CHARACTER GANG"))),
        }
    }
}

pub fn resp_serializer(resp_value: RespValue) -> String {
    match resp_value {
        RespValue::SimpleString(val) => {
            return format!("+{}\r\n",val);
        },
        RespValue::Error(val) => {
            return format!("-{}\r\n",val)
        },
        RespValue::Integer(val) => {
            return format!(":{}\r\n",val)
        }
         RespValue::BulkString(val) => match val {                                                                        
            Some(s) => format!("${}\r\n{}\r\n", s.len(), s),                                                             
            None => String::from("$-1\r\n"),
        },
        RespValue::Array(val) => {
            let length = val.len();
            let mut array_string = format!("*{}\r\n", length);
            for item in val {
                array_string.push_str(&resp_serializer(item));
            }
            return array_string;
        }
    }
}