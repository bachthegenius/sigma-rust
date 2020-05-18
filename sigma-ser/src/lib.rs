//! Sigma serialization

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Sigma serializer
pub mod serializer;
/// VLQ encoder
pub mod vlq_encode;
/// ZigZag encoder
pub mod zig_zag_encode;

// #[cfg(test)]
pub mod test_helpers;
