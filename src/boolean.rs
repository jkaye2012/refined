use std::marker::PhantomData;

use crate::Predicate;

pub struct True;

impl<T> Predicate<T> for True {
    fn test(_: &T) -> bool {
        true
    }
}

pub struct False;

impl<T> Predicate<T> for False {
    fn test(_: &T) -> bool {
        false
    }
}

pub struct And<P1, P2>(pub(crate) PhantomData<P1>, pub(crate) PhantomData<P2>);

impl<T, P1: Predicate<T>, P2: Predicate<T>> Predicate<T> for And<P1, P2> {
    fn test(t: &T) -> bool {
        P1::test(t) && P2::test(t)
    }
}

pub struct Or<P1, P2>(PhantomData<P1>, PhantomData<P2>);

impl<T, P1: Predicate<T>, P2: Predicate<T>> Predicate<T> for Or<P1, P2> {
    fn test(t: &T) -> bool {
        P1::test(t) || P2::test(t)
    }
}

pub struct Not<P>(PhantomData<P>);

impl<T, P: Predicate<T>> Predicate<T> for Not<P> {
    fn test(t: &T) -> bool {
        !P::test(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_true() {
        type Test = Refinement<String, True>;
        assert!(Test::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_false() {
        type Test = Refinement<String, False>;
        assert!(Test::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_and() {
        type TestTrueFalse = Refinement<String, And<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = Refinement<String, And<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = Refinement<String, And<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = Refinement<String, And<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_or() {
        type TestTrueFalse = Refinement<String, Or<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = Refinement<String, Or<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = Refinement<String, Or<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = Refinement<String, Or<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_not() {
        type TestTrue = Refinement<String, Not<True>>;
        assert!(TestTrue::refine("Hello".to_string()).is_err());

        type TestFalse = Refinement<String, Not<False>>;
        assert!(TestFalse::refine("Hello".to_string()).is_ok());
    }
}
