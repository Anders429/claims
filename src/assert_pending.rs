/// Asserts that the expression matches a [`Poll::Pending`] variant.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_pending!`] for assertions that are not enabled in release builds by default.
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
/// let res: Poll<i32> = Poll::Pending;
///
/// assert_pending!(res);
///
/// // With a custom message
/// assert_pending!(res, "Future is not ready yet");
/// # }
/// ```
///
/// A [`Poll::Pending`] variant will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<i32> = Poll::Pending;
///
/// let value = assert_pending!(res);
/// assert_eq!(value, Poll::Pending);
/// # }
/// ```
///
/// A [`Poll::Ready(_)`] variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res = Poll::Ready(42);
///
/// assert_pending!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(_)`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_pending!`]: crate::debug_assert_pending!
#[macro_export]
macro_rules! assert_pending {
    ($cond:expr $(,)?) => {
        match $cond {
            pending @ ::core::task::Poll::Pending => pending,
            ready @ ::core::task::Poll::Ready(_) => {
                ::core::panic!("assertion failed, expected Pending, got {:?}", ready);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            pending @ ::core::task::Poll::Pending => pending,
            ready @ ::core::task::Poll::Ready(_) => {
                ::core::panic!("assertion failed, expected Pending, got {:?}: {}", ready, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches a [`Poll::Pending`] variant on debug builds.
///
/// This macro behaves the same as [`assert_pending!`] on debug builds. On release builds it is a
/// no-op.
///
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
#[macro_export]
macro_rules! debug_assert_pending {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        #[cfg(debug_assertions)]
        {
            $crate::assert_pending!($($arg)*);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll::{Pending, Ready};

    #[test]
    fn pending() {
        let _ = assert_pending!(Pending::<()>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Pending, got Ready(())")]
    fn not_pending() {
        let _ = assert_pending!(Ready(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Pending, got Ready(()): foo")]
    fn not_pending_custom_message() {
        let _ = assert_pending!(Ready(()), "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_pending() {
        debug_assert_pending!(Pending::<()>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Pending, got Ready(())")]
    fn debug_not_pending() {
        debug_assert_pending!(Ready(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Pending, got Ready(()): foo")]
    fn debug_not_pending_custom_message() {
        debug_assert_pending!(Ready(()), "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_pending() {
        debug_assert_pending!(Ready(()));
    }
}
