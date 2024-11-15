/// Asserts that the left expression contains an [`Err(E)`] variant and its contained value of type
/// `E` equals the right expression.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled. See
/// [`debug_assert_err_eq!`] for assertions that are not enabled in release builds by default.
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
/// let res: Result<(), i32> = Err(1);
///
/// assert_err_eq!(res, 1);
///
/// // With a custom message
/// assert_err_eq!(res, 1, "Everything is good with {:?}", res);
/// # }
/// ```
///
/// The contained value will be returned from the macro call:
///
/// ```rust
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<(), i32> = Err(1);
///
/// let value = assert_err_eq!(res, 1);
/// assert_eq!(value, 1);
/// # }
/// ```
///
/// An `Ok(_)` variant will panic:
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let res: Result<(), i32> = Ok(());
///
/// assert_err_eq!(res, 1);  // Will panic
/// # }
/// ```
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_err_eq!`]: crate::debug_assert_err_eq!
#[macro_export]
macro_rules! assert_err_eq {
    ($cond:expr, $expected:expr $(,)?) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(_), got {:?}", ok);
            }
        }
    };
    ($cond:expr, $expected:expr, $($arg:tt)+) => {
        match $cond {
            Err(t) => {
                assert_eq!(t, $expected, $($arg)+);
                t
            },
            ok @ Ok(..) => {
                panic!("assertion failed, expected Err(_), got {:?}: {}", ok, format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the left expression contains an [`Err(E)`] variant and its contained value of type
/// `E` equals the right expression in debug builds.
///
/// This macro behaves nearly the same as [`assert_err_eq!`] on debug builds, although it does not
/// return the value contained in the `Err` variant. On release builds it is a no-op.
///
/// [`Err(E)`]: https://doc.rust-lang.org/core/result/enum.Result.html#variant.Err
#[macro_export]
macro_rules! debug_assert_err_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_err_eq!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    fn equal() {
        let _ = assert_err_eq!(Err::<(), _>(42), 42);
    }

    #[test]
    #[should_panic]
    fn not_equal() {
        let _ = assert_err_eq!(Err::<(), _>(42), 100);
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(())")]
    fn not_err() {
        let _ = assert_err_eq!(Ok::<_, usize>(()), 42);
    }

    #[test]
    #[should_panic(expected = "foo")]
    fn not_equal_custom_message() {
        let _ = assert_err_eq!(Err::<(), _>(1), 2, "foo");
    }

    #[test]
    #[should_panic(expected = "assertion failed, expected Err(_), got Ok(()): foo")]
    fn not_err_custom_message() {
        let _ = assert_err_eq!(Ok::<_, usize>(()), 2, "foo");
    }
}
