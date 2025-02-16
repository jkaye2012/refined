//! Logical type refinement.
//!
//! This module provides type refinements that act as logical combinators of other refinements.
//! These combinators allow for the creation of more complex type refinements via type aliases.
//!
//! # Example
//!
//! ```
//! use refined::{Refinement, boolean::*, boundable::unsigned::*};
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

/// Logical negated conjunction of two [predicates](Predicate).
pub type Nand<A, B> = Not<And<A, B>>;

/// Logical negated disjunction of two [predicates](Predicate).
pub type Nor<A, B> = Not<Or<A, B>>;

/// Always `true`, statefully.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TrueST;

impl<T> Predicate<T> for TrueST {
    fn test(_: &T) -> bool {
        true
    }

    fn error() -> String {
        String::from("true predicate")
    }
}

impl<T> StatefulPredicate<T> for TrueST {
    fn test(&self, value: &T) -> bool {
        <Self as Predicate<T>>::test(value)
    }

    fn error(&self) -> String {
        <Self as Predicate<T>>::error()
    }
}

/// Always `false`, statefully.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FalseST;

impl<T> Predicate<T> for FalseST {
    fn test(_: &T) -> bool {
        false
    }

    fn error() -> String {
        String::from("false predicate")
    }
}
impl<T> StatefulPredicate<T> for FalseST {
    fn test(&self, value: &T) -> bool {
        <Self as Predicate<T>>::test(value)
    }

    fn error(&self) -> String {
        <Self as Predicate<T>>::error()
    }
}

/// Logical conjunction of two [stateful predicates](StatefulPredicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AndST<A, B>(A, B);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for AndST<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) && B::test(t)
    }

    fn error() -> String {
        format!("{} and {}", A::error(), B::error())
    }
}

impl<T, A: StatefulPredicate<T>, B: StatefulPredicate<T>> StatefulPredicate<T> for AndST<A, B> {
    fn test(&self, t: &T) -> bool {
        self.0.test(t) && self.1.test(t)
    }

    fn error(&self) -> String {
        format!("{} and {}", self.0.error(), self.1.error())
    }
}

/// Logical disjunction of two [stateful predicates](StatefulPredicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OrST<A, B>(A, B);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for OrST<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) || B::test(t)
    }

    fn error() -> String {
        format!("{} or {}", A::error(), B::error())
    }
}

impl<T, A: StatefulPredicate<T>, B: StatefulPredicate<T>> StatefulPredicate<T> for OrST<A, B> {
    fn test(&self, t: &T) -> bool {
        self.0.test(t) || self.1.test(t)
    }

    fn error(&self) -> String {
        format!("{} or {}", self.0.error(), self.1.error())
    }
}

/// Logical exclusive disjunction of two [stateful predicates](StatefulPredicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XorST<A, B>(A, B);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for XorST<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) ^ B::test(t)
    }

    fn error() -> String {
        format!("{} xor {}", A::error(), B::error())
    }
}

impl<T, A: StatefulPredicate<T>, B: StatefulPredicate<T>> StatefulPredicate<T> for XorST<A, B> {
    fn test(&self, t: &T) -> bool {
        self.0.test(t) ^ self.1.test(t)
    }

    fn error(&self) -> String {
        format!("{} xor {}", self.0.error(), self.1.error())
    }
}

/// Logical negation of a [stateful predicate](StatefulPredicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotST<P>(P);

impl<T, P: Predicate<T>> Predicate<T> for NotST<P> {
    fn test(t: &T) -> bool {
        !P::test(t)
    }

    fn error() -> String {
        format!("not {}", P::error())
    }
}

impl<T, P: StatefulPredicate<T>> StatefulPredicate<T> for NotST<P> {
    fn test(&self, t: &T) -> bool {
        !self.0.test(t)
    }

    fn error(&self) -> String {
        format!("not {}", self.0.error())
    }
}

/// Logical negated conjunction of two [stateful predicates](StatefulPredicate).
pub type NandST<A, B> = NotST<AndST<A, B>>;

/// Logical negated disjunction of two [stateful predicates](StatefulPredicate).
pub type NorST<A, B> = NotST<OrST<A, B>>;

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
        type Test = StatefulRefinement<String, TrueST>;
        assert!(Test::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_false_st() {
        type Test = StatefulRefinement<String, FalseST>;
        assert!(Test::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_and_st() {
        type TestTrueFalse = StatefulRefinement<String, AndST<TrueST, FalseST>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = Refinement<String, AndST<TrueST, TrueST>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = Refinement<String, AndST<FalseST, TrueST>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = Refinement<String, AndST<FalseST, FalseST>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_or_st() {
        type TestTrueFalse = StatefulRefinement<String, OrST<TrueST, FalseST>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, OrST<TrueST, TrueST>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_ok());

        type TestFalseTrue = StatefulRefinement<String, OrST<FalseST, TrueST>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, OrST<FalseST, FalseST>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_xor_st() {
        type TestTrueFalse = StatefulRefinement<String, XorST<TrueST, FalseST>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, XorST<TrueST, TrueST>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, XorST<FalseST, TrueST>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, XorST<FalseST, FalseST>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_err());
    }

    #[test]
    fn test_not_st() {
        type TestNotTrue = StatefulRefinement<String, NotST<TrueST>>;
        assert!(TestNotTrue::refine("Hello".to_string()).is_err());

        type TestNotFalse = StatefulRefinement<String, NotST<FalseST>>;
        assert!(TestNotFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_nand_st() {
        type TestTrueFalse = StatefulRefinement<String, NandST<TrueST, FalseST>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_ok());

        type TestTrueTrue = StatefulRefinement<String, NandST<TrueST, TrueST>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, NandST<FalseST, TrueST>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_ok());

        type TestFalseFalse = StatefulRefinement<String, NandST<FalseST, FalseST>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }

    #[test]
    fn test_nor_st() {
        type TestTrueFalse = StatefulRefinement<String, NorST<TrueST, FalseST>>;
        assert!(TestTrueFalse::refine("Hello".to_string()).is_err());

        type TestTrueTrue = StatefulRefinement<String, NorST<TrueST, TrueST>>;
        assert!(TestTrueTrue::refine("Hello".to_string()).is_err());

        type TestFalseTrue = StatefulRefinement<String, NorST<FalseST, TrueST>>;
        assert!(TestFalseTrue::refine("Hello".to_string()).is_err());

        type TestFalseFalse = StatefulRefinement<String, NorST<FalseST, FalseST>>;
        assert!(TestFalseFalse::refine("Hello".to_string()).is_ok());
    }
}
