# claims

Missing assertion macros for Rust.

[![crates.io](https://img.shields.io/crates/v/claims.svg)](https://crates.io/crates/claims)
[![docs.rs](https://docs.rs/claims/badge.svg)](https://docs.rs/claims)
[![Build Status](https://github.com/Anders429/claims/workflows/Continuous%20Integration/badge.svg)](https://github.com/Anders429/claims/actions)
[![License](https://img.shields.io/crates/l/claims)](#license)
[![MSRV](https://img.shields.io/badge/rustc-1.36.0+-yellow.svg)](#minimum-supported-rust-version)

This crate provides assertion macros that are missing in the Rust standard library:

* Comparison: `assert_ge`, `assert_gt`, `assert_le`, and `assert_lt`.
* Matching: `assert_matches`.
* `Result`: `assert_ok`, `assert_err`, `assert_ok_eq`, and `assert_err_eq`.
* `Option`: `assert_some`, `assert_none`, and `assert_some_eq`.
* `Poll`: `assert_pending`, `assert_ready`, `assert_ready_ok`, `assert_ready_err`, and `assert_ready_eq`.

## Installation

Add the following to your `Cargo.toml` manifest to use this crate for tests, examples, and benchmarks:

```toml
[dev-dependencies]
claims = "0.7"
```

## Usage

Check out the [documentation](https://docs.rs/claims) for available macros and examples.

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.36.0` and up.

## License

Licensed under either of [Apache License 2.0](https://github.com/Anders429/claims/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/Anders429/claims/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
