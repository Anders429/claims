#![no_std]

//! Additional assertion macros for testing.
//!
//! ## Available macros
//!
//! Note that, like [`core`]/[`std`] macros, all macros in this crate have [`debug_*`](#macros)
//! counterparts.
//!
//! ### Comparison
//!
//! Assertions similar to [`assert_eq`] or [`assert_ne`]:
//!
//! * [`assert_ge!`]
//! * [`assert_gt!`]
//! * [`assert_le!`]
//! * [`assert_lt!`]
//!
//! ### Matching
//!
//! * [`assert_matches!`]
//!
//! ### `Result` macros
//!
//! Assertions for [`Result`] variants:
//!
//! * [`assert_ok!`]
//! * [`assert_err!`]
//! * [`assert_ok_eq!`]
//! * [`assert_err_eq!`]
//!
//! ### `Option` macros
//!
//! Assertions for [`Option`] variants:
//!
//! * [`assert_some!`]
//! * [`assert_none!`]
//! * [`assert_some_eq!`]
//!
//! ### `Poll` macros
//!
//! Assertions for [`Poll`] variants:
//!
//! * [`assert_pending!`]
//! * [`assert_ready!`]
//! * [`assert_ready_ok!`]
//! * [`assert_ready_err!`]
//! * [`assert_ready_eq!`]
//!
//! [`core`]: https://doc.rust-lang.org/stable/core/#macros
//! [`std`]: https://doc.rust-lang.org/stable/std/#macros
//! [`Option`]: https://doc.rust-lang.org/core/option/enum.Option.html
//! [`Result`]: https://doc.rust-lang.org/core/result/enum.Result.html
//! [`Poll`]: https://doc.rust-lang.org/core/task/enum.Poll.html
//! [`assert_eq`]: https://doc.rust-lang.org/core/macro.assert_eq.html
//! [`assert_ne`]: https://doc.rust-lang.org/core/macro.assert_ne.html

mod assert_err;
mod assert_err_eq;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_matches;
mod assert_none;
mod assert_ok;
mod assert_ok_eq;
mod assert_pending;
mod assert_ready;
mod assert_ready_eq;
mod assert_ready_err;
mod assert_ready_ok;
mod assert_some;
mod assert_some_eq;
