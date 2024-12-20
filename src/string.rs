//! [String] refinement.
use std::marker::PhantomData;

use crate::Predicate;

/// A string lifted into a context where it can be used as a type.
///
/// Most string predicates require type-level strings, but currently strings are not supported
/// as const generic trait bounds. `TypeString` is a workaround for this limitation.
pub trait TypeString {
    const VALUE: &'static str;
}

/// Creates a [type-level string](TypeString).
///
/// `$name` is the name of a type to create to hold the type-level string.
/// `$value` is the string that should be lifted into the type system.
///
/// # Example
///
/// ```
/// use refined::{type_string, TypeString};
/// type_string!(FooBar, "very stringy");
/// assert_eq!(FooBar::VALUE, "very stringy");
/// ```
#[macro_export]
macro_rules! type_string {
    ($name:ident, $value:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name;

        impl TypeString for $name {
            const VALUE: &'static str = $value;
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StartsWith<Prefix: TypeString>(PhantomData<Prefix>);

impl<T: AsRef<str>, Prefix: TypeString> Predicate<T> for StartsWith<Prefix> {
    fn test(s: &T) -> bool {
        s.as_ref().starts_with(Prefix::VALUE)
    }

    fn error() -> String {
        format!("must start with '{}'", Prefix::VALUE)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EndsWith<Suffix: TypeString>(PhantomData<Suffix>);

impl<T: AsRef<str>, Suffix: TypeString> Predicate<T> for EndsWith<Suffix> {
    fn test(s: &T) -> bool {
        s.as_ref().ends_with(Suffix::VALUE)
    }

    fn error() -> String {
        format!("must end with '{}'", Suffix::VALUE)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Contains<Substr: TypeString>(PhantomData<Substr>);

impl<T: AsRef<str>, Substr: TypeString> Predicate<T> for Contains<Substr> {
    fn test(s: &T) -> bool {
        s.as_ref().contains(Substr::VALUE)
    }

    fn error() -> String {
        format!("must contain '{}'", Substr::VALUE)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Trimmed;

impl<T: AsRef<str>> Predicate<T> for Trimmed {
    fn test(s: &T) -> bool {
        s.as_ref().trim() == s.as_ref()
    }

    fn error() -> String {
        String::from("must not start or end with whitespace")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    type_string!(Foo, "foo");

    #[test]
    fn test_starts_with() {
        type Test = Refinement<String, StartsWith<Foo>>;
        assert!(Test::refine("foo123".to_string()).is_ok());
        assert!(Test::refine("notfoo".to_string()).is_err());
    }

    #[test]
    fn test_ends_with() {
        type Test = Refinement<&'static str, EndsWith<Foo>>;
        assert!(Test::refine("123foo").is_ok());
        assert!(Test::refine("foobar").is_err());
    }

    #[test]
    fn test_contains() {
        type Test = Refinement<&'static str, Contains<Foo>>;
        assert!(Test::refine("123foo456").is_ok());
        assert!(Test::refine("bar").is_err());
    }

    #[test]
    fn test_trimmed() {
        type Test = Refinement<&'static str, Trimmed>;
        assert!(Test::refine("  foo  ").is_err());
        assert!(Test::refine("foo").is_ok());
    }
}
