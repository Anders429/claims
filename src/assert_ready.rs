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
    ($cond:expr $(,)?) => {
        match $cond {
            ::core::task::Poll::Ready(t) => t,
            ::core::task::Poll::Pending => {
                ::core::panic!("assertion failed, expected Ready(_), got Pending");
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::task::Poll::Ready(t) => t,
            ::core::task::Poll::Pending => {
                ::core::panic!("assertion failed, expected Ready(_), got Pending: {}", ::core::format_args!($($arg)+));
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
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_ready!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    use core::task::Poll::{Pending, Ready};

    #[test]
    fn ready() {
        assert_ready!(Ready(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(_), got Pending")]
    fn not_ready() {
        assert_ready!(Pending::<()>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ready(_), got Pending: foo")]
    fn not_ready_custom_message() {
        assert_ready!(Pending::<()>, "foo");
    }

    #[test]
    fn ready_value_returned() {
        let value = assert_ready!(Ready(42));
        assert_eq!(value, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_ready() {
        debug_assert_ready!(Ready(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(_), got Pending")]
    fn debug_not_ready() {
        debug_assert_ready!(Pending::<()>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ready(_), got Pending: foo")]
    fn debug_not_ready_custom_message() {
        debug_assert_ready!(Pending::<()>, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_ready() {
        debug_assert_ready!(Pending::<()>);
    }

    #[test]
    fn does_not_require_ready_to_impl_debug() {
        enum Foo {
            Bar,
        }

        assert_ready!(Ready(Foo::Bar));
    }

    #[test]
    fn debug_does_not_require_ready_to_impl_debug() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_ready!(Ready(Foo::Bar));
    }

    #[test]
    fn does_not_require_ready_to_impl_debug_custom_message() {
        enum Foo {
            Bar,
        }

        assert_ready!(Ready(Foo::Bar), "foo");
    }

    #[test]
    fn debug_does_not_require_ready_to_impl_debug_custom_message() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_ready!(Ready(Foo::Bar), "foo");
    }
}
