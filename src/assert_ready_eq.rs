/// Asserts that the left expression contains a [`Poll::Ready(T)`] variant and its contained value of type
/// `T` equals the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready_eq!`] for assertions that are not enabled in release builds by default.
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
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// assert_ready_eq!(res, Ok(42));
/// # }
/// ```
///
/// The contained value will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// let value = assert_ready_eq!(res, Ok(42));
/// assert_eq!(value, Ok(42));
/// # }
/// ```
///
/// A [`Poll::Pending`] variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_eq!(res, Ok(42));  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready_eq!`]: crate::debug_assert_ready_eq!
#[macro_export]
macro_rules! assert_ready_eq {
    ($cond:expr, $expected:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(t) => {
                assert_eq!(t, $expected);
                t
            },
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}", err_or_pending);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(t) => {
                assert_eq!(t, $expected, $($arg)+);
                t
            },
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(..)), got {:?}: {}", err_or_pending, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the left expression contains a [`Poll::Ready(T)`] variant and its contained value of type
/// `T` equals the right expression on debug builds.
///
/// This macro behaves nearly the same as [`assert_ready_eq!`] on debug builds, although it does not
/// return the value contained in the `Poll::Ready` variant. On release builds it is a no-op.
///
/// [`Poll::Ready(T)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready_eq {
    ($($arg:tt)*) => (if core::cfg!(debug_assertions) { $crate::assert_ready_eq!($($arg)*); })
}
