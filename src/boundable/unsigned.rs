//! Boundable refinement via unsigned values.
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{boolean::*, Predicate};

/// Types that can be reduced to an unsigned size so that they can be bounded.
pub trait UnsignedBoundable {
    fn bounding_value(&self) -> usize;
}

impl UnsignedBoundable for u8 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl UnsignedBoundable for std::num::Saturating<u8> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl UnsignedBoundable for std::num::NonZeroU8 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl UnsignedBoundable for u16 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl UnsignedBoundable for std::num::Saturating<u16> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl UnsignedBoundable for std::num::NonZeroU16 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl UnsignedBoundable for u32 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl UnsignedBoundable for std::num::Saturating<u32> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl UnsignedBoundable for std::num::NonZeroU32 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl UnsignedBoundable for usize {
    fn bounding_value(&self) -> usize {
        *self
    }
}

impl UnsignedBoundable for std::num::Saturating<usize> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl UnsignedBoundable for std::num::NonZeroUsize {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl UnsignedBoundable for u64 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl UnsignedBoundable for std::num::Saturating<u64> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl UnsignedBoundable for std::num::NonZeroU64 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

/// Creates an [UnsignedBoundable] implementation for a struct that has a `len` method.
///
/// # Example
///
/// ```
/// use refined::{unsigned_boundable_via_len, UnsignedBoundable};
/// use std::collections::HashMap;
///
/// struct Wrapper<K, V> { inner: HashMap<K, V> };
///
/// impl<K, V> Wrapper<K, V> {
///   pub fn len(&self) -> usize { self.inner.len() }
/// }
///
/// unsigned_boundable_via_len!(Wrapper<K, V>);
/// // `Wrapper<K, V> now implements `UnsignedBoundable`
/// ```
#[macro_export]
macro_rules! unsigned_boundable_via_len {
    ($t:ident $(<$($ts:ident),+>)?) => {
        impl $(<$($ts),+>)? UnsignedBoundable for $t $(<$($ts),+>)? {
            fn bounding_value(&self) -> usize {
                self.len()
            }
        }
    };
}

unsigned_boundable_via_len!(String);
unsigned_boundable_via_len!(BinaryHeap<T>);
unsigned_boundable_via_len!(BTreeMap<K, V>);
unsigned_boundable_via_len!(BTreeSet<T>);
unsigned_boundable_via_len!(HashMap<K, V>);
unsigned_boundable_via_len!(HashSet<T>);
unsigned_boundable_via_len!(LinkedList<T>);
unsigned_boundable_via_len!(Vec<T>);
unsigned_boundable_via_len!(VecDeque<T>);

impl<T> UnsignedBoundable for [T] {
    fn bounding_value(&self) -> usize {
        self.len()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GreaterThan<const MIN: usize>;

pub type GT<const MIN: usize> = GreaterThan<MIN>;

impl<T: UnsignedBoundable, const MIN: usize> Predicate<T> for GreaterThan<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() > MIN
    }

    fn error() -> String {
        format!("must be greater than {}", MIN)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct GreaterThanEqual<const MIN: usize>;

pub type GTE<const MIN: usize> = GreaterThanEqual<MIN>;

impl<T: UnsignedBoundable, const MIN: usize> Predicate<T> for GreaterThanEqual<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() >= MIN
    }

    fn error() -> String {
        format!("must be greater than or equal to {}", MIN)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LessThan<const MAX: usize>;

pub type LT<const MAX: usize> = LessThan<MAX>;

impl<T: UnsignedBoundable, const MAX: usize> Predicate<T> for LessThan<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() < MAX
    }

    fn error() -> String {
        format!("must be less than {}", MAX)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LessThanEqual<const MAX: usize>;

pub type LTE<const MAX: usize> = LessThanEqual<MAX>;

impl<T: UnsignedBoundable, const MAX: usize> Predicate<T> for LessThanEqual<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() <= MAX
    }

    fn error() -> String {
        format!("must be less than or equal to {}", MAX)
    }
}

pub type OpenInterval<const MIN: usize, const MAX: usize> = And<GT<MIN>, LT<MAX>>;

pub type OpenClosedInterval<const MIN: usize, const MAX: usize> = And<GT<MIN>, LTE<MAX>>;

pub type ClosedOpenInterval<const MIN: usize, const MAX: usize> = And<GTE<MIN>, LT<MAX>>;

pub type ClosedInterval<const MIN: usize, const MAX: usize> = And<GTE<MIN>, LTE<MAX>>;
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Modulo<const DIV: usize, const MOD: usize>;

impl<T: UnsignedBoundable, const DIV: usize, const MOD: usize> Predicate<T> for Modulo<DIV, MOD> {
    fn test(value: &T) -> bool {
        value.bounding_value() % DIV == MOD
    }

    fn error() -> String {
        format!("must be divisible by {} with a remainder of {}", DIV, MOD)
    }
}

pub type Divisible<const DIV: usize> = Modulo<DIV, 0>;

pub type Even = Modulo<2, 0>;

pub type Odd = Not<Even>;
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Equals<const VAL: usize>;

impl<T: UnsignedBoundable, const VAL: usize> Predicate<T> for Equals<VAL> {
    fn test(value: &T) -> bool {
        value.bounding_value() == VAL
    }

    fn error() -> String {
        format!("must be equal to {}", VAL)
    }
}

pub type Zero = Equals<0>;

pub type NonZero = Not<Zero>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_greater_than() {
        type Test = Refinement<u64, GreaterThan<5>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_greater_than_equal() {
        type Test = Refinement<u32, GreaterThanEqual<5>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_less_than() {
        type Test = Refinement<u16, LessThan<5>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(6).is_err());
    }

    #[test]
    fn test_less_than_equal() {
        type Test = Refinement<u8, LessThanEqual<5>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_err());
    }

    #[test]
    fn test_open_interval() {
        type Test = Refinement<u8, OpenInterval<5, 10>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(9).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(10).is_err());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_open_closed_interval() {
        type Test = Refinement<u16, OpenClosedInterval<5, 10>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(9).is_ok());
        assert!(Test::refine(5).is_err());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_closed_open_interval() {
        type Test = Refinement<u32, ClosedOpenInterval<5, 10>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_err());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_closed_interval() {
        type Test = Refinement<u64, ClosedInterval<5, 10>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(11).is_err());
    }

    #[test]
    fn test_equals() {
        type Test = Refinement<u16, Equals<5>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_err());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_zero() {
        type Test = Refinement<u8, Zero>;
        assert!(Test::refine(0).is_ok());
        assert!(Test::refine(1).is_err());
    }

    #[test]
    fn test_non_zero() {
        type Test = Refinement<u16, NonZero>;
        assert!(Test::refine(1).is_ok());
        assert!(Test::refine(0).is_err());
    }

    #[test]
    fn test_modulo() {
        type Test = Refinement<usize, Modulo<4, 2>>;
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_ok());
        assert!(Test::refine(4).is_err());
    }

    #[test]
    fn test_divisible() {
        type Test = Refinement<usize, Divisible<4>>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(5).is_err());
    }

    #[test]
    fn test_even() {
        type Test = Refinement<usize, Even>;
        assert!(Test::refine(4).is_ok());
        assert!(Test::refine(0).is_ok());
        assert!(Test::refine(5).is_err());
    }

    #[test]
    fn test_odd() {
        type Test = Refinement<usize, Odd>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(4).is_err());
        assert!(Test::refine(0).is_err());
    }
}
