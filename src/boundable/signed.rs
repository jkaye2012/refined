use crate::boolean::*;
use crate::Predicate;

pub trait Boundable {
    fn bounding_value(&self) -> isize;
}

impl Boundable for i8 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl Boundable for std::num::Saturating<i8> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl Boundable for std::num::NonZeroI8 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl Boundable for i16 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl Boundable for std::num::Saturating<i16> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl Boundable for std::num::NonZeroI16 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl Boundable for i32 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

impl Boundable for std::num::Saturating<i32> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl Boundable for std::num::NonZeroI32 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

impl Boundable for isize {
    fn bounding_value(&self) -> isize {
        *self
    }
}

impl Boundable for std::num::Saturating<isize> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

impl Boundable for std::num::NonZeroIsize {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for i64 {
    fn bounding_value(&self) -> isize {
        *self as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for std::num::Saturating<i64> {
    fn bounding_value(&self) -> isize {
        self.0 as isize
    }
}

#[cfg(target_pointer_width = "64")]
impl Boundable for std::num::NonZeroI64 {
    fn bounding_value(&self) -> isize {
        self.get() as isize
    }
}

pub struct GreaterThan<const MIN: isize>;

pub type GT<const MIN: isize> = GreaterThan<MIN>;

impl<T: Boundable, const MIN: isize> Predicate<T> for GreaterThan<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() > MIN
    }
}

pub struct GreaterThanEqual<const MIN: isize>;

pub type GTE<const MIN: isize> = GreaterThanEqual<MIN>;

impl<T: Boundable, const MIN: isize> Predicate<T> for GreaterThanEqual<MIN> {
    fn test(value: &T) -> bool {
        value.bounding_value() >= MIN
    }
}

pub struct LessThan<const MAX: isize>;

pub type LT<const MAX: isize> = LessThan<MAX>;

impl<T: Boundable, const MAX: isize> Predicate<T> for LessThan<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() < MAX
    }
}

pub struct LessThanEqual<const MAX: isize>;

pub type LTE<const MAX: isize> = LessThanEqual<MAX>;

impl<T: Boundable, const MAX: isize> Predicate<T> for LessThanEqual<MAX> {
    fn test(value: &T) -> bool {
        value.bounding_value() <= MAX
    }
}

pub type Between<const MIN: isize, const MAX: isize> = And<GTE<MIN>, LT<MAX>>;

// TODO: implement open/closed ranges instead

// TODO: Modulo, Divisible, Even, Odd

pub struct Equals<const VAL: isize>;

impl<T: Boundable, const VAL: isize> Predicate<T> for Equals<VAL> {
    fn test(value: &T) -> bool {
        value.bounding_value() == VAL
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
    fn test_between() {
        type Test = Refinement<i32, Between<5, 10>>;
        assert!(Test::refine(5).is_ok());
        assert!(Test::refine(6).is_ok());
        assert!(Test::refine(10).is_err());
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
}
