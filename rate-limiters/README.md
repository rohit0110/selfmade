# Rate Limiters in Rust

Implementations of five rate limiting algorithms in Rust, with single-thread and concurrent benchmarks via [Criterion](https://github.com/bheisler/criterion.rs).

---

## Algorithms

- Token Bucket
- Fixed Window
- Leaking Bucket
- Sliding Window
- Sliding Window Counter

---

## Benchmarks

Benchmarks run with Criterion (100 measurements each). Single-thread: 20 sequential requests per iteration. Concurrent: 20 threads × 25 requests per iteration.

### Single-threaded

| Algorithm               | Time (ns) — low / mid / high |
|-------------------------|-------------------------------|
| Token Bucket            | 323.67 / 324.69 / 325.60     |
| Fixed Window            | 405.53 / 407.34 / 409.87     |
| Sliding Window          | 472.31 / 473.62 / 475.24     |
| Sliding Window Counter  | 658.30 / 658.46 / 658.66     |
| Leaking Bucket          | 715.76 / 716.12 / 716.50     |

### Concurrent (20 threads)

| Algorithm               | Time (µs) — low / mid / high |
|-------------------------|-------------------------------|
| Fixed Window            | 122.96 / 123.45 / 124.06     |
| Sliding Window          | 124.21 / 124.62 / 125.13     |
| Leaking Bucket          | 125.01 / 125.82 / 126.83     |
| Token Bucket            | 126.36 / 127.91 / 129.53     |
| Sliding Window Counter  | 125.42 / 127.07 / 129.26     |

---

## Interpretation

For single threaded, Token bucket seems to prouce the best results due to its simplicity, other algos feel slower due to complexity and single thread not letting time average out. 

The same can not be said during concurrent since the lock contention nearly balances out the time utilized by each algo.

---

## Run Benchmarks

```bash
cargo bench
```
