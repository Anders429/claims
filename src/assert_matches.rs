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
        #[allow(unreachable_patterns)]
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => {
                ::core::panic!(r#"assertion failed, expression does not match the given pattern.
    expression: {:?}
    pattern: {}"#, other, ::core::stringify!($($pattern)|+ $(if $guard)?));
            }
        }
    };
    ($expression:expr, $($pattern:pat)|+ $(if $guard:expr)?, $($arg:tt)+) => {
        #[allow(unreachable_patterns)]
        match $expression {
            $($pattern)|+ $(if $guard)? => {},
            other => {
                ::core::panic!(r#"assertion failed, expression does not match the given pattern.
    expression: {:?}
    pattern: {}: {}"#, other, ::core::stringify!($($pattern)|+ $(if $guard)?), ::core::format_args!($($arg)+));
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

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Foo {
        Bar(usize),
        Baz(usize),
    }

    #[test]
    fn matches() {
        assert_matches!(Foo::Bar(42), Foo::Bar(_));
    }

    #[test]
    fn matches_multiple_variants() {
        assert_matches!(Foo::Baz(42), Foo::Bar(_) | Foo::Baz(_));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Baz(_)"
    )]
    fn not_matches() {
        assert_matches!(Foo::Bar(42), Foo::Baz(_));
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Baz(_): foo"
    )]
    fn not_matches_custom_message() {
        assert_matches!(Foo::Bar(42), Foo::Baz(_), "foo");
    }

    #[test]
    fn matches_if_guard() {
        assert_matches!(Foo::Bar(42), Foo::Bar(x) if x < 100);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Bar(x) if x > 100"
    )]
    fn not_matches_if_guard() {
        assert_matches!(Foo::Bar(42), Foo::Bar(x) if x > 100);
    }

    #[test]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Bar(x) if x > 100: foo"
    )]
    fn not_matches_if_guard_custom_message() {
        assert_matches!(Foo::Bar(42), Foo::Bar(x) if x > 100, "foo");
    }

    #[rustversion::since(1.53)]
    #[test]
    fn matches_nested_pattern() {
        assert_matches!(Some(Foo::Bar(42)), Some(Foo::Bar(_) | Foo::Baz(1 | 2)));
    }

    #[rustversion::since(1.53)]
    #[test]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: None\n    pattern: Some(Foo::Bar(_) | Foo::Baz(1 | 2))"
    )]
    fn not_matches_nested_pattern() {
        assert_matches!(None, Some(Foo::Bar(_) | Foo::Baz(1 | 2)));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_matches() {
        debug_assert_matches!(Foo::Bar(42), Foo::Bar(_));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_matches_multiple_variants() {
        debug_assert_matches!(Foo::Baz(42), Foo::Bar(_) | Foo::Baz(_));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Baz(_)"
    )]
    fn debug_not_matches() {
        debug_assert_matches!(Foo::Bar(42), Foo::Baz(_));
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Baz(_): foo"
    )]
    fn debug_not_matches_custom_message() {
        debug_assert_matches!(Foo::Bar(42), Foo::Baz(_), "foo");
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_matches_if_guard() {
        debug_assert_matches!(Foo::Bar(42), Foo::Bar(x) if x < 100);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Bar(x) if x > 100"
    )]
    fn debug_not_matches_if_guard() {
        debug_assert_matches!(Foo::Bar(42), Foo::Bar(x) if x > 100);
    }

    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: Bar(42)\n    pattern: Foo::Bar(x) if x > 100: foo"
    )]
    fn debug_not_matches_if_guard_custom_message() {
        debug_assert_matches!(Foo::Bar(42), Foo::Bar(x) if x > 100, "foo");
    }

    #[rustversion::since(1.53)]
    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    fn debug_matches_nested_pattern() {
        debug_assert_matches!(Some(Foo::Bar(42)), Some(Foo::Bar(_) | Foo::Baz(1 | 2)));
    }

    #[rustversion::since(1.53)]
    #[test]
    #[cfg_attr(not(debug_assertions), ignore = "only run in debug mode")]
    #[should_panic(
        expected = "assertion failed, expression does not match the given pattern.\n    expression: None\n    pattern: Some(Foo::Bar(_) | Foo::Baz(1 | 2))"
    )]
    fn debug_not_matches_nested_pattern() {
        debug_assert_matches!(None, Some(Foo::Bar(_) | Foo::Baz(1 | 2)));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_matches() {
        debug_assert_matches!(Foo::Bar(42), Foo::Baz(_));
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_matches_if_guard() {
        debug_assert_matches!(Foo::Bar(42), Foo::Bar(x) if x > 100);
    }

    #[rustversion::since(1.53)]
    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn debug_release_not_matches_nested_pattern() {
        debug_assert_matches!(None, Some(Foo::Bar(_) | Foo::Baz(1 | 2)));
    }
}
