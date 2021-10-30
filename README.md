### KVS - Key-Value Store

This is a log structured key-value store.

While it's not trying to become a production ready alternative to any of the key stores,

It still aims to implement all the basic features of a log-structured key-value store serving as an learning tool,
prioritizing simplicity over performance


#### Currently implemented features

- CLI: this supports a cli interface for acessing kvs. run `cargo run -- --help` for more details
- KVS currently uses a single WAL for writing logs, a memory index is maintained for finding the elements from logs for reads
- Currently KVS supports a naive form of compaction where all the current data is compacted into a single compact log, this is not yet sharded.


#### Pending Implementation

- Concurrency & Multithreading: use separate threads for reading & share the in-memory indexes between threads
- Sharding: Shard the compacted logs for better performance
- Other key optimizations