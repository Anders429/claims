/// Asserts that the left expression contains a [`Some(T)`] variant and its contained value of type
/// `T` equals the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_some_eq!`] for assertions that are not enabled in release builds by default.
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
/// let maybe = Some(42);
///
/// assert_some_eq!(maybe, 42);
///
/// // With a custom message
/// assert_some_eq!(maybe, 42, "Got some value");
/// # }
/// ```
///
/// A `None` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let maybe: Option<i32> = None;
///
/// assert_some_eq!(maybe, 42);  // Will panic
/// # }
/// ```
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_some_eq!`]: crate::debug_assert_some_eq!
#[macro_export]
macro_rules! assert_some_eq {
    ($cond:expr, $expected:expr $(,)?) => {
        match $cond {
            ::core::option::Option::Some(t) => {
                ::core::assert_eq!(t, $expected);
                t
            },
            ::core::option::Option::None => {
                ::core::panic!("assertion failed, expected Some(_), got None");
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            ::core::option::Option::Some(t) => {
                ::core::assert_eq!(t, $expected, $($arg)+);
                t
            },
            ::core::option::Option::None => {
                ::core::panic!("assertion failed, expected Some(_), got None: {}", ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the left expression contains a [`Some(T)`] variant and its contained value of type
/// `T` equals the right expression on debug builds.
///
/// This macro behaves nearly the same as [`assert_some_eq!`] on debug builds, although it does not
/// return the value contained in the `Some` variant. On release builds it is a no-op.
///
/// [`Some(T)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
#[macro_export]
macro_rules! debug_assert_some_eq {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_some_eq!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        assert_some_eq!(Some(42), 42);
    }

    #[test]
    #[should_panic]
    fn not_equal() {
        assert_some_eq!(Some(42), 100);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Some(_), got None")]
    fn not_some() {
        assert_some_eq!(None::<usize>, 42);
    }

    #[test]
    #[should_panic(expected = "foo")]
    fn not_equal_custom_message() {
        assert_some_eq!(Some(1), 2, "foo");
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Some(_), got None: foo")]
    fn not_some_custom_message() {
        assert_some_eq!(None::<usize>, 2, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_equal() {
        debug_assert_some_eq!(Some(42), 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic]
    fn debug_not_equal() {
        debug_assert_some_eq!(Some(42), 100);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Some(_), got None")]
    fn debug_not_some() {
        debug_assert_some_eq!(None::<usize>, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "foo")]
    fn debug_not_equal_custom_message() {
        debug_assert_some_eq!(Some(1), 2, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Some(_), got None: foo")]
    fn debug_not_some_custom_message() {
        debug_assert_some_eq!(None::<usize>, 2, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_equal() {
        debug_assert_some_eq!(Some(42), 100);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_some() {
        debug_assert_some_eq!(None::<usize>, 42);
    }
}
