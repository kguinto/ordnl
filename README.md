# ordnl
A Rust library for formatting ordinal numbers.

[![Crate](https://img.shields.io/crates/v/ordnl.svg)](https://crates.io/crates/ordnl)
[![Docs](https://docs.rs/ordnl/badge.svg)](https://docs.rs/ordnl)

## Usage

Add this to your Cargo.toml:
```toml
[dependencies]
ordnl = "1.0.3"
```

## Example
```rust
use ordnl;

fn main() {
    println!("{}", ordnl::of_u8(0u8)); // "0th"
    println!("{}", ordnl::of_u8(1u8)); // "1st"
    println!("{}", ordnl::of_u8(2u8)); // "2nd"
    println!("{}", ordnl::of_u8(3u8)); // "3rd"
    println!("{}", ordnl::of_u8(4u8)); // "4th"

    println!("{}", ordnl::of_u8(0u8)); // "0th"
    println!("{}", ordnl::of_u16(1u16)); // "1st"
    println!("{}", ordnl::of_u32(2u32)); // "2nd"
    println!("{}", ordnl::of_u64(3u64)); // "3rd"
    println!("{}", ordnl::of_u128(4u128)); // "4th"
}
```