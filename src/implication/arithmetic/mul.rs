use std::{marker::PhantomData, ops::Mul};

use crate::{boundable::*, Predicate, Refinement};

use super::*;

impl<
        const A: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::LessThan<A>>
where
    Refinement<Type, unsigned::LessThan<{ (A - 1) * B::UMAX + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThan<{ (A - 1) * B::UMAX + 1 }>>;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::LessThanEqual<A>>
where
    Refinement<Type, unsigned::LessThanEqual<{ A * B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThanEqual<{ A * B::UMAX }>>;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMin<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThan<A>>
where
    Refinement<Type, unsigned::GreaterThan<{ (A + 1) * B::UMIN - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThan<{ (A + 1) * B::UMIN - 1 }>>;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMin<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThanEqual<A>>
where
    Refinement<Type, unsigned::GreaterThanEqual<{ A * B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThanEqual<{ A * B::UMIN }>>;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::OpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        unsigned::OpenInterval<{ (MIN + 1) * B::UMIN - 1 }, { (MAX - 1) * B::UMAX + 1 }>,
    >: Sized,
{
    type Output = Refinement<
        Type,
        unsigned::OpenInterval<{ (MIN + 1) * B::UMIN - 1 }, { (MAX - 1) * B::UMAX + 1 }>,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedInterval<{ MIN * B::UMIN }, { MAX * B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::ClosedInterval<{ MIN * B::UMIN }, { MAX * B::UMAX }>>;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenClosedInterval<{ (MIN + 1) * B::UMIN - 1 }, { MAX * B::UMAX }>>:
        Sized,
{
    type Output = Refinement<
        Type,
        unsigned::OpenClosedInterval<{ (MIN + 1) * B::UMIN - 1 }, { MAX * B::UMAX }>,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: unsigned::UnsignedBoundable + Mul<Output = Type>,
        B: UnsignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedOpenInterval<{ MIN * B::UMIN }, { (MAX - 1) * B::UMAX + 1 }>>:
        Sized,
{
    type Output = Refinement<
        Type,
        unsigned::ClosedOpenInterval<{ MIN * B::UMIN }, { (MAX - 1) * B::UMAX + 1 }>,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod unsigned_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_lt_mul_lt() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let c: Refinement<u8, unsigned::LessThan<82>> = a * b;
        assert_eq!(*c, 81);
    }

    #[test]
    fn test_lte_mul_lte() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(6).unwrap();
        let b = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<100>> = a * b;
        assert_eq!(*c, 18);
    }

    #[test]
    fn test_lte_mul_lt() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(6).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<11>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<100>> = a * b;
        assert_eq!(*c, 18);
    }

    #[test]
    fn test_gt_mul_gt() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<120>> = a * b;
        assert_eq!(*c, 121);
    }

    #[test]
    fn test_gt_mul_gte() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<3>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<32>> = a * b;
        assert_eq!(*c, 33);
    }

    #[test]
    fn test_gte_mul_gte() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(12).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<3>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<30>> = a * b;
        assert_eq!(*c, 36);
    }

    #[test]
    fn test_gte_mul_gt() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<12>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<3>>::refine(4).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<48>> = a * b;
        assert_eq!(*c, 60);
    }

    #[test]
    fn test_open_closed_interval_mul() {
        let a = Refinement::<u8, unsigned::OpenClosedInterval<15, 20>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::OpenClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<u8, unsigned::OpenClosedInterval<63, 120>> = a * b;
        assert_eq!(*c, 108);
    }

    #[test]
    fn test_closed_open_interval_mul() {
        let a = Refinement::<u16, unsigned::ClosedOpenInterval<50, 100>>::refine(99).unwrap();
        let b = Refinement::<u16, unsigned::ClosedOpenInterval<5, 10>>::refine(9).unwrap();
        let c: Refinement<u16, unsigned::ClosedOpenInterval<250, 892>> = a * b;
        assert_eq!(*c, 891);
    }

    #[test]
    fn test_open_interval_mul() {
        let a = Refinement::<u8, unsigned::OpenInterval<15, 30>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::OpenInterval<3, 6>>::refine(5).unwrap();
        let c: Refinement<u8, unsigned::OpenInterval<63, 146>> = a * b;
        assert_eq!(*c, 90);
    }

    #[test]
    fn test_closed_interval_mul() {
        let a = Refinement::<u8, unsigned::ClosedInterval<15, 50>>::refine(30).unwrap();
        let b = Refinement::<u8, unsigned::ClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<u8, unsigned::ClosedInterval<45, 300>> = a * b;
        assert_eq!(*c, 180);
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: signed::SignedBoundable + Mul<Output = Type>,
        B: SignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, signed::OpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::OpenInterval<
            { min_mul(MIN + 1, MAX - 1, B::UMIN, B::UMAX) - 1 },
            { max_mul(MIN + 1, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::OpenInterval<
            { min_mul(MIN + 1, MAX - 1, B::UMIN, B::UMAX) - 1 },
            { max_mul(MIN + 1, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: signed::SignedBoundable + Mul<Output = Type>,
        B: SignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, signed::ClosedInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::ClosedInterval<
            { min_mul(MIN, MAX, B::UMIN, B::UMAX) },
            { max_mul(MIN, MAX, B::UMIN, B::UMAX) },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::ClosedInterval<
            { min_mul(MIN, MAX, B::UMIN, B::UMAX) },
            { max_mul(MIN, MAX, B::UMIN, B::UMAX) },
        >,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: signed::SignedBoundable + Mul<Output = Type>,
        B: SignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, signed::OpenClosedInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::OpenClosedInterval<
            { min_mul(MIN + 1, MAX, B::UMIN, B::UMAX) - 1 },
            { max_mul(MIN + 1, MAX, B::UMIN, B::UMAX) },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::OpenClosedInterval<
            { min_mul(MIN + 1, MAX, B::UMIN, B::UMAX) - 1 },
            { max_mul(MIN + 1, MAX, B::UMIN, B::UMAX) },
        >,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: signed::SignedBoundable + Mul<Output = Type>,
        B: SignedMinMax<Type> + Predicate<Type>,
    > Mul<Refinement<Type, B>> for Refinement<Type, signed::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::ClosedOpenInterval<
            { min_mul(MIN, MAX - 1, B::UMIN, B::UMAX) },
            { max_mul(MIN, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::ClosedOpenInterval<
            { min_mul(MIN, MAX - 1, B::UMIN, B::UMAX) },
            { max_mul(MIN, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >;

    fn mul(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod signed_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_open_closed_interval_mul() {
        let a = Refinement::<i8, signed::OpenClosedInterval<15, 20>>::refine(20).unwrap();
        let b = Refinement::<i8, signed::OpenClosedInterval<-6, -3>>::refine(-5).unwrap();
        let c: Refinement<i8, signed::OpenClosedInterval<-101, -48>> = a * b;
        assert_eq!(*c, -100);
    }

    #[test]
    fn test_closed_open_interval_mul() {
        let a = Refinement::<i16, signed::ClosedOpenInterval<-50, 100>>::refine(99).unwrap();
        let b = Refinement::<i16, signed::ClosedOpenInterval<5, 10>>::refine(9).unwrap();
        let c: Refinement<i16, signed::ClosedOpenInterval<-450, 892>> = a * b;
        assert_eq!(*c, 891);
    }

    #[test]
    fn test_open_interval_mul() {
        let a = Refinement::<i8, signed::OpenInterval<15, 30>>::refine(16).unwrap();
        let b = Refinement::<i8, signed::OpenInterval<-13, -6>>::refine(-7).unwrap();
        let c: Refinement<i8, signed::OpenInterval<-349, -111>> = a * b;
        assert_eq!(*c, -112);
    }

    #[test]
    fn test_closed_interval_mul() {
        let a = Refinement::<i16, signed::ClosedInterval<15, 50>>::refine(30).unwrap();
        let b = Refinement::<i16, signed::ClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<i16, signed::ClosedInterval<45, 300>> = a * b;
        assert_eq!(*c, 180);
    }
}
