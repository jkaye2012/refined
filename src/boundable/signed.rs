//! Boundable refinement via signed values.
//!
//! # Example
//!
//! ```
//! use refined::{Refinement, RefinementOps, boundable::signed::ClosedInterval};
//!
//! type RangedI64 = Refinement<i64, ClosedInterval<-10, 10>>;
//!
//! let ok = RangedI64::refine(0);
//! assert!(ok.is_ok());
//!
//! let not_ok = RangedI64::refine(11);
//! assert!(not_ok.is_err());
//! ```
use crate::{boolean::*, Predicate};

/// Types that can be reduced to a signed size so that they can be bounded.
pub trait SignedBoundable {
    fn bounding_value(&self) -> isize;
}

impl SignedBoundable for i8 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl SignedBoundable for std::num::Saturating<i8> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl SignedBoundable for std::num::NonZeroI8 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl SignedBoundable for i16 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl SignedBoundable for std::num::Saturating<i16> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl SignedBoundable for std::num::NonZeroI16 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl SignedBoundable for i32 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl SignedBoundable for std::num::Saturating<i32> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl SignedBoundable for std::num::NonZeroI32 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl SignedBoundable for isize {
    fn bounding_value(&self) -> isize {
        *self
    }
}

impl SignedBoundable for std::num::Saturating<isize> {
    fn bounding_value(&self) -> isize {
        self.0
    }
}

impl SignedBoundable for std::num::NonZeroIsize {
    fn bounding_value(&self) -> isize {
        self.get()
    }
}

#[cfg(target_pointer_width = "64")]
impl SignedBoundable for i64 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl SignedBoundable for std::num::Saturating<i64> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl SignedBoundable for std::num::NonZeroI64 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GreaterThan<const MIN: isize>;

pub type GT<const MIN: isize> = GreaterThan<MIN>;

impl<T: SignedBoundable, const MIN: isize> Predicate<T> for GreaterThan<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() > MIN
    }

    fn error() -> String {
        format!("must be greater than {}", MIN)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GreaterThanEqual<const MIN: isize>;

pub type GTE<const MIN: isize> = GreaterThanEqual<MIN>;

impl<T: SignedBoundable, const MIN: isize> Predicate<T> for GreaterThanEqual<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() >= MIN
    }

    fn error() -> String {
        format!("must be greater than or equal to {}", MIN)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LessThan<const MAX: isize>;

pub type LT<const MAX: isize> = LessThan<MAX>;

impl<T: SignedBoundable, const MAX: isize> Predicate<T> for LessThan<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() < MAX
    }

    fn error() -> String {
        format!("must be less than {}", MAX)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LessThanEqual<const MAX: isize>;

pub type LTE<const MAX: isize> = LessThanEqual<MAX>;

impl<T: SignedBoundable, const MAX: isize> Predicate<T> for LessThanEqual<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() <= MAX
    }

    fn error() -> String {
        format!("must be less than or equal to {}", MAX)
    }
}

pub type OpenInterval<const MIN: isize, const MAX: isize> = And<GT<MIN>, LT<MAX>>;

pub type OpenClosedInterval<const MIN: isize, const MAX: isize> = And<GT<MIN>, LTE<MAX>>;

pub type ClosedOpenInterval<const MIN: isize, const MAX: isize> = And<GTE<MIN>, LT<MAX>>;

pub type ClosedInterval<const MIN: isize, const MAX: isize> = And<GTE<MIN>, LTE<MAX>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Modulo<const DIV: isize, const MOD: isize>;

impl<T: SignedBoundable, const DIV: isize, const MOD: isize> Predicate<T> for Modulo<DIV, MOD> {
    fn test(value: &T) -> bool {
        value.bounding_value() % DIV == MOD
    }

    fn error() -> String {
        format!("must be divisible by {} with a remainder of {}", DIV, MOD)
    }
}

pub type Divisible<const DIV: isize> = Modulo<DIV, 0>;

pub type Even = Modulo<2, 0>;

pub type Odd = Not<Even>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Equals<const VAL: isize>;

impl<T: SignedBoundable, const VAL: isize> Predicate<T> for Equals<VAL> {
    fn test(value: &T) -> bool {
        value.bounding_value() == VAL
    }

    fn error() -> String {
        format!("must be equal to {}", VAL)
    }
}

pub type Zero = Equals<0>;

pub type NonZero = Not<Zero>;

pub type Positive = GT<0>;

pub type NonPositive = Not<Positive>;

pub type Negative = LT<0>;

pub type NonNegative = Not<Negative>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_greater_than() {
        type Test = Refinement<i64, GreaterThan<5>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_greater_than_equal() {
        type Test = Refinement<i32, GreaterThanEqual<5>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_less_than() {
        type Test = Refinement<i16, LessThan<5>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(6).is_err());
    }

    #[test]
    fn test_less_than_equal() {
        type Test = Refinement<i8, LessThanEqual<5>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_err());
    }

    #[test]
    fn test_open_interval() {
        type Test = Refinement<i8, OpenInterval<5, 10>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(9).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(10).is_err());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_open_closed_interval() {
        type Test = Refinement<i16, OpenClosedInterval<5, 10>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(9).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_closed_open_interval() {
        type Test = Refinement<i32, ClosedOpenInterval<5, 10>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_err());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_closed_interval() {
        type Test = Refinement<i64, ClosedInterval<5, 10>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_equals() {
        type Test = Refinement<i16, Equals<5>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_err());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_zero() {
        type Test = Refinement<i8, Zero>;
        assert!(Test::refine(0).is_ok());
        assert!(Test::refine(1).is_err());
        assert!(Test::refine(-1).is_err());
    }

    #[test]
    fn test_non_zero() {
        type Test = Refinement<i16, NonZero>;
        assert!(Test::refine(1).is_ok());
        assert!(Test::refine(-1).is_ok());
        assert!(Test::refine(0).is_err());
    }

    #[test]
    fn test_positive() {
        type Test = Refinement<i32, Positive>;
        assert!(Test::refine(1).is_ok());
        assert!(Test::refine(-1).is_err());
        assert!(Test::refine(0).is_err());
    }

    #[test]
    fn test_non_positive() {
        type Test = Refinement<i64, NonPositive>;
        assert!(Test::refine(-1).is_ok());
        assert!(Test::refine(1).is_err());
        assert!(Test::refine(0).is_ok());
    }

    #[test]
    fn test_negative() {
        type Test = Refinement<i64, Negative>;
        assert!(Test::refine(-1).is_ok());
        assert!(Test::refine(1).is_err());
        assert!(Test::refine(0).is_err());
    }

    #[test]
    fn test_non_negative() {
        type Test = Refinement<isize, NonNegative>;
        assert!(Test::refine(1).is_ok());
        assert!(Test::refine(0).is_ok());
        assert!(Test::refine(-1).is_err());
    }

    #[test]
    fn test_modulo() {
        type Test = Refinement<isize, Modulo<4, 2>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_divisible() {
        type Test = Refinement<isize, Divisible<4>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(-4).is_ok());
        assert!(Test::refine(5).is_err());
    }

    #[test]
    fn test_even() {
        type Test = Refinement<isize, Even>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(-4).is_ok());
        assert!(Test::refine(0).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(-5).is_err());
    }

    #[test]
    fn test_odd() {
        type Test = Refinement<isize, Odd>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(-3).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(0).is_err());
        assert!(Test::refine(-2).is_err());
    }
}
