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
            Some(t) => t,
            None => {
                panic!("assertion failed, expected Some(..), got None");
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Some(t) => t,
            None => {
                panic!("assertion failed, expected Some(..), got None: {}", format_args!($($arg)+));
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
    ($($arg:tt)*) => (if core::cfg!(debug_assertions) { $crate::assert_some!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed, expected Some(..), got None")]
    fn default_panic_message() {
        let maybe: Option<i32> = None;
        let _ = assert_some!(maybe);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expected Some(..), got None: we are checking if there was an error with None"
    )]
    fn custom_panic_message() {
        let maybe: Option<i32> = None;
        let _ = assert_some!(
            maybe,
            "we are checking if there was an error with {:?}",
            maybe
        );
    }

    #[test]
    fn does_not_require_some_debug() {
        enum Foo {
            Bar,
        }

        let res: Option<Foo> = Some(Foo::Bar);
        let _ = assert_some!(res);
    }
}
