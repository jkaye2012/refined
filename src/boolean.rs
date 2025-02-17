//! Logical type refinement.
//!
//! This module provides type refinements that act as logical combinators of other refinements.
//! These combinators allow for the creation of more complex type refinements via type aliases.
//!
//! # Example
//!
//! ```
//! use refined::{Refinement, RefinementOps, boolean::*, boundable::unsigned::*};
//!
//! type SizedString = Refinement<String, And<GreaterThan<3>, LessThan<10>>>;
//!
//! let ok_string = SizedString::refine("Good size".to_string());
//! assert!(ok_string.is_ok());
//!
//! let not_ok_string = SizedString::refine("Way too long I'm afraid".to_string());
//! assert!(not_ok_string.is_err());
//! ```

use std::marker::PhantomData;

use crate::{Predicate, StatefulPredicate};

/// Always `true`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct True;

impl<T> Predicate<T> for True {
    fn test(_: &T) -> bool {
        true
    }

    fn error() -> String {
        String::from("true predicate")
    }
}

impl<T> StatefulPredicate<T> for True {}

/// Always `false`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct False;

impl<T> Predicate<T> for False {
    fn test(_: &T) -> bool {
        false
    }

    fn error() -> String {
        String::from("false predicate")
    }
}

impl<T> StatefulPredicate<T> for False {}

/// Logical conjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct And<A, B>(pub(crate) PhantomData<A>, pub(crate) PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for And<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) && B::test(t)
    }

    fn error() -> String {
        format!("{} and {}", A::error(), B::error())
    }
}

impl<T, A: Predicate<T> + Default, B: Predicate<T> + Default> StatefulPredicate<T> for And<A, B> {}

/// Logical disjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Or<A, B>(PhantomData<A>, PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for Or<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) || B::test(t)
    }

    fn error() -> String {
        format!("{} or {}", A::error(), B::error())
    }
}

impl<T, A: Predicate<T> + Default, B: Predicate<T> + Default> StatefulPredicate<T> for Or<A, B> {}

/// Logical exclusive disjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Xor<A, B>(PhantomData<A>, PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for Xor<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) ^ B::test(t)
    }

    fn error() -> String {
        format!("{} xor {}", A::error(), B::error())
    }
}

impl<T, A: Predicate<T> + Default, B: Predicate<T> + Default> StatefulPredicate<T> for Xor<A, B> {}

/// Logical negation of a [predicate](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Not<P>(PhantomData<P>);

impl<T, P: Predicate<T>> Predicate<T> for Not<P> {
    fn test(t: &T) -> bool {
        !P::test(t)
    }

    fn error() -> String {
        format!("not {}", P::error())
    }
}

impl<T, P: Predicate<T> + Default> StatefulPredicate<T> for Not<P> {}

/// Logical negated conjunction of two [predicates](Predicate).
pub type Nand<A, B> = Not<And<A, B>>;

/// Logical negated disjunction of two [predicates](Predicate).
pub type Nor<A, B> = Not<Or<A, B>>;

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

    #[test]
    fn test_xor() {
        type TestTrueFalse = Refinement<String, Xor<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = Refinement<String, Xor<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = Refinement<String, Xor<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = Refinement<String, Xor<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_nand() {
        type TestTrueFalse = Refinement<String, Nand<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = Refinement<String, Nand<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = Refinement<String, Nand<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = Refinement<String, Nand<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_nor() {
        type TestTrueFalse = Refinement<String, Nor<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = Refinement<String, Nor<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = Refinement<String, Nor<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = Refinement<String, Nor<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_true_st() {
        type Test = StatefulRefinement<String, True>;
        assert!(Test::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_false_st() {
        type Test = StatefulRefinement<String, False>;
        assert!(Test::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_and_st() {
        type TestTrueFalse = StatefulRefinement<String, And<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = Refinement<String, And<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = Refinement<String, And<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = Refinement<String, And<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_or_st() {
        type TestTrueFalse = StatefulRefinement<String, Or<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, Or<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = StatefulRefinement<String, Or<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, Or<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_xor_st() {
        type TestTrueFalse = StatefulRefinement<String, Xor<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, Xor<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, Xor<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, Xor<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_not_st() {
        type TestNotTrue = StatefulRefinement<String, Not<True>>;
        assert!(TestNotTrue::refine("Hello".to_string()).is_err());

        type TestNotFalse = StatefulRefinement<String, Not<False>>;
        assert!(TestNotFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_nand_st() {
        type TestTrueFalse = StatefulRefinement<String, Nand<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, Nand<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, Nand<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, Nand<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_nor_st() {
        type TestTrueFalse = StatefulRefinement<String, Nor<True, False>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = StatefulRefinement<String, Nor<True, True>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, Nor<False, True>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = StatefulRefinement<String, Nor<False, False>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }
}
