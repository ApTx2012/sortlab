# sortlab

> An experimental, extensible Rust library of sorting algorithms with a unified API.

[简体中文文档](README.zh-CN.md)

## Goals

- **Pluggable strategies**: a single [`Sorter`] trait implemented by every algorithm.
- **Iterator adaptors**: sort any iterator directly with `collect_sorted()`.
- **Educational focus**: clear, readable implementations over raw performance.
- **Incremental growth**: the skeleton compiles first; real algorithms land one by one.

## Quick start

```bash
cargo add sortlab
```

```rust
use sortlab::prelude::*;

fn main() {
    let mut data = vec![5, 2, 8, 1, 9, 3];
    Bubble.sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 5, 8, 9]);

    let sorted = vec![3, 1, 2].into_iter().collect_sorted();
    assert_eq!(sorted, vec![1, 2, 3]);
}
```

Enable the `parallel` feature for multi-threaded merge sort:

```toml
[dependencies]
sortlab = { version = "0.1", features = ["parallel"] }
```

```rust
use sortlab::prelude::*;

let mut data: Vec<i64> = /* large slice */;
ParallelMerge.sort(&mut data); // splits across std threads
```

## Project layout

```
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs          # crate root and re-exports
│   ├── prelude.rs      # one-stop import for users
│   ├── traits.rs       # Sorter trait and IterAdaptor
│   ├── algorithms
│   │   ├── mod.rs
│   │   ├── bubble.rs
│   │   ├── quick.rs
│   │   ├── merge.rs
│   │   └── shell.rs
│   └── utils
│       ├── mod.rs
│       └── helpers.rs  # is_sorted and other utilities
```

> **Note:** this repository starts as a skeleton. Placeholder
> implementations currently delegate to `slice::sort`; real algorithms
> will replace them step by step.

## Roadmap

- [x] `Sorter` trait and `IterAdaptor` blanket impl.
- [x] Real implementations: bubble, quick (Lomuto + median-of-three), merge (stable, O(n) scratch), shell (3x+1 gaps).
- [x] Randomized stress tests against `slice::sort`.
- [x] `parallel` feature: `ParallelMerge` using `std::thread::scope`, no third-party deps.
- [ ] Benchmarks.

## License

MIT.
