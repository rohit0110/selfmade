use std::io::{Write, BufRead, BufReader};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::sync::Once;
use redis::Redis;
use tokio::net::TcpListener;

static START: Once = Once::new();

fn start_server() {
    START.call_once(|| {
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let listener = TcpListener::bind("127.0.0.1:7879").await.unwrap();
                Redis::new(listener).run().await;
            });
        });
        loop {
            if TcpStream::connect("127.0.0.1:7879").is_ok() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}

fn connect() -> (BufReader<TcpStream>, TcpStream) {
    let stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    let writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);
    (reader, writer)
}

fn read_line(reader: &mut BufReader<TcpStream>) -> String {
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response
}

fn send(writer: &mut TcpStream, cmd: &[u8]) {
    writer.write_all(cmd).unwrap();
}

#[test]
fn test_ping() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*1\r\n$4\r\nPING\r\n");
    assert_eq!(read_line(&mut reader), "+PONG\r\n");
}

#[test]
fn test_set_get() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$6\r\nsetkey\r\n$3\r\nbar\r\n");
    assert_eq!(read_line(&mut reader), "+OK\r\n");
    send(&mut writer, b"*2\r\n$3\r\nGET\r\n$6\r\nsetkey\r\n");
    assert_eq!(read_line(&mut reader), "$3\r\n");
    assert_eq!(read_line(&mut reader), "bar\r\n");
}

#[test]
fn test_get_nonexistent() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*2\r\n$3\r\nGET\r\n$11\r\nnonexistent\r\n");
    assert_eq!(read_line(&mut reader), "$-1\r\n");
}

#[test]
fn test_del() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$6\r\ndelkey\r\n$3\r\nfoo\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*2\r\n$3\r\nDEL\r\n$6\r\ndelkey\r\n");
    assert_eq!(read_line(&mut reader), ":1\r\n");
    send(&mut writer, b"*2\r\n$3\r\nDEL\r\n$6\r\ndelkey\r\n");
    assert_eq!(read_line(&mut reader), ":0\r\n");
}

#[test]
fn test_exists() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$9\r\nexistskey\r\n$3\r\nfoo\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*2\r\n$6\r\nEXISTS\r\n$9\r\nexistskey\r\n");
    assert_eq!(read_line(&mut reader), ":1\r\n");
    send(&mut writer, b"*2\r\n$6\r\nEXISTS\r\n$11\r\nnonexistent\r\n");
    assert_eq!(read_line(&mut reader), ":0\r\n");
}

#[test]
fn test_incr() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*2\r\n$4\r\nINCR\r\n$8\r\nincrkey1\r\n");
    assert_eq!(read_line(&mut reader), ":1\r\n");
    send(&mut writer, b"*2\r\n$4\r\nINCR\r\n$8\r\nincrkey1\r\n");
    assert_eq!(read_line(&mut reader), ":2\r\n");
    send(&mut writer, b"*2\r\n$4\r\nINCR\r\n$8\r\nincrkey1\r\n");
    assert_eq!(read_line(&mut reader), ":3\r\n");
}

#[test]
fn test_mget() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$4\r\nkey1\r\n$3\r\nval\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$4\r\nkey2\r\n$3\r\nval\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*4\r\n$4\r\nMGET\r\n$4\r\nkey1\r\n$4\r\nkey2\r\n$11\r\nnonexistent\r\n");
    assert_eq!(read_line(&mut reader), "*3\r\n");
    assert_eq!(read_line(&mut reader), "$3\r\n");
    assert_eq!(read_line(&mut reader), "val\r\n");
    assert_eq!(read_line(&mut reader), "$3\r\n");
    assert_eq!(read_line(&mut reader), "val\r\n");
    assert_eq!(read_line(&mut reader), "$-1\r\n");
}

#[test]
fn test_expire_ttl() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$7\r\nttlkey1\r\n$3\r\nfoo\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*3\r\n$6\r\nEXPIRE\r\n$7\r\nttlkey1\r\n$1\r\n1\r\n");
    assert_eq!(read_line(&mut reader), ":1\r\n");
    send(&mut writer, b"*2\r\n$3\r\nTTL\r\n$7\r\nttlkey1\r\n");
    assert_eq!(read_line(&mut reader), ":1\r\n");
    thread::sleep(Duration::from_millis(1100));
    send(&mut writer, b"*2\r\n$3\r\nGET\r\n$7\r\nttlkey1\r\n");
    assert_eq!(read_line(&mut reader), "$-1\r\n");
}

#[test]
fn test_expire_nonexistent() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$6\r\nEXPIRE\r\n$11\r\nnonexistent\r\n$1\r\n5\r\n");
    assert_eq!(read_line(&mut reader), ":0\r\n");
}

#[test]
fn test_ttl_no_expiry() {
    start_server();
    let (mut reader, mut writer) = connect();
    send(&mut writer, b"*3\r\n$3\r\nSET\r\n$7\r\nttlkey2\r\n$3\r\nfoo\r\n");
    read_line(&mut reader);
    send(&mut writer, b"*2\r\n$3\r\nTTL\r\n$7\r\nttlkey2\r\n");
    assert_eq!(read_line(&mut reader), ":-1\r\n");
}

#[test]
fn test_concurrent_sets_gets() {
    start_server();
    let mut handles = vec![];
    for i in 0..20 {
        let handle = thread::spawn(move || {
            let (mut reader, mut writer) = connect();
            let key = format!("conckey{}", i);
            let key_len = key.len();
            let cmd = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n$3\r\nval\r\n", key_len, key);
            send(&mut writer, cmd.as_bytes());
            assert_eq!(read_line(&mut reader), "+OK\r\n");
            let cmd = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key_len, key);
            send(&mut writer, cmd.as_bytes());
            assert_eq!(read_line(&mut reader), "$3\r\n");
            assert_eq!(read_line(&mut reader), "val\r\n");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
