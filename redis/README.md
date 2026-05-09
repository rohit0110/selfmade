# Redis in Rust

A Redis-compatible server built from scratch in Rust. Implements the RESP2 protocol over TCP with an async event-loop concurrency model (tokio) and a shared in-memory store.

---

## Architecture

```
src/
  main.rs        ← entry point, binds port, starts tokio runtime
  lib.rs         ← module declarations
  redis.rs       ← TCP listener, async connection loop, task spawning
  resp.rs        ← RESP2 parser (async) and serializer
  commands.rs    ← command dispatch
  store.rs       ← in-memory HashMap store
```

**Request flow:**
```
Client → TCP → RESP parser → command handler → store → RESP serializer → TCP → Client
```

**Concurrency:** uses tokio's async runtime. Each client connection is a lightweight task (not an OS thread). The store is shared across tasks via `Arc<Mutex<Store>>`, with the lock held only during store operations. The event loop (kqueue on macOS, epoll on Linux) notifies the runtime when a socket has data ready, so no threads are blocked waiting on I/O.

---

## Protocol

Implements [RESP2](https://redis.io/docs/latest/develop/reference/protocol-spec/). Clients always send commands as arrays of bulk strings. The server responds with the appropriate RESP type per command.

---

## Supported Commands

| Command | Description | Response |
|---------|-------------|----------|
| `PING` | Health check | `+PONG` |
| `SET key value` | Set a key | `+OK` |
| `GET key` | Get a value | Bulk string or null |
| `DEL key` | Delete a key | `:1` if deleted, `:0` if not found |
| `EXISTS key` | Check if key exists | `:1` or `:0` |
| `INCR key` | Increment integer value | New integer value |
| `MGET key1 key2 ...` | Get multiple keys | Array of bulk strings |
| `EXPIRE key seconds` | Set TTL on a key | `:1` if set, `:0` if key not found |
| `TTL key` | Get remaining TTL | Seconds remaining, `:-1` if no expiry |

---

## Run

```bash
cargo run -- 7878
```

## Test with redis-cli

```bash
redis-cli -p 7878 ping
redis-cli -p 7878 set foo bar
redis-cli -p 7878 get foo
redis-cli -p 7878 mget foo goo hoo
```

## Test with nc

```bash
printf "*1\r\n\$4\r\nPING\r\n" | nc localhost 7878
printf "*3\r\n\$3\r\nSET\r\n\$3\r\nfoo\r\n\$3\r\nbar\r\n" | nc localhost 7878
printf "*2\r\n\$3\r\nGET\r\n\$3\r\nfoo\r\n" | nc localhost 7878
```

---

## Benchmarks

Run with:
```bash
cargo bench
```

HTML reports are generated at `target/criterion/report/index.html`.

### Single-client latency (round-trip per command)

| Command | Time |
|---------|------|
| `get` | ~13.7 µs |
| `exists` | ~13.7 µs |
| `ttl` | ~13.8 µs |
| `incr` | ~13.9 µs |
| `set` | ~14.2 µs |
| `mget (3 keys)` | ~15.5 µs |
| `del` (set + del) | ~28.0 µs |
| `expire` (set + expire) | ~28.6 µs |

### Pipelining throughput

| Benchmark | Time | Throughput |
|-----------|------|------------|
| `get x50` (50 commands, 1 write) | ~86 µs | ~579k commands/sec |

Pipelining is ~8x faster per command than single round-trip (1.72 µs vs 13.7 µs), since it eliminates the per-command network round-trip overhead.

### Concurrent clients

| Benchmark | Clients | Total time | Per-client |
|-----------|---------|------------|------------|
| `get` | 20 | ~219 µs | ~11.0 µs |
| `get` | 100 | ~1.11 ms | ~11.1 µs |
| `get` | 200 | ~2.29 ms | ~11.4 µs |
| `set` | 20 | ~223 µs | ~11.2 µs |
| `set` | 100 | ~1.11 ms | ~11.1 µs |
| `set` | 200 | ~2.28 ms | ~11.4 µs |

Scaling is linear from 20 to 200 clients — per-client time stays flat at ~11 µs regardless of concurrency, indicating no significant mutex contention at this scale.

---

## Interpretation

Redis isnt multithreaded, uses event loop instead of multithreading since operations are extrememly cheap in latency terms, do not add any major difference on top of the Network I/O latency

DEL and EXPIRE have higher latency in this due to SET key done before hand, makes 2 round trips.

Redis keeps operations and communcaition lightweight for fast processing

Aso didnt implement KEYS *, since you should never use it anyway as it pauses the event loop to scan over all the keys
