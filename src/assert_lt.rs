/// Asserts that the first expression is less than the second.
///
/// Requires that both expressions be comparable with `<`.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_lt!`] for assertions that are not enabled in release builds by default.
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
/// assert_lt!(1, 2);
///
/// // With a custom message
/// assert_lt!(4, 5, "Expecting that {} is less than {}", 4, 5);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// assert_lt!(5, 5);  // Will panic
/// assert_lt!(6, 5);
///
/// // With a custom message
/// assert_lt!(6, 5, "Not expecting {} to be less than {}", 6, 5);
/// # }
/// ```
///
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_lt!`]: crate::debug_assert_lt!
#[macro_export]
macro_rules! assert_lt {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
    left: `{:?}`,
    right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val < *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left < right)`
    left: `{:?}`,
    right: `{:?}`: {}"#, &*left_val, &*right_val, format_args!($($arg)+))
                }
            }
        }
    };
}

/// Asserts that the first expression is less than the second on debug builds.
///
/// This macro behaves the same as [`assert_lt!`] on debug builds. On release builds it is a no-op.
#[macro_export]
macro_rules! debug_assert_lt {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_lt!($($arg)*);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "assertion failed: `(left < right)`\n    left: `5`,\n    right: `3`")]
    fn greater_than() {
        assert_lt!(5, 3);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `(left < right)`\n    left: `3`,\n    right: `3`")]
    fn equal() {
        assert_lt!(3, 3);
    }

    #[test]
    fn less_than() {
        assert_lt!(1, 3);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `(left < right)`\n    left: `5`,\n    right: `3`: foo"
    )]
    fn greater_than_custom_message() {
        assert_lt!(5, 3, "foo");
    }

    #[test]
    #[should_panic(
        expected = "assertion failed: `(left < right)`\n    left: `3`,\n    right: `3`: foo"
    )]
    fn equal_custom_message() {
        assert_lt!(3, 3, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed: `(left < right)`\n    left: `5`,\n    right: `3`")]
    fn debug_greater_than() {
        debug_assert_lt!(5, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(expected = "assertion failed: `(left < right)`\n    left: `3`,\n    right: `3`")]
    fn debug_equal() {
        debug_assert_lt!(3, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_less_than() {
        debug_assert_lt!(1, 3);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed: `(left < right)`\n    left: `5`,\n    right: `3`: foo"
    )]
    fn debug_greater_than_custom_message() {
        debug_assert_lt!(5, 3, "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed: `(left < right)`\n    left: `3`,\n    right: `3`: foo"
    )]
    fn debug_equal_custom_message() {
        debug_assert_lt!(3, 3, "foo");
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_greater_than() {
        debug_assert_lt!(5, 3);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_equal() {
        debug_assert_lt!(3, 3);
    }
}
