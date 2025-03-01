use std::{marker::PhantomData, ops::Sub};

use crate::{boundable::*, Assert, IsTrue, Predicate, Refinement};

use super::*;

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::LessThan<A>>
where
    Assert<{ A >= B::UMAX }>: IsTrue,
    Refinement<Type, unsigned::LessThan<A>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThan<A>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, unsigned::LessThanEqual<A>>
where
    Assert<{ A >= B::UMAX }>: IsTrue,
    Refinement<Type, unsigned::LessThanEqual<A>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThanEqual<A>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

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
    Refinement<Type, unsigned::OpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::OpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

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
    Refinement<Type, unsigned::ClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::ClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

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
    Refinement<Type, unsigned::OpenClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::OpenClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

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
    Refinement<Type, unsigned::ClosedOpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::ClosedOpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod unsigned_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_lt_sub_lt() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(5).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<10>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThan<10>> = a - b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_lte_sub_lte() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(5).unwrap();
        let b = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<10>> = a - b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_lte_sub_lt() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(5).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<11>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<10>> = a - b;
        assert_eq!(*c, 2);
    }

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
        let b = Refinement::<u8, unsigned::OpenClosedInterval<0, 5>>::refine(2).unwrap();
        let c: Refinement<u8, unsigned::OpenClosedInterval<6, 19>> = a - b;
        assert_eq!(*c, 13);
    }

    #[test]
    fn test_closed_open_interval_sub() {
        let a = Refinement::<u8, unsigned::ClosedOpenInterval<50, 100>>::refine(58).unwrap();
        let b = Refinement::<u8, unsigned::ClosedOpenInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::ClosedOpenInterval<36, 90>> = a - b;
        assert_eq!(*c, 47);
    }

    #[test]
    fn test_open_interval_sub() {
        let a = Refinement::<u8, unsigned::OpenInterval<15, 30>>::refine(29).unwrap();
        let b = Refinement::<u8, unsigned::OpenInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::OpenInterval<1, 19>> = a - b;
        assert_eq!(*c, 18);
    }

    #[test]
    fn test_closed_interval_sub() {
        let a = Refinement::<u8, unsigned::ClosedInterval<15, 50>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::ClosedInterval<5, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::ClosedInterval<0, 45>> = a - b;
        assert_eq!(*c, 6);
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Sub<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, signed::OpenInterval<MIN, MAX>>
where
    Refinement<Type, signed::OpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, signed::OpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Sub<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, signed::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, signed::ClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, signed::ClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Sub<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, signed::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, signed::OpenClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output =
        Refinement<Type, signed::OpenClosedInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Sub<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Sub<Refinement<Type, B>> for Refinement<Type, signed::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, signed::ClosedOpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>: Sized,
{
    type Output =
        Refinement<Type, signed::ClosedOpenInterval<{ MIN - B::UMAX }, { MAX - B::UMIN }>>;

    fn sub(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod signed_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_open_closed_interval_sub() {
        let a = Refinement::<i8, signed::OpenClosedInterval<-11, 20>>::refine(-10).unwrap();
        let b = Refinement::<i8, signed::OpenClosedInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<i8, signed::OpenClosedInterval<-26, 9>> = a - b;
        assert_eq!(*c, -21);
    }

    #[test]
    fn test_closed_open_interval_sub() {
        let a = Refinement::<i8, signed::ClosedOpenInterval<-50, 100>>::refine(99).unwrap();
        let b = Refinement::<i8, signed::ClosedOpenInterval<-15, -10>>::refine(-15).unwrap();
        let c: Refinement<i8, signed::ClosedOpenInterval<-39, 115>> = a - b;
        assert_eq!(*c, 114);
    }

    #[test]
    fn test_open_interval_sub() {
        let a = Refinement::<i8, signed::OpenInterval<-30, -15>>::refine(-29).unwrap();
        let b = Refinement::<i8, signed::OpenInterval<-15, -5>>::refine(-6).unwrap();
        let c: Refinement<i8, signed::OpenInterval<-24, -1>> = a - b;
        assert_eq!(*c, -23);
    }

    #[test]
    fn test_closed_interval_sub() {
        let a = Refinement::<i8, signed::ClosedInterval<15, 50>>::refine(18).unwrap();
        let b = Refinement::<i8, signed::ClosedInterval<5, 15>>::refine(12).unwrap();
        let c: Refinement<i8, signed::ClosedInterval<0, 45>> = a - b;
        assert_eq!(*c, 6);
    }
}
