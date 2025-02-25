use std::{marker::PhantomData, ops::Sub};

use crate::{boundable::*, Predicate, Refinement};

use super::{UnsignedMin, UnsignedMinMax};

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThan<A>>
where
    Refinement<Type, unsigned::GreaterThan<{ A + 1 - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThan<{ A + 1 - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThanEqual<A>>
where
    Refinement<Type, unsigned::GreaterThanEqual<{ A - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThanEqual<{ A - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::OpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::OpenInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::ClosedInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenClosedInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::OpenClosedInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedOpenInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::ClosedOpenInterval<{ MIN - B::UMIN }, { MAX - B::UMAX }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod unsigned_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_gt_sub_gt() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<0>> = a - b;
        assert_eq!(*c, 0);
    }

    #[test]
    fn test_gt_sub_gte() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<11>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<0>> = a - b;
        assert_eq!(*c, 0);
    }

    #[test]
    fn test_gte_sub_gte() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(10).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<0>> = a - b;
        assert_eq!(*c, 0);
    }

    #[test]
    fn test_gte_sub_gt() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<11>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<0>> = a - b;
        assert_eq!(*c, 0);
    }

    #[test]
    fn test_open_closed_interval_sub() {
        let a = Refinement::<u8, unsigned::OpenClosedInterval<11, 20>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::OpenClosedInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::OpenClosedInterval<0, 5>> = a - b;
        assert_eq!(*c, 3);
    }

    #[test]
    fn test_closed_open_interval_sub() {
        let a = Refinement::<u8, unsigned::ClosedOpenInterval<50, 100>>::refine(58).unwrap();
        let b = Refinement::<u8, unsigned::ClosedOpenInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::ClosedOpenInterval<40, 86>> = a - b;
        assert_eq!(*c, 47);
    }

    #[test]
    fn test_open_interval_sub() {
        let a = Refinement::<u8, unsigned::OpenInterval<15, 30>>::refine(17).unwrap();
        let b = Refinement::<u8, unsigned::OpenInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::OpenInterval<4, 16>> = a - b;
        assert_eq!(*c, 5);
    }

    #[test]
    fn test_closed_interval_sub() {
        let a = Refinement::<u8, unsigned::ClosedInterval<15, 50>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::ClosedInterval<5, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::ClosedInterval<10, 35>> = a - b;
        assert_eq!(*c, 6);
    }
}
