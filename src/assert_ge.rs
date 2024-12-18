/// Asserts that the first expression is greater than or equal to the second.
///
/// Requires that both expressions be comparable with `>=`.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_ge!`] for assertions that are not enabled in release builds by default.
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
/// assert_ge!(2, 1);
///
/// // With a custom message.
/// assert_ge!(2, 1, "Expecting that {} is greater or equal than {}", 2, 1);
/// assert_ge!(5, 5, "Expecting that both arguments are equal");
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// assert_ge!(5, 6);  // Will panic
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_ge!`]: crate::debug_assert_ge!
#[macro_export]
macro_rules! assert_ge {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::core::panic!(r#"assertion failed: `(left >= right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val >= *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    ::core::panic!(r#"assertion failed: `(left >= right)`
    left: `{:?}`,
    right: `{:?}`: {}"#, &*left_val, &*right_val, ::core::format_args!($($arg)+))
                }
            }
        }
    };
}

/// Asserts that the first expression is greater than or equal to the second on debug builds.
///
/// This macro behaves the same as [`assert_ge!`] on debug builds. On release builds it is a no-op.
#[macro_export]
macro_rules! debug_assert_ge {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_ge!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn greater_than() {
        assert_ge!(5, 3);
    }

    #[test]
    fn equal() {
        assert_ge!(3, 3);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `(left >= right)`\n    left: `1`,\n    right: `3`"
    )]
    fn less_than() {
        assert_ge!(1, 3);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `(left >= right)`\n    left: `1`,\n    right: `3`: foo"
    )]
    fn less_than_custom_message() {
        assert_ge!(1, 3, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_greater_than() {
        debug_assert_ge!(5, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_equal() {
        debug_assert_ge!(3, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed: `(left >= right)`\n    left: `1`,\n    right: `3`"
    )]
    fn debug_less_than() {
        debug_assert_ge!(1, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed: `(left >= right)`\n    left: `1`,\n    right: `3`: foo"
    )]
    fn debug_less_than_custom_message() {
        debug_assert_ge!(1, 3, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_less_than() {
        debug_assert_ge!(1, 3);
    }
}
