# Changelog

## Unreleased
### Changed
- `assert_err_eq`, `assert_ok_eq`, and `assert_some_eq` now properly display custom messages.
- MSRV raised to `1.36.0`.
- Rust edition bumped to 2018.
### Fixed
- Corrected the name of `debug_assert_ready_ok_eq!` to instead by `debug_assert_ready_eq!`.
### Removed
- Dependency on `autocfg`.

## 0.7.1 - 2022-08-31
### Changed
- Fixed release CI job.

## 0.7.0 - 2022-08-31
### Added
- `assert_err_eq!` and `debug_assert_err_eq!` macros. Thanks @Anders429!

## 0.6.0 - 2022-08-30

### Changed
- Forked the [original project](https://github.com/svartalf/rust-claim) as the maintainer is unreachable at the moment. Renamed crate to `claims`.
- Fixed versioning conflict with `autocfg` v1.x, thanks @Turbo87!

## 0.5.0 - 2021-02-04
### Changed
- Fixed compatibility for Rust versions <= `1.6.0`.
- Fixed compatibility with Rust version `1.30.0`.
- MSRV for `assert_matches!` macro was changed from Rust `1.32.0` to `1.26.0`.

## 0.4.0 - 2020-10-26
### Changed
- Removed `Debug` requirement for `Ok(T)` in `assert_err!` macro.
- Removed `Debug` requirement for `Err(e)` in `assert_ok!` macro.

## 0.3.1 - 2020-03-13
### Changed
- Minimal required Rust version for `assert_matches!` macro downgraded from the `1.37` to `1.32`.

## 0.3.0 - 2020-03-13
### Added
- `assert_matches!` and `debug_assert_matches!` macros.

## 0.2.0 - 2020-03-09
### Added
- `assert_some_eq!` and `debug_assert_some_eq!` macros.
- `assert_ok_eq!`  and `debug_assert_ok_eq!` macros.
### Changed
- Ensuring support for older Rust versions.

## 0.1.1 - 2020-03-08
### Changed
- Improved documentation.

## 0.1.0 - 2020-03-07
### Added
- `assert_err!` and `debug_assert_err!` macros.
- `assert_ge!` and `debug_assert_ge!` macros.
- `assert_gt!` and `debug_assert_gt!` macros.
- `assert_le!` and `debug_assert_le!` macros.
- `assert_lt!` and `debug_assert_lt!` macros.
- `assert_none!` and `debug_assert_none!` macros.
- `assert_ok!` and `debug_assert_ok!` macros.
- `assert_pending!` and `debug_assert_pending!` macros.
- `assert_ready!` and `debug_assert_ready!` macros.
- `assert_ready_eq!` and `debug_assert_ready_eq!` macros.
- `assert_ready_err!` and `debug_assert_ready_err!` macros.
- `assert_ready_ok!` and `debug_assert_ready_ok!` macros.
- `assert_some!` and `debug_assert_some!` macros.
