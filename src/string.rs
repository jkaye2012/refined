//! [String] refinement.
//!
//! To properly refine strings at the type level, we resort to [TypeString] to lift strings into
//! the type system. See the [type_string!](crate::type_string) macro for a convenient way to create these types.
//!
//! # Example
//!
//! ```
//! use refined::{Refinement, RefinementOps, type_string, TypeString, string::StartsWith};
//!
//! type_string!(Foo, "foo");
//! type Test = Refinement<String, StartsWith<Foo>>;
//!
//! assert!(Test::refine("foobar".to_string()).is_ok());
//! assert!(Test::refine("barfoo".to_string()).is_err());
//! ```
use std::marker::PhantomData;

use crate::{Predicate, StatefulPredicate, TypeString};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StartsWith<Prefix: TypeString>(PhantomData<Prefix>);

impl<T: AsRef<str>, Prefix: TypeString> Predicate<T> for StartsWith<Prefix> {
    fn test(s: &T) -> bool {
        s.as_ref().starts_with(Prefix::VALUE)
    }

    fn error() -> String {
        format!("must start with '{}'", Prefix::VALUE)
    }
}

impl<T: AsRef<str>, Prefix: TypeString> StatefulPredicate<T> for StartsWith<Prefix> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EndsWith<Suffix: TypeString>(PhantomData<Suffix>);

impl<T: AsRef<str>, Suffix: TypeString> Predicate<T> for EndsWith<Suffix> {
    fn test(s: &T) -> bool {
        s.as_ref().ends_with(Suffix::VALUE)
    }

    fn error() -> String {
        format!("must end with '{}'", Suffix::VALUE)
    }
}

impl<T: AsRef<str>, Suffix: TypeString> StatefulPredicate<T> for EndsWith<Suffix> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Contains<Substr: TypeString>(PhantomData<Substr>);

impl<T: AsRef<str>, Substr: TypeString> Predicate<T> for Contains<Substr> {
    fn test(s: &T) -> bool {
        s.as_ref().contains(Substr::VALUE)
    }

    fn error() -> String {
        format!("must contain '{}'", Substr::VALUE)
    }
}

impl<T: AsRef<str>, Substr: TypeString> StatefulPredicate<T> for Contains<Substr> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trimmed;

impl<T: AsRef<str>> Predicate<T> for Trimmed {
    fn test(s: &T) -> bool {
        s.as_ref().trim() == s.as_ref()
    }

    fn error() -> String {
        String::from("must not start or end with whitespace")
    }
}

impl<T: AsRef<str>> StatefulPredicate<T> for Trimmed {}

#[cfg(feature = "regex")]
mod regex_pred {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Regex<S: TypeString>(PhantomData<S>);

    impl<S: TypeString, T: AsRef<str>> Predicate<T> for Regex<S> {
        fn test(s: &T) -> bool {
            regex::Regex::new(S::VALUE)
                .expect("Invalid regex")
                .is_match(s.as_ref())
        }

        fn error() -> String {
            format!("must match regular expression {}", S::VALUE)
        }
    }

    impl<S: TypeString, T: AsRef<str>> StatefulPredicate<T> for Regex<S> {}

    #[derive(Clone, Debug)]
    pub struct StatefulRegex<S: TypeString>(regex::Regex, PhantomData<S>);

    impl<S: TypeString> Default for StatefulRegex<S> {
        fn default() -> Self {
            Self(
                regex::Regex::new(S::VALUE).expect("Invalid regex"),
                PhantomData,
            )
        }
    }

    impl<S: TypeString, T: AsRef<str>> Predicate<T> for StatefulRegex<S> {
        fn test(s: &T) -> bool {
            regex::Regex::new(S::VALUE)
                .expect("Invalid regex")
                .is_match(s.as_ref())
        }

        fn error() -> String {
            format!("must match regular expression {}", S::VALUE)
        }
    }

    impl<S: TypeString, T: AsRef<str>> StatefulPredicate<T> for StatefulRegex<S> {
        fn test(&self, value: &T) -> bool {
            self.0.is_match(value.as_ref())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        type_string!(AllAs, "^a+$");

        #[test]
        fn test_regex() {
            type Test = Refinement<String, Regex<AllAs>>;
            assert!(Test::refine("aaa".to_string()).is_ok());
            assert!(Test::refine("aab".to_string()).is_err());
        }

        #[test]
        fn test_stateful_regex() {
            type Test = Refinement<String, StatefulRegex<AllAs>>;
            assert!(Test::refine("aaa".to_string()).is_ok());
            assert!(Test::refine("aab".to_string()).is_err());
        }

        #[test]
        fn test_stateful_regex_modify() {
            type Test = Refinement<String, StatefulRegex<AllAs>>;
            let it = Test::refine("aaa".to_string()).unwrap();
            let it = it.modify(|s| s + "aaa").unwrap();
            assert!(it.modify(|s| s + "b").is_err());
        }

        #[test]
        fn test_stateful_regex_replace() {
            type Test = Refinement<String, StatefulRegex<AllAs>>;
            let it = Test::refine("aaa".to_string()).unwrap();
            let it = it.replace("aaaa".to_string()).unwrap();
            assert!(it.replace("bbbb".to_string()).is_err());
        }
    }
}

#[cfg(feature = "regex")]
pub use regex_pred::*;

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

    #[cfg(feature = "regex")]
    type_string!(AllAs, "^a+$");

    #[test]
    #[cfg(feature = "regex")]
    fn test_regex() {
        type Test = Refinement<String, Regex<AllAs>>;
        assert!(Test::refine("aaa".to_string()).is_ok());
        assert!(Test::refine("aab".to_string()).is_err());
    }
}
