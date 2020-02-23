//! A Rust library for formatting ordinal numbers.
//!
//! # Example
//!
//! ```
//!
//! fn main() {
//!     println!("{}", ordnl::of_u8(0u8)); // "0th"
//!     println!("{}", ordnl::of_u8(1u8)); // "1st"
//!     println!("{}", ordnl::of_u8(2u8)); // "2nd"
//!     println!("{}", ordnl::of_u8(3u8)); // "3rd"
//!     println!("{}", ordnl::of_u8(4u8)); // "4th"
//!
//!     println!("{}", ordnl::of_u8(0u8)); // "0th"
//!     println!("{}", ordnl::of_u16(1u16)); // "1st"
//!     println!("{}", ordnl::of_u32(2u32)); // "2nd"
//!     println!("{}", ordnl::of_u64(3u64)); // "3rd"
//!     println!("{}", ordnl::of_u128(4u128)); // "4th"
//! }

/// Ordinalizes a `u8` integer.
///
/// ```
/// let first = ordnl::of_u8(1u8);
/// let second = ordnl::of_u8(2u8);
/// let third = ordnl::of_u8(3u8);
/// let fourth = ordnl::of_u8(4u8);
///
/// assert_eq!(first, "1st");
/// assert_eq!(second, "2nd");
/// assert_eq!(third, "3rd");
/// assert_eq!(fourth, "4th");
///
/// ```
pub fn of_u8(n: u8) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

/// Ordinalizes a `u16` integer.
///
/// ```
/// let first = ordnl::of_u16(1u16);
/// let second = ordnl::of_u16(2u16);
/// let third = ordnl::of_u16(3u16);
/// let fourth = ordnl::of_u16(4u16);
///
/// assert_eq!(first, "1st");
/// assert_eq!(second, "2nd");
/// assert_eq!(third, "3rd");
/// assert_eq!(fourth, "4th");
///
/// ```
pub fn of_u16(n: u16) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

/// Ordinalizes a `u32` integer.
///
/// ```
/// let first = ordnl::of_u32(1u32);
/// let second = ordnl::of_u32(2u32);
/// let third = ordnl::of_u32(3u32);
/// let fourth = ordnl::of_u32(4u32);
///
/// assert_eq!(first, "1st");
/// assert_eq!(second, "2nd");
/// assert_eq!(third, "3rd");
/// assert_eq!(fourth, "4th");
///
/// ```
pub fn of_u32(n: u32) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

/// Ordinalizes a `u64` integer.
///
/// ```
/// let first = ordnl::of_u64(1u64);
/// let second = ordnl::of_u64(2u64);
/// let third = ordnl::of_u64(3u64);
/// let fourth = ordnl::of_u64(4u64);
///
/// assert_eq!(first, "1st");
/// assert_eq!(second, "2nd");
/// assert_eq!(third, "3rd");
/// assert_eq!(fourth, "4th");
///
/// ```
pub fn of_u64(n: u64) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}

/// Ordinalizes a `u128` integer.
///
/// ```
/// let first = ordnl::of_u128(1u128);
/// let second = ordnl::of_u128(2u128);
/// let third = ordnl::of_u128(3u128);
/// let fourth = ordnl::of_u128(4u128);
///
/// assert_eq!(first, "1st");
/// assert_eq!(second, "2nd");
/// assert_eq!(third, "3rd");
/// assert_eq!(fourth, "4th");
///
/// ```
pub fn of_u128(n: u128) -> String {
    match (n % 10, (n % 100) < 11 || (n % 100) > 13) {
        (1, true) => format!("{}st", n),
        (2, true) => format!("{}nd", n),
        (3, true) => format!("{}rd", n),
        _ => format!("{}th", n),
    }
}
