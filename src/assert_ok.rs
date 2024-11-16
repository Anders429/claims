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
                panic!("assertion failed, expected Ok(_), got Err({:?})", e);
            }
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Ok(t) => t,
            Err(e) => {
                panic!("assertion failed, expected Ok(_), got Err({:?}): {}", e, format_args!($($arg)+));
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
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_ok!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        assert_ok!(Ok::<_, ()>(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(())")]
    fn not_ok() {
        assert_ok!(Err::<(), _>(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(()): foo")]
    fn not_ok_custom_message() {
        assert_ok!(Err::<(), _>(()), "foo");
    }

    #[test]
    fn ok_value_returned() {
        let value = assert_ok!(Ok::<_, ()>(42));
        assert_eq!(value, 42);
    }

    #[test]
    fn debug_ok() {
        debug_assert_ok!(Ok::<_, ()>(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(())")]
    fn debug_not_ok() {
        debug_assert_ok!(Err::<(), _>(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(()): foo")]
    fn debug_not_ok_custom_message() {
        debug_assert_ok!(Err::<(), _>(()), "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_ok() {
        debug_assert_ok!(Err::<(), _>(()));
    }

    #[test]
    fn does_not_require_ok_to_impl_debug() {
        enum Foo {
            Bar,
        }

        assert_ok!(Ok::<_, ()>(Foo::Bar));
    }

    #[test]
    fn debug_does_not_require_ok_to_impl_debug() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_ok!(Ok::<_, ()>(Foo::Bar));
    }
}
