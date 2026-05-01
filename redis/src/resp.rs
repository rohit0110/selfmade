use std::io::BufReader;
use std::net::TcpStream;
use std::io::BufRead;
use std::io::Read;

pub enum RespValue {
    SimpleString(String), //Non binary safe strings, starts with +, ends with CRLF, \r\n
    Error(String), // starts with -
    Integer(i64), // starts with :
    BulkString(String), //starts with $, binary safe strings
    Array(Vec<RespValue>), //starts with *
}

pub fn resp_parser(mut reader: &mut BufReader<TcpStream>) -> RespValue {
    let mut line: String = String::new();
    reader.read_line(&mut line);

    match line.chars().nth(0).unwrap() {
        '+' => return RespValue::SimpleString(line[1..].to_string()),
        '-' => return RespValue::Error(line[1..].to_string()),
        ':' => return RespValue::Integer(line[1..].trim_end_matches("\r\n").parse::<i64>().expect("NOT AN INT")),
        '$' => {
            let chars = line[1..].trim_end_matches("\r\n").parse::<usize>().expect("NOT AN INT FOR SIZE BULK STRING");
            let mut buf = vec![0u8;chars];
            reader.read_exact(&mut buf).unwrap();
            let mut crlf = String::new();
            reader.read_line(&mut crlf); //READ WASTE CHRACTERS \r\n
            return RespValue::BulkString(String::from_utf8(buf).unwrap());
        },
        '*' => {
            let items = line[1..].trim_end_matches("\r\n").parse::<i64>().expect("NOT AN INT FOR SIZE ARRAY");
            let mut elements :Vec<RespValue> = vec![];
            for _ in 0..items {
                elements.push(resp_parser(&mut *reader));
            }
            return RespValue::Array(elements);
        },
         _ => todo!(),
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
        RespValue::BulkString(val) => {
            let length = val.len();
            return format!("${}\r\n{}\r\n",length,val);
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