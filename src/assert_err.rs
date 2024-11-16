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
            Err(e) => e,
            Ok(t) => panic!("assertion failed, expected Err(_), got Ok({:?})", t),
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        match $cond {
            Err(e) => e,
            Ok(t) => panic!("assertion failed, expected Err(_), got Ok({:?}): {}", t, format_args!($($arg)+)),
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
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_err!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn err() {
        assert_err!(Err::<(), _>(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(())")]
    fn not_err() {
        assert_err!(Ok::<_, ()>(()));
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(()): foo")]
    fn not_err_custom_message() {
        assert_err!(Ok::<_, ()>(()), "foo");
    }

    #[test]
    fn err_value_returned() {
        let value = assert_err!(Err::<(), _>(42));
        assert_eq!(value, 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_err() {
        debug_assert_err!(Err::<(), _>(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(())")]
    fn debug_not_err() {
        debug_assert_err!(Ok::<_, ()>(()));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(()): foo")]
    fn debug_not_err_custom_message() {
        debug_assert_err!(Ok::<_, ()>(()), "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_err() {
        debug_assert_err!(Ok::<_, ()>(()));
    }

    #[test]
    fn does_not_require_err_to_impl_debug() {
        enum Foo {
            Bar,
        }

        assert_err!(Err::<(), _>(Foo::Bar));
    }

    #[test]
    fn debug_does_not_require_err_to_impl_debug() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_err!(Err::<(), _>(Foo::Bar));
    }

    #[test]
    fn does_not_require_err_to_impl_debug_custom_message() {
        enum Foo {
            Bar,
        }

        assert_err!(Err::<(), _>(Foo::Bar), "foo");
    }

    #[test]
    fn debug_does_not_require_err_to_impl_debug_custom_message() {
        #[allow(dead_code)]
        enum Foo {
            Bar,
        }

        debug_assert_err!(Err::<(), _>(Foo::Bar), "foo");
    }
}
