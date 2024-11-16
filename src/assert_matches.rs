/// Asserts that the expression matches the provided pattern.
///
/// Works like the [`std::matches!`] macro, but panics if there is no match.
///
/// ## Uses
///
/// Assertions are always checked in both debug and release builds, and cannot be disabled.
/// See [`debug_assert_matches!`] for assertions that are not enabled in release builds by default.
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
/// let foo = 'f';
/// assert_matches!(foo, 'A'..='Z' | 'a'..='z');
///
/// // With a custom message
/// assert_matches!(foo, 'A'..='Z' | 'a'..='z', "expecting it to be letter: {}", foo);
/// # }
/// ```
///
/// ```rust,should_panic
/// # #[macro_use] extern crate claims;
/// # fn main() {
/// let bar: Option<i32> = None;
/// assert_matches!(bar, Some(x) if x > 2);  // Will panic
/// # }
/// ```
///
/// [`std::matches!`]: https://doc.rust-lang.org/stable/std/macro.matches.html
/// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
/// [`debug_assert_matches!`]: crate::debug_assert_matches!
#[macro_export]
macro_rules! assert_matches {
    ($expression:expr, $($pattern:pat)|+ $(if $guard:expr)? $(,)?) => {
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}"#, other, stringify!($($pattern)|+));
            }
        }
    };
    ($expression:expr, $($pattern:pat)|+ $(if $guard:expr)?, $($arg:tt)+) => {
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => {
                panic!(r#"assertion failed, expression does not match any of the given variants.
    expression: {:?}
    variants: {}: {}"#, other, stringify!($($pattern)|+), format_args!($($arg)+));
            }
        }
    };
}

/// Asserts that the expression matches the provided pattern on debug builds.
///
///
/// This macro behaves the same as [`assert_matches!`] on debug builds. On release builds it is a
/// no-op.
#[macro_export]
macro_rules! debug_assert_matches {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        $crate::assert_matches!($($arg)*);
    }
}
