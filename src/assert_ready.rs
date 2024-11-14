/// Asserts that the expression matches a [`Poll::Ready(_)`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided with or without
/// arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// assert_ready!(res);
/// # }
/// ```
///
/// The contained value will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// let value = assert_ready!(res);
/// assert_eq!(value, 42);
/// # }
/// ```
///
/// A [`Poll::Pending`] variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res = Poll::Pending;
///
/// assert_ready!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(_)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready!`]: crate::debug_assert_ready!
#[macro_export]
macro_rules! assert_ready {
    ($cond:expr,) => {
        $crate::assert_ready!($cond);
    };
    ($cond:expr) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            p @ core::task::Poll::Pending => {
                panic!("assertion failed, expected Ready(..), got {:?}", p);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(t) => t,
            p @ core::task::Poll::Pending => {
                panic!("assertion failed, expected Ready(..), got {:?}", format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches a [`Poll::Ready(_)`] variant on debug builds.
///
/// This macro behaves nearly the same as [`assert_ready!`] on debug builds, although it does not
/// return the value contained in the `Ready` variant. On release builds it is a no-op.
///
/// [`Poll::Ready(_)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready {
    ($($arg:tt)*) => (if core::cfg!(debug_assertions) { $crate::assert_ready!($($arg)*); })
}
