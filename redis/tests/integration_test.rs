use std::io::{Write, BufRead, BufReader};                                                                        
use std::net::TcpStream;                                                                                         
use std::thread;                                                                                                 
use std::time::Duration;                                                                                         
use std::sync::Once;                                                                                             
use redis::Redis;   
use std::sync::{Arc,Mutex};                                                                                             

static START: Once = Once::new();                                                                                
                  
fn start_server() {
    START.call_once(|| {
        thread::spawn(|| {
            Redis::new(7879).run();
        });                                                                                                      
        thread::sleep(Duration::from_millis(100));
    });                                                                                                          
}               

#[test]
fn test_ping() {
    start_server();
    let mut stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    stream.write_all(b"*1\r\n$4\r\nPING\r\n").unwrap();                                                          
    let mut reader = BufReader::new(stream);
    let mut response = String::new();                                                                            
    reader.read_line(&mut response).unwrap();
    assert_eq!(response, "+PONG\r\n");                                                                           
}                                                                                                                
   
#[test]                                                                                                          
fn test_set_get() {
    start_server();
    let mut stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    let mut stream_write = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);                                                                         
                    
    stream_write.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();                               
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();                                                                        
                                                                                                                
    stream_write.write_all(b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n").unwrap();                                            
    let mut response2 = String::new();                                                                               
    reader.read_line(&mut response2).unwrap();                                                                             
}

#[test]
fn concurrent_gets() {                                                                                           
    start_server();                                                                                              
    let mut handles = vec![];
    for _ in 0..100 {                                                                                             
        let handle = thread::spawn(move || {
            let mut stream_write = TcpStream::connect("127.0.0.1:7879").unwrap();
            let mut reader = BufReader::new(stream_write.try_clone().unwrap());                                  
            stream_write.write_all(b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n").unwrap();
            let mut response = String::new();                                                                    
            reader.read_line(&mut response).unwrap();                                                            
            assert!(response.starts_with("$"));                                                                  
        });                                                                                                      
        handles.push(handle);
    }                                                                                                            
    for handle in handles {
        handle.join().unwrap();                                                                                  
    }                
}