# Changelog

## Unreleased
### Changed
- `assert_err_eq`, `assert_ok_eq`, and `assert_some_eq` now properly display custom messages.
- MSRV raised to `1.36.0`.
- Rust edition bumped to 2018.
### Removed
- Dependency on `autocfg`.

## 0.7.1 - 2022-08-31
### Changed
- Fixed release CI job.

## 0.7.0 - 2022-08-31
### Added
- `assert_err_eq!` macro. Thanks @Anders429!

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
- `assert_matches!` macro.

## 0.2.0 - 2020-03-09
### Added
- `assert_some_eq!` macro.
- `assert_ok_eq!` macro.
### Changed
- Ensuring support for older Rust versions.
