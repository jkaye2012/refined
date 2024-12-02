use std::marker::PhantomData;

use crate::Predicate;

pub trait TypeString {
    const VALUE: &'static str;
}

#[macro_export]
macro_rules! type_string {
    ($name:ident, $value:literal) => {
        pub struct $name;

        impl TypeString for $name {
            const VALUE: &'static str = $value;
        }
    };
}

pub struct StartsWith<Prefix: TypeString>(PhantomData<Prefix>);

impl<T: AsRef<str>, Prefix: TypeString> Predicate<T> for StartsWith<Prefix> {
    fn test(s: &T) -> bool {
        s.as_ref().starts_with(Prefix::VALUE)
    }
}

pub struct EndsWith<Suffix: TypeString>(PhantomData<Suffix>);

impl<T: AsRef<str>, Suffix: TypeString> Predicate<T> for EndsWith<Suffix> {
    fn test(s: &T) -> bool {
        s.as_ref().ends_with(Suffix::VALUE)
    }
}

pub struct Contains<Substr: TypeString>(PhantomData<Substr>);

impl<T: AsRef<str>, Substr: TypeString> Predicate<T> for Contains<Substr> {
    fn test(s: &T) -> bool {
        s.as_ref().contains(Substr::VALUE)
    }
}

pub struct Trimmed;

impl<T: AsRef<str>> Predicate<T> for Trimmed {
    fn test(s: &T) -> bool {
        s.as_ref().trim() == s.as_ref()
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
