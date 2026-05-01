use std::net::TcpListener;
use std::thread;
use std::io::BufReader;
use crate::resp::{resp_parser, resp_serializer};
use crate::resp::RespValue;
use crate::commands::handle;
use std::io::Write;

pub struct Redis {
    pub listener: TcpListener
}

impl Redis {
    pub fn new(port: u16) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).expect("Failed to bind port");
        Self{
            listener
        }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let thread = thread::spawn(move || {
                let mut reader = BufReader::new(stream);
                let parsed_resp = resp_parser(&mut reader);
                match_resp(&parsed_resp);
                let response = handle(parsed_resp);
                let serialized_resp = resp_serializer(response);
                let mut stream = reader.into_inner();
                stream.write_all(serialized_resp.as_bytes()).unwrap();
            });
        }
    }
}

fn match_resp(parsed_resp: &RespValue) {
    match parsed_resp {
        RespValue::SimpleString(val) => println!("{}", val),
        RespValue::Error(val) => println!("{}", val),
        RespValue::Integer(val) => println!("{}", val),
        RespValue::BulkString(val) => println!("{}", val),
        RespValue::Array(val) => {
            for i in val {
                match_resp(i);
            }
        }
    }
}