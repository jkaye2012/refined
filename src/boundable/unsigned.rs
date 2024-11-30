use crate::boolean::*;
use crate::Predicate;

pub trait Boundable {
    fn bounding_value(&self) -> usize;
}

impl Boundable for u8 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl Boundable for std::num::Saturating<u8> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl Boundable for std::num::NonZeroU8 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl Boundable for u16 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl Boundable for std::num::Saturating<u16> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl Boundable for std::num::NonZeroU16 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl Boundable for u32 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

impl Boundable for std::num::Saturating<u32> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl Boundable for std::num::NonZeroU32 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

impl Boundable for usize {
    fn bounding_value(&self) -> usize {
        *self
    }
}

impl Boundable for std::num::Saturating<usize> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

impl Boundable for std::num::NonZeroUsize {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for u64 {
    fn bounding_value(&self) -> usize {
        *self as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for std::num::Saturating<u64> {
    fn bounding_value(&self) -> usize {
        self.0 as usize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for std::num::NonZeroU64 {
    fn bounding_value(&self) -> usize {
        self.get() as usize
    }
}

macro_rules! boundable_via_len {
    ($t:ident $(<$($ts:ident),+>)?) => {
        impl $(<$($ts),+>)? Boundable for $t $(<$($ts),+>)? {
            fn bounding_value(&self) -> usize {
                self.len()
            }
        }
    };
}

boundable_via_len!(String);
boundable_via_len!(Vec<T>);

impl<T> Boundable for [T] {
    fn bounding_value(&self) -> usize {
        self.len()
    }
}
pub struct GreaterThan<const MIN: usize>;

pub type GT<const MIN: usize> = GreaterThan<MIN>;

impl<T: Boundable, const MIN: usize> Predicate<T> for GreaterThan<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() > MIN
    }
}

pub struct GreaterThanEqual<const MIN: usize>;

pub type GTE<const MIN: usize> = GreaterThanEqual<MIN>;

impl<T: Boundable, const MIN: usize> Predicate<T> for GreaterThanEqual<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() >= MIN
    }
}

pub struct LessThan<const MAX: usize>;

pub type LT<const MAX: usize> = LessThan<MAX>;

impl<T: Boundable, const MAX: usize> Predicate<T> for LessThan<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() < MAX
    }
}

pub struct LessThanEqual<const MAX: usize>;

pub type LTE<const MAX: usize> = LessThanEqual<MAX>;

impl<T: Boundable, const MAX: usize> Predicate<T> for LessThanEqual<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() <= MAX
    }
}

pub type OpenInterval<const MIN: usize, const MAX: usize> = And<GT<MIN>, LT<MAX>>;

pub type OpenClosedInterval<const MIN: usize, const MAX: usize> = And<GT<MIN>, LTE<MAX>>;

pub type ClosedOpenInterval<const MIN: usize, const MAX: usize> = And<GTE<MIN>, LT<MAX>>;

pub type ClosedInterval<const MIN: usize, const MAX: usize> = And<GTE<MIN>, LTE<MAX>>;

pub struct Modulo<const DIV: usize, const MOD: usize>;

impl<T: Boundable, const DIV: usize, const MOD: usize> Predicate<T> for Modulo<DIV, MOD> {
    fn test(value: &T) -> bool {
        value.bounding_value() % DIV == MOD
    }
}

pub type Divisible<const DIV: usize> = Modulo<DIV, 0>;

pub type Even = Modulo<2, 0>;

pub type Odd = Not<Even>;

pub struct Equals<const VAL: usize>;

impl<T: Boundable, const VAL: usize> Predicate<T> for Equals<VAL> {
    fn test(value: &T) -> bool {
        value.bounding_value() == VAL
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
