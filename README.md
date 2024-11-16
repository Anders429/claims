# claims

[![crates.io](https://img.shields.io/crates/v/claims.svg)](https://crates.io/crates/claims)
[![docs.rs](https://docs.rs/claims/badge.svg)](https://docs.rs/claims)
[![Build Status](https://github.com/Anders429/claims/workflows/Continuous%20Integration/badge.svg)](https://github.com/Anders429/claims/actions)
[![License](https://img.shields.io/crates/l/claims)](#license)
[![MSRV](https://img.shields.io/badge/rustc-1.38.0+-yellow.svg)](#minimum-supported-rust-version)

Additional assertion macros for testing.

This crate provides assertion macros that are missing in the Rust standard library:

* Comparison: [`assert_ge`](https://docs.rs/claims/latest/claims/macro.assert_ge.html), [`assert_gt`](https://docs.rs/claims/latest/claims/macro.assert_gt.html), [`assert_le`](https://docs.rs/claims/latest/claims/macro.assert_le.html), and [`assert_lt`](https://docs.rs/claims/latest/claims/macro.assert_lt.html).
* Matching: [`assert_matches`](https://docs.rs/claims/latest/claims/macro.assert_matches.html).
* `Result`: [`assert_ok`](https://docs.rs/claims/latest/claims/macro.assert_ok.html), [`assert_err`](https://docs.rs/claims/latest/claims/macro.assert_err.html), [`assert_ok_eq`](https://docs.rs/claims/latest/claims/macro.assert_ok_eq.html), and [`assert_err_eq`](https://docs.rs/claims/latest/claims/macro.assert_err_eq.html).
* `Option`: [`assert_some`](https://docs.rs/claims/latest/claims/macro.assert_some.html), [`assert_none`](https://docs.rs/claims/latest/claims/macro.assert_none.html), and [`assert_some_eq`](https://docs.rs/claims/latest/claims/macro.assert_some_eq.html).
* `Poll`: [`assert_pending`](https://docs.rs/claims/latest/claims/macro.assert_pending.html), [`assert_ready`](https://docs.rs/claims/latest/claims/macro.assert_ready.html), [`assert_ready_ok`](https://docs.rs/claims/latest/claims/macro.assert_ready_ok.html), [`assert_ready_err`](https://docs.rs/claims/latest/claims/macro.assert_ready_err.html), and [`assert_ready_eq`](https://docs.rs/claims/latest/claims/macro.assert_ready_eq.html).

## Installation

Add the following to your `Cargo.toml` manifest to use this crate for tests, examples, and benchmarks:

```toml
[dev-dependencies]
claims = "0.7"
```

## Usage

Check out the [documentation](https://docs.rs/claims) for available macros and examples.

## Minimum Supported Rust Version
This crate is guaranteed to compile on stable `rustc 1.38.0` and up.

## License

Licensed under either of [Apache License 2.0](https://github.com/Anders429/claims/blob/master/LICENSE-APACHE)
or [MIT license](https://github.com/Anders429/claims/blob/master/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
