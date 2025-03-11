use core::{marker::PhantomData, ops::Add};

use crate::{boundable::*, Predicate, Refinement};

use super::*;

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::LessThan<A>>
where
    Refinement<Type, unsigned::LessThan<{ A + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThan<{ A + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::LessThanEqual<A>>
where
    Refinement<Type, unsigned::LessThanEqual<{ A + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThanEqual<{ A + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThan<A>>
where
    Refinement<Type, unsigned::GreaterThan<{ A + B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThan<{ A + B::UMIN }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThanEqual<A>>
where
    Refinement<Type, unsigned::GreaterThanEqual<{ A + B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThanEqual<{ A + B::UMIN }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::OpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::OpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::ClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::OpenClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedOpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, unsigned::ClosedOpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod unsigned_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_lt_add_lt() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let c: Refinement<u8, unsigned::LessThan<19>> = a + b;
        assert_eq!(*c, 18);
    }

    #[test]
    fn test_lt_add_lte() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(10).unwrap();
        let c: Refinement<u8, unsigned::LessThan<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_lt_add_eq() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::Equals<10>>::refine(10).unwrap();
        let c: Refinement<u8, unsigned::LessThan<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_gt_add_gt() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<21>> = a + b;
        assert_eq!(*c, 23);
    }

    #[test]
    fn test_gt_add_gte() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(10).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<20>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_lte_add_lt() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<10>>::refine(9).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<19>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_lte_add_lte() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(9).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_gte_add_gte() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<20>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_gte_add_gt() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<21>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_open_closed_interval_add() {
        let a = Refinement::<u8, unsigned::OpenClosedInterval<5, 10>>::refine(7).unwrap();
        let b = Refinement::<u8, unsigned::OpenClosedInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::OpenClosedInterval<16, 25>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_closed_open_interval_add() {
        let a = Refinement::<u8, unsigned::ClosedOpenInterval<5, 10>>::refine(8).unwrap();
        let b = Refinement::<u8, unsigned::ClosedOpenInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<u8, unsigned::ClosedOpenInterval<15, 24>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_open_interval_add() {
        let a = Refinement::<u8, unsigned::OpenInterval<5, 10>>::refine(7).unwrap();
        let b = Refinement::<u8, unsigned::OpenInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::OpenInterval<16, 24>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_closed_interval_add() {
        let a = Refinement::<u8, unsigned::ClosedInterval<5, 10>>::refine(8).unwrap();
        let b = Refinement::<u8, unsigned::ClosedInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<u8, unsigned::ClosedInterval<15, 25>> = a + b;
        assert_eq!(*c, 20);
    }
}

impl<
        const A: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::LessThan<A>>
where
    Refinement<Type, signed::LessThan<{ A + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, signed::LessThan<{ A + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::LessThanEqual<A>>
where
    Refinement<Type, signed::LessThanEqual<{ A + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, signed::LessThanEqual<{ A + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMin<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::GreaterThan<A>>
where
    Refinement<Type, signed::GreaterThan<{ A + B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, signed::GreaterThan<{ A + B::UMIN }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMin<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::GreaterThanEqual<A>>
where
    Refinement<Type, signed::GreaterThanEqual<{ A + B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, signed::GreaterThanEqual<{ A + B::UMIN }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::OpenInterval<MIN, MAX>>
where
    Refinement<Type, signed::OpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, signed::OpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, signed::ClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output = Refinement<Type, signed::ClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, signed::OpenClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, signed::OpenClosedInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Add<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Add<Refinement<Type, B>> for Refinement<Type, signed::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, signed::ClosedOpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>: Sized,
{
    type Output =
        Refinement<Type, signed::ClosedOpenInterval<{ MIN + B::UMIN }, { MAX + B::UMAX }>>;

    fn add(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod signed_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_lt_add_lt() {
        let a = Refinement::<i8, signed::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<i8, signed::LessThan<10>>::refine(9).unwrap();
        let c: Refinement<i8, signed::LessThan<19>> = a + b;
        assert_eq!(*c, 18);
    }

    #[test]
    fn test_lt_add_lte() {
        let a = Refinement::<i8, signed::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<i8, signed::LessThanEqual<10>>::refine(10).unwrap();
        let c: Refinement<i8, signed::LessThan<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_lt_add_eq() {
        let a = Refinement::<i8, signed::LessThan<10>>::refine(9).unwrap();
        let b = Refinement::<i8, signed::Equals<10>>::refine(10).unwrap();
        let c: Refinement<i8, signed::LessThan<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_gt_add_gt() {
        let a = Refinement::<i8, signed::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<i8, signed::GreaterThan<10>>::refine(12).unwrap();
        let c: Refinement<i8, signed::GreaterThan<21>> = a + b;
        assert_eq!(*c, 23);
    }

    #[test]
    fn test_gt_add_gte() {
        let a = Refinement::<i8, signed::GreaterThan<10>>::refine(11).unwrap();
        let b = Refinement::<i8, signed::GreaterThanEqual<10>>::refine(10).unwrap();
        let c: Refinement<i8, signed::GreaterThan<20>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_lte_add_lt() {
        let a = Refinement::<i8, signed::LessThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<i8, signed::LessThan<10>>::refine(9).unwrap();
        let c: Refinement<i8, signed::LessThanEqual<19>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_lte_add_lte() {
        let a = Refinement::<i8, signed::LessThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<i8, signed::LessThanEqual<10>>::refine(9).unwrap();
        let c: Refinement<i8, signed::LessThanEqual<20>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_gte_add_gte() {
        let a = Refinement::<i8, signed::GreaterThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<i8, signed::GreaterThanEqual<10>>::refine(11).unwrap();
        let c: Refinement<i8, signed::GreaterThanEqual<20>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_gte_add_gt() {
        let a = Refinement::<i8, signed::GreaterThanEqual<10>>::refine(10).unwrap();
        let b = Refinement::<i8, signed::GreaterThan<10>>::refine(11).unwrap();
        let c: Refinement<i8, signed::GreaterThanEqual<21>> = a + b;
        assert_eq!(*c, 21);
    }

    #[test]
    fn test_open_closed_interval_add() {
        let a = Refinement::<i8, signed::OpenClosedInterval<5, 10>>::refine(7).unwrap();
        let b = Refinement::<i8, signed::OpenClosedInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<i8, signed::OpenClosedInterval<16, 25>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_closed_open_interval_add() {
        let a = Refinement::<i8, signed::ClosedOpenInterval<5, 10>>::refine(8).unwrap();
        let b = Refinement::<i8, signed::ClosedOpenInterval<10, 15>>::refine(11).unwrap();
        let c: Refinement<i8, signed::ClosedOpenInterval<15, 24>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_open_interval_add() {
        let a = Refinement::<i8, signed::OpenInterval<5, 10>>::refine(7).unwrap();
        let b = Refinement::<i8, signed::OpenInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<i8, signed::OpenInterval<16, 24>> = a + b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_closed_interval_add() {
        let a = Refinement::<i8, signed::ClosedInterval<5, 10>>::refine(8).unwrap();
        let b = Refinement::<i8, signed::ClosedInterval<10, 15>>::refine(12).unwrap();
        let c: Refinement<i8, signed::ClosedInterval<15, 25>> = a + b;
        assert_eq!(*c, 20);
    }
}
