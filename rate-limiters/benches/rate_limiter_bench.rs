use criterion::{criterion_group, criterion_main, Criterion};
use rate_limiters::token_bucket::TokenBucket;
use rate_limiters::RateLimiter;
use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};

fn benchmark_token_bucket(c: &mut Criterion) {
    c.bench_function("token_bucket_single_thread", |b| {
        b.iter_with_setup(|| TokenBucket::new(5,Duration::from_millis(100)), |mut limiter| {
            for _ in 0..20 {
                limiter.check();
            }
        })
    } );
}

fn benchmark_token_bucket_concurrent_20(c: &mut Criterion) {
    c.bench_function("token_bucket_multiple_thread_20", |b|{
        b.iter_with_setup(|| Arc::new(Mutex::new(TokenBucket::new(100, Duration::from_millis(100)))), |mut limiter| {
            let mut handles = vec![];
            for _ in 0..20 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                        for _ in 0..25 {
                            clone.lock().unwrap().check();
                        }
                    });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

fn benchmark_token_bucket_concurrent_50(c: &mut Criterion) {
    c.bench_function("token_bucket_multiple_thread_50", |b|{
        b.iter_with_setup(|| Arc::new(Mutex::new(TokenBucket::new(100, Duration::from_millis(100)))), |mut limiter| {
            let mut handles = vec![];
            for _ in 0..50 {
                let clone = limiter.clone();
                let handle = thread::spawn(move || {
                        for _ in 0..25 {
                            clone.lock().unwrap().check();
                        }
                    });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

criterion_group!(benches, benchmark_token_bucket, benchmark_token_bucket_concurrent_20, benchmark_token_bucket_concurrent_50);
criterion_main!(benches);