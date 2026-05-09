use tokio::net::TcpListener;
use tokio::io::BufReader;
use crate::resp::{resp_parser, resp_serializer};
use crate::resp::RespValue;
use crate::commands::handle;
use tokio::io::AsyncWriteExt;
use crate::store::Store;
use std::sync::{Arc,Mutex};

pub struct Redis {
    pub listener: TcpListener,
    pub store: Arc<Mutex<Store>>
}

impl Redis {
    pub fn new(listener: TcpListener) -> Self {
        Self{
            listener,
            store: Arc::new(Mutex::new(Store::new()))
        }
    }

    pub async fn run(&self) {
        loop {
            let (stream, _addr) = self.listener.accept().await.unwrap(); 
            let binding = self.store.clone();
            let thread = tokio::spawn(async move {
                let mut reader = BufReader::new(stream);
                loop {
                    match resp_parser(&mut reader).await {
                        Some(parsed_resp) => {
                            // match_resp(&parsed_resp);
                            let response = handle(parsed_resp, binding.clone());
                            let serialized_resp = resp_serializer(response);
                            let mut stream = reader.get_mut();
                            stream.write_all(serialized_resp.as_bytes()).await.unwrap();
                        },
                        None => break,
                    }
                }
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