/// Asserts that the expression matches a [`Poll::Ready(Err(_))`] variant, returning the contained
/// value.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ready_err!`] for assertions that are not enabled in release builds by default.
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
/// # #[cfg(feature = "std")] extern crate std as core;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Err(()));
///
/// assert_ready_err!(res);
/// # }
/// ```
///
/// The contained value will also be returned from this macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, String>> = Poll::Ready(Err("Something went wrong".to_string()));
///
/// let message = assert_ready_err!(res);
/// assert_eq!("Something went wrong", message);
/// # }
/// ```
///
/// Both `Poll::Ready(Ok(..))` and [`Poll::Pending`] variants will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Ready(Ok(42));
///
/// assert_ready_err!(res);  // Will panic
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # use std::task::Poll;
/// # fn main() {
/// let res: Poll<Result<i32, ()>> = Poll::Pending;
///
/// assert_ready_err!(res);  // Will panic
/// # }
/// ```
///
/// [`Poll::Ready(Err(_))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
/// [`Poll::Pending`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Pending
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ready_err!`]: crate::debug_assert_ready_err!
#[macro_export]
macro_rules! assert_ready_err {
    ($cond:expr $(,)?) => {
        match $cond {
            core::task::Poll::Ready(Err(e)) => e,
            ok_or_pending => {
                panic!("assertion failed, expected Ready(Err(_)), got {:?}", ok_or_pending);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            core::task::Poll::Ready(Err(e)) => e,
            ok_or_pending => {
                panic!("assertion failed, expected Ready(Err(_)), got {:?}: {}", ok_or_pending, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches a [`Poll::Ready(Err(_))`] variant on debug builds.
///
/// This macro behaves nearly the same as [`assert_ready_err!`] on debug builds, although it does not
/// return the value contained in the `Poll::Ready(Err(_))` variant. On release builds it is a no-op.
///
/// [`Poll::Ready(Err(_))`]: https://doc.rust-lang.org/core/task/enum.Poll.html#variant.Ready
#[macro_export]
macro_rules! debug_assert_ready_err {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_ready_err!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll::{Pending, Ready};

    #[test]
    fn ready_err() {
        assert_ready_err!(Ready(Err::<(), _>(())));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Ready(Ok(()))")]
    fn ready_ok() {
        assert_ready_err!(Ready(Ok::<_, ()>(())));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Pending")]
    fn not_ready() {
        assert_ready_err!(Pending::<Result<(), ()>>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Ready(Ok(())): foo")]
    fn ready_ok_custom_message() {
        assert_ready_err!(Ready(Ok::<_, ()>(())), "foo");
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Pending: foo")]
    fn not_ready_custom_message() {
        assert_ready_err!(Pending::<Result<(), ()>>, "foo");
    }

    #[test]
    fn ready_err_value_returned() {
        let value = assert_ready_err!(Ready(Err::<(), _>(42)));
        assert_eq!(value, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_ready_err() {
        debug_assert_ready_err!(Ready(Err::<(), _>(())));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Ready(Ok(()))")]
    fn debug_ready_ok() {
        debug_assert_ready_err!(Ready(Ok::<_, ()>(())));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Pending")]
    fn debug_not_ready() {
        debug_assert_ready_err!(Pending::<Result<(), ()>>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Ready(Ok(())): foo")]
    fn debug_ready_ok_custom_message() {
        debug_assert_ready_err!(Ready(Ok::<_, ()>(())), "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(Err(_)), got Pending: foo")]
    fn debug_not_ready_custom_message() {
        debug_assert_ready_err!(Pending::<Result<(), ()>>, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_ready_ok() {
        debug_assert_ready_err!(Ready(Ok::<_, ()>(())));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_ready() {
        debug_assert_ready_err!(Pending::<Result<(), ()>>);
    }
}
