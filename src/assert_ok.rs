/// Asserts that the expression matches an [`Ok(_)`] variant, returning the contained value.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ok!`] for assertions that are not enabled in release builds by default.
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
/// let res: Result<i32, ()> = Ok(1);
///
/// assert_ok!(res);
///
/// // With a custom message
/// assert_ok!(res, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// The contained value will be returned from the macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<i32, ()> = Ok(1);
///
/// let value = assert_ok!(res);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// An `Err(..)` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res = Err(());
///
/// assert_ok!(res);  // Will panic
/// # }
/// ```
///
/// [`Ok(_)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ok!`]: crate::debug_assert_ok!
#[macro_export]
macro_rules! assert_ok {
    ($cond:expr $(,)?) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?})", e);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(..), got Err({:?}): {}", e, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches an [`Ok(_)`] variant on debug builds.
///
/// This macro behaves nearly the same as [`assert_ok!`] on debug builds, although it does not
/// return the value contained in the `Ok` variant. On release builds it is a no-op.
///
/// [`Ok(_)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
#[macro_export]
macro_rules! debug_assert_ok {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_ok!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(..), got Err(())")]
    fn default_panic_message() {
        let res = Err(());
        assert_ok!(res);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expected Ok(..), got Err(()): Everything is good with Err(())"
    )]
    fn custom_panic_message() {
        let res = Err(());
        assert_ok!(res, "Everything is good with {:?}", res);
    }

    #[test]
    fn does_not_require_ok_debug() {
        enum Foo {
            Bar,
        }

        let res: Result<Foo, ()> = Ok(Foo::Bar);
        let _ = assert_ok!(res);
    }
}
