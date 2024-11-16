/// Asserts that the expression matches a [`Some(_)`] variant, returning the contained value.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_some!`] for assertions that are not enabled in release builds by default.
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
/// assert_some!(maybe);
///
/// // With a custom message
/// assert_some!(maybe, "Found it at {:?}", maybe);
/// # }
/// ```
///
/// A `None` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let maybe = None;
///
/// assert_some!(maybe);  // Will panic
/// # }
/// ```
///
/// [`Some(_)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_some!`]: crate::debug_assert_some!
#[macro_export]
macro_rules! assert_some {
    ($cond:expr $(,)?) => {
        match $cond {
            ::core::option::Option::Some(t) => t,
            ::core::option::Option::None => {
                ::core::panic!("assertion failed, expected Some(_), got None");
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            ::core::option::Option::Some(t) => t,
            ::core::option::Option::None => {
                ::core::panic!("assertion failed, expected Some(_), got None: {}", ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches a [`Some(_)`] variant on debug builds.
///
/// This macro behaves nearly the same as [`assert_some!`] on debug builds, although it does not
/// return the value contained in the `Some` variant. On release builds it is a no-op.
///
/// [`Some(_)`]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.Some
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_some!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn some() {
        assert_some!(Some(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Some(_), got None")]
    fn not_some() {
        assert_some!(None::<()>);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Some(_), got None: foo")]
    fn not_some_custom_message() {
        assert_some!(None::<()>, "foo");
    }

    #[test]
    fn some_value_returned() {
        let value = assert_some!(Some(42));
        assert_eq!(value, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_some() {
        debug_assert_some!(Some(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Some(_), got None")]
    fn debug_not_some() {
        debug_assert_some!(None::<()>);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Some(_), got None: foo")]
    fn debug_not_some_custom_message() {
        debug_assert_some!(None::<()>, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_some() {
        debug_assert_some!(None::<()>);
    }

    #[test]
    fn does_not_require_some_to_impl_debug() {
        enum Foo {
            Bar,
        }

        assert_some!(Some(Foo::Bar));
    }

    #[test]
    fn debug_does_not_require_some_to_impl_debug() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_some!(Some(Foo::Bar));
    }

    #[test]
    fn does_not_require_some_to_impl_debug_custom_message() {
        enum Foo {
            Bar,
        }

        assert_some!(Some(Foo::Bar), "foo");
    }

    #[test]
    fn debug_does_not_require_some_to_impl_debug_custom_message() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_some!(Some(Foo::Bar), "foo");
    }
}
