/// Asserts that the left expression contains an [`Ok(T)`] variant and its contained value of type
/// `T` equals the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ok_eq!`] for assertions that are not enabled in release builds by default.
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
/// assert_ok_eq!(res, 1);
///
/// // With a custom message
/// assert_ok_eq!(res, 1, "Everything is good with {:?}", res);
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
/// let value = assert_ok_eq!(res, 1);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// An `Err(_)` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<i32, ()> = Err(());
///
/// assert_ok_eq!(res, 1);  // Will panic
/// # }
/// ```
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ok_eq!`]: crate::debug_assert_ok_eq!
#[macro_export]
macro_rules! assert_ok_eq {
    ($cond:expr, $expected:expr $(,)?) => {
        match $cond {
            ::core::result::Result::Ok(t) => {
                ::core::assert_eq!(t, $expected);
                t
            },
            e @ ::core::result::Result::Err(_) => {
                ::core::panic!("assertion failed, expected Ok(_), got {:?}", e);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            ::core::result::Result::Ok(t) => {
                ::core::assert_eq!(t, $expected, $($arg)+);
                t
            },
            e @ ::core::result::Result::Err(_) => {
                ::core::panic!("assertion failed, expected Ok(_), got {:?}: {}", e, ::core::format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the left expression contains an [`Ok(T)`] variant and its contained value of type
/// `T` equals the right expression on debug builds.
///
/// This macro behaves nearly the same as [`assert_ok_eq!`] on debug builds, although it does not
/// return the value contained in the `Ok` variant. On release builds it is a no-op.
///
/// [`Ok(T)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Ok
#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_ok_eq!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        assert_ok_eq!(Ok::<_, ()>(42), 42);
    }

    #[test]
    #[should_panic]
    fn not_equal() {
        assert_ok_eq!(Ok::<_, ()>(42), 100);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(())")]
    fn not_ok() {
        assert_ok_eq!(Err::<usize, _>(()), 42);
    }

    #[test]
    #[should_panic(expected = "foo")]
    fn not_equal_custom_message() {
        assert_ok_eq!(Ok::<_, ()>(1), 2, "foo");
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(()): foo")]
    fn not_ok_custom_message() {
        assert_ok_eq!(Err::<usize, ()>(()), 2, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_equal() {
        debug_assert_ok_eq!(Ok::<_, ()>(42), 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic]
    fn debug_not_equal() {
        debug_assert_ok_eq!(Ok::<_, ()>(42), 100);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(())")]
    fn debug_not_ok() {
        debug_assert_ok_eq!(Err::<usize, _>(()), 42);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "foo")]
    fn debug_not_equal_custom_message() {
        debug_assert_ok_eq!(Ok::<_, ()>(1), 2, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed, expected Ok(_), got Err(()): foo")]
    fn debug_not_ok_custom_message() {
        debug_assert_ok_eq!(Err::<usize, ()>(()), 2, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_equal() {
        debug_assert_ok_eq!(Ok::<_, ()>(42), 100);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_ok() {
        debug_assert_ok_eq!(Err::<usize, _>(()), 42);
    }
}
