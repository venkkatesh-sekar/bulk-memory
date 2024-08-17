## Testing slice limits
```sh
dfx start --background 
export RUSTFLAGS="-Ctarget-feature=+bulk-memory"
dfx deploy


# To test memory copy
dfx canister call memory_canister memory '(0: nat8)'

Error: Failed update call.
Caused by: The replica returned a rejection error: reject code CanisterError, reject message Error from Canister hiser-juaaa-aaaaa-qaawa-cai: Canister attempted to perform a large memory operation that used 3999981591 instructions and exceeded the slice limit 2000000000., error code None

# To test memory fill
dfx canister call memory_canister memory '(1: nat8)'
Error: Failed update call.
Caused by: The replica returned a rejection error: reject code CanisterError, reject message Error from Canister hiser-juaaa-aaaaa-qaawa-cai: Canister attempted to perform a large memory operation that used 3999981589 instructions and exceeded the slice limit 2000000000., error code None
```

## Benchmarking bulk-memory
1. Install `canbench`
```sh
cargo binstall canbench
```
2. Run `canbench` without `bulk-memory` enabled
```sh
$ cat canbench.yml

build_cmd:
  # RUSTFLAGS='-C target-feature=+bulk-memory -Copt-level=3' cargo build --release -p memory_canister --target wasm32-unknown-unknown --features canbench-rs
  RUSTFLAGS='-Copt-level=3' cargo build --release -p memory_canister --target wasm32-unknown-unknown --features canbench-rs 

wasm_path:
  target/wasm32-unknown-unknown/release/memory_canister.wasm

$ canbench

---------------------------------------------------

Benchmark: benchmark_linear_extend_1kb (new)
  total:
    instructions: 23.65 B (new)
    heap_increase: 50.30 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_4kb (new)
  total:
    instructions: 23.22 B (new)
    heap_increase: 50.30 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_64kb (new)
  total:
    instructions: 24.93 B (new)
    heap_increase: 41.55 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_1mb (new)
  total:
    instructions: 26.11 B (new)
    heap_increase: 44.72 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------
```
3. Run `canbench` with `bulk-memory` enabled
```sh
$ cat canbench.yml

build_cmd:
  RUSTFLAGS='-C target-feature=+bulk-memory -Copt-level=3' cargo build --release -p memory_canister --target wasm32-unknown-unknown --features canbench-rs
  # RUSTFLAGS='-Copt-level=3' cargo build --release -p memory_canister --target wasm32-unknown-unknown --features canbench-rs 

wasm_path:
  target/wasm32-unknown-unknown/release/memory_canister.wasm

$ canbench

---------------------------------------------------

Benchmark: benchmark_linear_extend_1kb (new)
  total:
    instructions: 18.75 B (new)
    heap_increase: 50.30 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_4kb (new)
  total:
    instructions: 18.28 B (new)
    heap_increase: 50.30 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_64kb (new)
  total:
    instructions: 18.49 B (new)
    heap_increase: 41.55 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------

Benchmark: benchmark_linear_extend_1mb (new)
  total:
    instructions: 19.67 B (new)
    heap_increase: 44.72 K pages (new)
    stable_memory_increase: 0 pages (new)

---------------------------------------------------
```