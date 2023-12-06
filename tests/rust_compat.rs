//! Test syntax compatible with Rust ignoring proc macros.

#![cfg_attr(feature = "used_linker", feature(used_with_arg))]
use linkme::distributed_slice;

#[distributed_slice]
pub static SLICE1: [u32; 0] = [];

#[distributed_slice(SLICE1)]
static ONE: u32 = 1;

#[distributed_slice(SLICE1)]
static TWO: u32 = 2;

#[test]
fn test_rust_compat() {
    assert_eq!(2, SLICE1.len());
}
