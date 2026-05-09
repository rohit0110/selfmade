use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::io::{Write, BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{Once, Arc, Mutex};
use std::thread;
use std::time::Duration;
use redis::Redis;
use tokio::net::TcpListener;

static START: Once = Once::new();

fn start_server() {
    START.call_once(|| {
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let listener = TcpListener::bind("127.0.0.1:7880").await.unwrap();
                Redis::new(listener).run().await;
            });
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

// --- single-client latency ---

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

fn benchmark_del(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    c.bench_function("del", |b| {
        b.iter(|| {
            writer.write_all(b"*3\r\n$3\r\nSET\r\n$6\r\ndelkey\r\n$3\r\nbar\r\n").unwrap();
            read_line(&mut reader);
            writer.write_all(b"*2\r\n$3\r\nDEL\r\n$6\r\ndelkey\r\n").unwrap();
            read_line(&mut reader);
        })
    });
}

fn benchmark_exists(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$9\r\nexistskey\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);
    c.bench_function("exists", |b| {
        b.iter(|| {
            writer.write_all(b"*2\r\n$6\r\nEXISTS\r\n$9\r\nexistskey\r\n").unwrap();
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

fn benchmark_mget(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$4\r\nkey1\r\n$3\r\nval\r\n").unwrap();
    read_line(&mut reader);
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$4\r\nkey2\r\n$3\r\nval\r\n").unwrap();
    read_line(&mut reader);
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$4\r\nkey3\r\n$3\r\nval\r\n").unwrap();
    read_line(&mut reader);
    c.bench_function("mget_3keys", |b| {
        b.iter(|| {
            writer.write_all(b"*4\r\n$4\r\nMGET\r\n$4\r\nkey1\r\n$4\r\nkey2\r\n$4\r\nkey3\r\n").unwrap();
            // *3 header + 3 x ($ header + value)
            for _ in 0..7 {
                read_line(&mut reader);
            }
        })
    });
}

// --- pipelining throughput ---

fn benchmark_pipeline(c: &mut Criterion) {
    start_server();
    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);

    const BATCH: usize = 50;
    let cmd = b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n";
    let pipeline: Vec<u8> = cmd.repeat(BATCH);

    let mut group = c.benchmark_group("pipeline");
    group.throughput(Throughput::Elements(BATCH as u64));
    group.bench_function("get_x50", |b| {
        b.iter(|| {
            writer.write_all(&pipeline).unwrap();
            for _ in 0..BATCH {
                read_line(&mut reader);
                read_line(&mut reader);
            }
        })
    });
    group.finish();
}

// --- concurrent clients ---

fn make_pool(size: usize) -> Vec<Arc<Mutex<(BufReader<TcpStream>, TcpStream)>>> {
    (0..size).map(|_| Arc::new(Mutex::new(connect()))).collect()
}

fn bench_concurrent_get(group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>, pool: &[Arc<Mutex<(BufReader<TcpStream>, TcpStream)>>], name: &str) {
    group.bench_function(name, |b| {
        b.iter(|| {
            let handles: Vec<_> = pool.iter().map(|conn| {
                let conn = conn.clone();
                thread::spawn(move || {
                    let mut guard = conn.lock().unwrap();
                    let (reader, writer) = &mut *guard;
                    writer.write_all(b"*2\r\n$3\r\nGET\r\n$3\r\nfoo\r\n").unwrap();
                    read_line(reader);
                    read_line(reader);
                })
            }).collect();
            for h in handles { h.join().unwrap(); }
        })
    });
}

fn bench_concurrent_set(group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>, pool: &[Arc<Mutex<(BufReader<TcpStream>, TcpStream)>>], name: &str) {
    group.bench_function(name, |b| {
        b.iter(|| {
            let handles: Vec<_> = pool.iter().map(|conn| {
                let conn = conn.clone();
                thread::spawn(move || {
                    let mut guard = conn.lock().unwrap();
                    let (reader, writer) = &mut *guard;
                    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
                    read_line(reader);
                })
            }).collect();
            for h in handles { h.join().unwrap(); }
        })
    });
}

fn benchmark_concurrent(c: &mut Criterion) {
    start_server();

    let (mut reader, mut writer) = connect();
    writer.write_all(b"*3\r\n$3\r\nSET\r\n$3\r\nfoo\r\n$3\r\nbar\r\n").unwrap();
    read_line(&mut reader);

    let pool_20 = make_pool(20);
    let pool_100 = make_pool(100);
    let pool_200 = make_pool(200);

    let mut group = c.benchmark_group("concurrent");
    group.sample_size(10);

    bench_concurrent_get(&mut group, &pool_20, "get_20clients");
    bench_concurrent_get(&mut group, &pool_100, "get_100clients");
    bench_concurrent_get(&mut group, &pool_200, "get_200clients");

    bench_concurrent_set(&mut group, &pool_20, "set_20clients");
    bench_concurrent_set(&mut group, &pool_100, "set_100clients");
    bench_concurrent_set(&mut group, &pool_200, "set_200clients");

    group.finish();
}

criterion_group!(
    benches,
    benchmark_set,
    benchmark_get,
    benchmark_del,
    benchmark_exists,
    benchmark_incr,
    benchmark_expire,
    benchmark_ttl,
    benchmark_mget,
    benchmark_pipeline,
    benchmark_concurrent,
);
criterion_main!(benches);
