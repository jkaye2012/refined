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

#[cfg(feature = "alloc")]
use alloc::format;
use core::marker::PhantomData;

use crate::{ErrorMessage, Predicate};

/// Always `true`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct True;

impl<T> Predicate<T> for True {
    fn test(_: &T) -> bool {
        true
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        ErrorMessage::from("true predicate")
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "true predicate"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

/// Always `false`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct False;

impl<T> Predicate<T> for False {
    fn test(_: &T) -> bool {
        false
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        ErrorMessage::from("false predicate")
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "false predicate"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

/// Logical conjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct And<A, B>(pub(crate) PhantomData<A>, pub(crate) PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for And<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) && B::test(t)
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        format!("{} and {}", A::error(), B::error())
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "conjunction"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

/// Logical disjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Or<A, B>(PhantomData<A>, PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for Or<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) || B::test(t)
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        format!("{} or {}", A::error(), B::error())
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "disjunction"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

/// Logical exclusive disjunction of two [predicates](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Xor<A, B>(PhantomData<A>, PhantomData<B>);

impl<T, A: Predicate<T>, B: Predicate<T>> Predicate<T> for Xor<A, B> {
    fn test(t: &T) -> bool {
        A::test(t) ^ B::test(t)
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        format!("{} xor {}", A::error(), B::error())
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "exclusive disjunction"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

/// Logical negation of a [predicate](Predicate).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Not<P>(PhantomData<P>);

impl<T, P: Predicate<T>> Predicate<T> for Not<P> {
    fn test(t: &T) -> bool {
        !P::test(t)
    }

    #[cfg(feature = "alloc")]
    fn error() -> ErrorMessage {
        format!("not {}", P::error())
    }

    #[cfg(not(feature = "alloc"))]
    fn error() -> ErrorMessage {
        "negation"
    }

    unsafe fn optimize(value: &T) {
        core::hint::assert_unchecked(Self::test(value));
    }
}

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
        type Test = Refinement<u8, True>;
        assert!(Test::refine(123).is_ok());
    }

    #[test]
    fn test_false() {
        type Test = Refinement<u8, False>;
        assert!(Test::refine(123).is_err());
    }

    #[test]
    fn test_and() {
        type TestTrueFalse = Refinement<u8, And<True, False>>;
        assert!(TestTrueFalse::refine(123).is_err());

        type TestTrueTrue = Refinement<u8, And<True, True>>;
        assert!(TestTrueTrue::refine(123).is_ok());

        type TestFalseTrue = Refinement<u8, And<False, True>>;
        assert!(TestFalseTrue::refine(123).is_err());

        type TestFalseFalse = Refinement<u8, And<False, False>>;
        assert!(TestFalseFalse::refine(123).is_err());
    }

    #[test]
    fn test_or() {
        type TestTrueFalse = Refinement<u8, Or<True, False>>;
        assert!(TestTrueFalse::refine(123).is_ok());

        type TestTrueTrue = Refinement<u8, Or<True, True>>;
        assert!(TestTrueTrue::refine(123).is_ok());

        type TestFalseTrue = Refinement<u8, Or<False, True>>;
        assert!(TestFalseTrue::refine(123).is_ok());

        type TestFalseFalse = Refinement<u8, Or<False, False>>;
        assert!(TestFalseFalse::refine(123).is_err());
    }

    #[test]
    fn test_not() {
        type TestTrue = Refinement<u8, Not<True>>;
        assert!(TestTrue::refine(123).is_err());

        type TestFalse = Refinement<u8, Not<False>>;
        assert!(TestFalse::refine(123).is_ok());
    }

    #[test]
    fn test_xor() {
        type TestTrueFalse = Refinement<u8, Xor<True, False>>;
        assert!(TestTrueFalse::refine(123).is_ok());

        type TestTrueTrue = Refinement<u8, Xor<True, True>>;
        assert!(TestTrueTrue::refine(123).is_err());

        type TestFalseTrue = Refinement<u8, Xor<False, True>>;
        assert!(TestFalseTrue::refine(123).is_ok());

        type TestFalseFalse = Refinement<u8, Xor<False, False>>;
        assert!(TestFalseFalse::refine(123).is_err());
    }

    #[test]
    fn test_nand() {
        type TestTrueFalse = Refinement<u8, Nand<True, False>>;
        assert!(TestTrueFalse::refine(123).is_ok());

        type TestTrueTrue = Refinement<u8, Nand<True, True>>;
        assert!(TestTrueTrue::refine(123).is_err());

        type TestFalseTrue = Refinement<u8, Nand<False, True>>;
        assert!(TestFalseTrue::refine(123).is_ok());

        type TestFalseFalse = Refinement<u8, Nand<False, False>>;
        assert!(TestFalseFalse::refine(123).is_ok());
    }

    #[test]
    fn test_nor() {
        type TestTrueFalse = Refinement<u8, Nor<True, False>>;
        assert!(TestTrueFalse::refine(123).is_err());

        type TestTrueTrue = Refinement<u8, Nor<True, True>>;
        assert!(TestTrueTrue::refine(123).is_err());

        type TestFalseTrue = Refinement<u8, Nor<False, True>>;
        assert!(TestFalseTrue::refine(123).is_err());

        type TestFalseFalse = Refinement<u8, Nor<False, False>>;
        assert!(TestFalseFalse::refine(123).is_ok());
    }
}
