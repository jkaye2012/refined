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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // NEXT: error messages for tests? Probably makes more sense not to include message
    // testing in every individual test

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
}
