/// Asserts that the expression matches a [`Poll::Ready(Ok(_))`] variant, returning the contained
/// value.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready_ok!`] for assertions that are not enabled in release builds by default.
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
/// assert_ready_ok!(res);
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
/// let value = assert_ready_ok!(res);
/// assert_eq!(value, 42);
/// # }
/// ```
///
/// Both `Poll::Ready(Err(..))` and [`Poll::Pending`] variants will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Err(()));
///
/// assert_ready_ok!(res);  // Will panic
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_ok!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(Ok(T))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready_ok!`]: crate::debug_assert_ready_ok
#[macro_export]
macro_rules! assert_ready_ok {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(Ok(t)) => t,
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(_)), got {:?}", err_or_pending);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(Ok(t)) => t,
            err_or_pending => {
                panic!("assertion failed, expected Ready(Ok(_)), got {:?}: {}", err_or_pending, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches a [`Poll::Ready(Ok(_))`] variant on debug builds.
///
/// This macro behaves nearly the same as [`assert_ready_ok!`] on debug builds, although it does not
/// return the value contained in the `Poll::Ready(Ok(_))` variant. On release builds it is a no-op.
///
/// [`Poll::Ready(Ok(_))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready_ok {
    ($($arg:tt)*) => (if core::cfg!(debug_assertions) { $crate::assert_ready_ok!($($arg)*); })
}

#[cfg(test)]
mod tests {
    use core::task::Poll::{Pending, Ready};

    #[test]
    fn ready_ok() {
        assert_ready_ok!(Ready(Ok::<_, ()>(())));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Ready(Err(()))")]
    fn ready_err() {
        assert_ready_ok!(Ready(Err::<(), _>(())));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Pending")]
    fn not_ready() {
        assert_ready_ok!(Pending::<Result<(), ()>>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Ready(Err(())): foo")]
    fn ready_err_custom_message() {
        assert_ready_ok!(Ready(Err::<(), _>(())), "foo");
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Pending: foo")]
    fn not_ready_custom_message() {
        assert_ready_ok!(Pending::<Result<(), ()>>, "foo");
    }

    #[test]
    fn ready_ok_value_returned() {
        let value = assert_ready_ok!(Ready(Ok::<_, ()>(42)));
        assert_eq!(value, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_ready_ok() {
        debug_assert_ready_ok!(Ready(Ok::<_, ()>(())));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Ready(Err(()))")]
    fn debug_ready_err() {
        debug_assert_ready_ok!(Ready(Err::<(), _>(())));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Pending")]
    fn debug_not_ready() {
        debug_assert_ready_ok!(Pending::<Result<(), ()>>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Ready(Err(())): foo")]
    fn debug_ready_err_custom_message() {
        debug_assert_ready_ok!(Ready(Err::<(), _>(())), "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Ok(_)), got Pending: foo")]
    fn debug_not_ready_custom_message() {
        debug_assert_ready_ok!(Pending::<Result<(), ()>>, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_ready_err() {
        debug_assert_ready_ok!(Ready(Err::<(), _>(())));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_ready() {
        debug_assert_ready_ok!(Pending::<Result<(), ()>>);
    }
}
