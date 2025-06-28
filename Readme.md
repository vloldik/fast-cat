# `fast-cat`

[![crates.io](https://img.shields.io/crates/v/fast-cat.svg)](https://crates.io/crates/fast-cat)
[![docs.rs](https://docs.rs/fast-cat/badge.svg)](https://docs.rs/fast-cat)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

An ergonomic macro for highly efficient string concatenation with a single memory allocation.

### Why use `fast-cat`?

Standard methods like `format!` or using the `+` operator can be inefficient for joining multiple strings, often leading to several memory re-allocations. This crate provides a `concat_str!` macro that solves this problem by first calculating the total size of all string parts and then performing only a **single memory allocation**.

It gives you the performance of a manually optimized `String::with_capacity` approach but with a much cleaner and more convenient syntax.

### Features

*   ⚡ **Blazingly Fast**: Significantly outperforms `format!` and `+` for string concatenation (see benchmarks below).
*   ⚙️ **Single Allocation**: Avoids intermediate allocations, reducing memory pressure and improving performance.
*   🕊️ **Zero Dependencies**: This crate has **no dependencies**, which means fast compile times and no extra bloat for your project.
*   ✍️ **Ergonomic**: Clean, intuitive syntax that is easy to read and write.

### Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
fast-cat = "0.1.0" # Replace with the latest version
```

### Usage

The `concat_str!` macro accepts any number of comma-separated arguments that can be referenced as a `&str`, including string literals, `String` variables, and `&str` slices.

```rust
use fast_cat::concat_str;

fn main() {
    let world = "world".to_string();
    let excitement = "!";

    // Mix and match literals, owned Strings, and slices
    let greeting = concat_str!("Hello, ", &world, excitement);

    assert_eq!(greeting, "Hello, world!");
}
```

### Benchmarks

Benchmarks show that `concat_str!` is not only significantly faster than standard library alternatives but also manages to outperform a hand-written, manually optimized implementation.

The test scenario involved concatenating 6 string parts of mixed types (`&str` and `String`).

| Method | Average Time (ns) | Relative Speed (Compared to `concat_str!`) |
| :--- | :---: | :---: |
| ✅ **`concat_str!` (Ours)** | **47.8 ns** | **1.0x** |
| `manual_with_capacity` | 54.2 ns | Same |
| `[T]::concat()` | 69.6 ns | ~1.5x slower |
| `addition (+)` | 222.7 ns | ~4.7x slower |
| `format!` | 256.1 ns | ~5.4x slower |

*Results were obtained on a specific machine and may vary. The relative performance is the key takeaway.*

As the data shows, `concat_str!` provides the best performance, making it an ideal choice for performance-sensitive applications.

### License

This project is licensed under either of
*   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
*   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.