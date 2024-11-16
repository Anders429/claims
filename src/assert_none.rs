/// Asserts that the expression is [`None`].
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_none!`] for assertions that are not enabled in release builds by default.
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
/// # fn main() {
/// let maybe: Option<i32> = None;
///
/// assert_none!(maybe);
///
/// // With a custom message
/// assert_none!(maybe, "Yep, there is nothing in here");
/// assert_none!(maybe, "we asserting that there are no droids we are looking for at {:?}", maybe);
/// # }
/// ```
///
/// A `Some(_)` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let maybe = Some(42i32);
///
/// assert_none!(maybe);  // Will panic
/// # }
/// ```
///
/// [`None`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_none!`]: crate::debug_assert_none!
#[macro_export]
macro_rules! assert_none {
    ($cond:expr $(,)?) => {
        match $cond {
            none @ ::core::option::Option::None => none,
            some @ ::core::option::Option::Some(_) => {
                ::core::panic!("assertion failed, expected None, got {:?}", some);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            none @ ::core::option::Option::None => none,
            some @ ::core::option::Option::Some(_) => {
                ::core::panic!("assertion failed, expected None, got {:?}: {}", some, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression is [`None`] on debug builds.
///
/// This macro behaves the same as [`assert_none!`] on debug builds. On release builds it is a
/// no-op.
#[macro_export]
macro_rules! debug_assert_none {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_none!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn none() {
        assert_none!(None::<()>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected None, got Some(())")]
    fn not_none() {
        assert_none!(Some(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected None, got Some(()): foo")]
    fn not_none_custom_message() {
        assert_none!(Some(()), "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_none() {
        debug_assert_none!(None::<()>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected None, got Some(())")]
    fn debug_not_none() {
        debug_assert_none!(Some(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected None, got Some(()): foo")]
    fn debug_not_none_custom_message() {
        debug_assert_none!(Some(()), "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_none() {
        debug_assert_none!(Some(()));
    }
}
