use std::net::TcpListener;
use std::thread;
use std::io::BufReader;
use crate::resp::{resp_parser, resp_serializer};
use crate::resp::RespValue;
use crate::commands::handle;
use std::io::Write;
use crate::store::Store;
use std::sync::{Arc,Mutex};

pub struct Redis {
    pub listener: TcpListener,
    pub store: Arc<Mutex<Store>>
}

impl Redis {
    pub fn new(port: u16) -> Self {
        let listener = TcpListener::bind(format!("127.0.0.1:{}",port)).expect("Failed to bind port");
        Self{
            listener,
            store: Arc::new(Mutex::new(Store::new()))
        }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let binding = self.store.clone();
            let thread = thread::spawn(move || {
                let mut reader = BufReader::new(stream);
                let parsed_resp = resp_parser(&mut reader);
                match_resp(&parsed_resp);
                let mut clone = binding.lock().unwrap();
                let response = handle(parsed_resp, &mut clone);
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
        RespValue::BulkString(Some(val)) => println!("{}", val),
        RespValue::BulkString(None) => println!("NONE"),
        RespValue::Array(val) => {
            for i in val {
                match_resp(i);
            }
        }
    }
}