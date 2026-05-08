use criterion::{criterion_group, criterion_main, Criterion};
use std::io::{Write, BufRead, BufReader};
use std::net::TcpStream;
use std::sync::Once;
use std::thread;
use std::time::Duration;
use redis::Redis;

static START: Once = Once::new();

fn start_server() {
    START.call_once(|| {
        thread::spawn(|| {
            Redis::new(7880).run();
        });
        loop {
            if TcpStream::connect("127.0.0.1:7880").is_ok() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
}

fn connect() -> (BufReader<TcpStream>, TcpStream) {
    let stream = TcpStream::connect("127.0.0.1:7880").unwrap();
    let writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);
    (reader, writer)
}

fn read_line(reader: &mut BufReader<TcpStream>) -> String {
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    response
}

fn benchmark_set(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    c.bench_function("set", |b| {
        b.iter(|| {
            writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
            read_line(&mut reader);
        })
    });
}

fn benchmark_get(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);

    c.bench_function("get", |b| {
        b.iter(|| {
            writer.write_all(b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n").unwrap();
            read_line(&mut reader);
            read_line(&mut reader);
        })
    });
}

fn benchmark_incr(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    c.bench_function("incr", |b| {
        b.iter(|| {
            writer.write_all(b"*2\r\n$4\r\nINCR\r\n$8\r\nbenchkey\r\n").unwrap();
            read_line(&mut reader);
        })
    });
}

fn benchmark_get_concurrent_20(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);

    let mut group = c.benchmark_group("concurrent");
    group.sample_size(10);
    group.bench_function("get_concurrent_20", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..20 {
                let handle = thread::spawn(|| {
                    let (mut reader, mut writer) = connect();
                    writer.write_all(b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n").unwrap();
                    read_line(&mut reader);
                    read_line(&mut reader);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
    group.bench_function("set_concurrent_20", |b| {
        b.iter(|| {
            let mut handles = vec![];
            for _ in 0..20 {
                let handle = thread::spawn(|| {
                    let (mut reader, mut writer) = connect();
                    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
                    read_line(&mut reader);
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
    group.finish();
}

fn benchmark_expire(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    c.bench_function("expire", |b| {
        b.iter(|| {
            writer.write_all(b"*3\r\n$3\r\nSET\r\n$9\r\nexpirekey\r\n$3\r\nbar\r\n").unwrap();
            read_line(&mut reader);
            writer.write_all(b"*3\r\n$6\r\nEXPIRE\r\n$9\r\nexpirekey\r\n$2\r\n60\r\n").unwrap();
            read_line(&mut reader);
        })
    });
}

fn benchmark_ttl(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$6\r\nttlkey\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);
    writer.write_all(b"*3\r\n$6\r\nEXPIRE\r\n$6\r\nttlkey\r\n$4\r\n3600\r\n").unwrap();
    read_line(&mut reader);

    c.bench_function("ttl", |b| {
        b.iter(|| {
            writer.write_all(b"*2\r\n$3\r\nTTL\r\n$6\r\nttlkey\r\n").unwrap();
            read_line(&mut reader);
        })
    });
}

criterion_group!(
    benches,
    benchmark_set,
    benchmark_get,
    benchmark_incr,
    benchmark_expire,
    benchmark_ttl,
    benchmark_get_concurrent_20,
);
criterion_main!(benches);
