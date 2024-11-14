/// Asserts that the expression matches an [`Err(_)`] variant, returning the contained value.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_err!`] for assertions that are not enabled in release builds by default.
///
/// ## Custom messages
///
/// This macro has a second form, where a custom panic message can be provided
/// with or without arguments for formatting. See [`std::fmt`] for syntax for this form.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<i32, ()> = Err(());
///
/// assert_err!(res);
///
/// // With a custom message.
/// assert_err!(res, "we are checking if there was an error with {:?}", res);
/// # }
/// ```
///
/// An `Ok(_)` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(42);
///
/// assert_err!(res);  // Will panic
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`Err(_)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`debug_assert_err!`]: crate::debug_assert_err!
#[macro_export]
macro_rules! assert_err {
    ($cond:expr $(,)?) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?})", t);
            },
            Err(e) => e,
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => {
                panic!("assertion failed, expected Err(..), got Ok({:?}): {}", t, format_args!($($arg)+));
            },
            Err(e) => e,
        }
    };
}

/// Asserts that the expression matches an [`Err(_)`] variant in debug builds.
///
/// This macro behaves nearly the same as [`assert_err!`] on debug builds, although it does not
/// return the value contained in the `Err` variant. On release builds it is a no-op.
///
/// [`Err(_)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed, expected Err(..), got Ok(42)")]
    fn default_panic_message() {
        let res: Result<i32, ()> = Ok(42);
        assert_err!(res);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expected Err(..), got Ok(42): we are checking if there was an error with Ok(42)"
    )]
    fn custom_panic_message() {
        let res: Result<i32, ()> = Ok(42);
        assert_err!(res, "we are checking if there was an error with {:?}", res);
    }

    #[test]
    fn does_not_require_err_debug() {
        enum Foo {
            Bar,
        }

        let res: Result<i32, Foo> = Err(Foo::Bar);
        let _ = assert_err!(res);
    }
}
