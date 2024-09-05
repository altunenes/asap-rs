# asap-rs
[![crates.io](https://img.shields.io/crates/v/asap-rs.svg)](https://crates.io/crates/asap-rs)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
[![Rust](https://github.com/altunenes/asap-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/altunenes/asap-rs/actions/workflows/rust.yml)

A Rust implementation of ASAP (Automatic Smoothing for Attention Prioritization), based on the paper "ASAP: Prioritizing Attention via Time Series Smoothing" learn more: [source paper & js code: ](https://github.com/stanford-futuredata/ASAP)

This project provides a high-performance Rust implementation of the ASAP algorithm for time series smoothing. It aims to efficiently reduce complexity in time series data while preserving significant trends, optimized for visualization purposes.

### Usage

```rust
use asap_rs::smooth;
fn main() {
    // Example data
    let data = [1.0, 3.2, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let resolution = 2;
    let smoothed_data = smooth(&data, resolution);
}
```

### Rust vs JS Performance

```diff
🦀 Rust: 1.4776ms
🟨 JS: 8.96ms
  Data size: 500000
  Smoothed size: 25
  Original mean: 2499.97
  Smoothed mean: 2499.97
```
