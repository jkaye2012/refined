use core::{marker::PhantomData, ops::Div};

use crate::{boundable::*, Predicate, Refinement};

use super::*;

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::LessThan<A>>
where
    Refinement<Type, unsigned::LessThan<A>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThan<A>>;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::LessThanEqual<A>>
where
    Refinement<Type, unsigned::LessThanEqual<A>>: Sized,
{
    type Output = Refinement<Type, unsigned::LessThanEqual<A>>;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThan<A>>
where
    Refinement<Type, unsigned::GreaterThan<{ A / B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThan<{ A / B::UMIN }>>;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMin<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::GreaterThanEqual<A>>
where
    Refinement<Type, unsigned::GreaterThanEqual<{ A / B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GreaterThanEqual<{ A / B::UMIN }>>;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::OpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        unsigned::OpenInterval<{ (MIN + 1) / B::UMAX - 1 }, { (MAX - 1) / B::UMIN + 1 }>,
    >: Sized,
{
    type Output = Refinement<
        Type,
        unsigned::OpenInterval<{ (MIN + 1) / B::UMAX - 1 }, { (MAX - 1) / B::UMIN + 1 }>,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedInterval<{ MIN / B::UMAX }, { MAX / B::UMIN }>>: Sized,
{
    type Output = Refinement<Type, unsigned::ClosedInterval<{ MIN / B::UMAX }, { MAX / B::UMIN }>>;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::OpenClosedInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::OpenClosedInterval<{ (MIN + 1) / B::UMAX - 1 }, { MAX / B::UMIN }>>:
        Sized,
{
    type Output = Refinement<
        Type,
        unsigned::OpenClosedInterval<{ (MIN + 1) / B::UMAX - 1 }, { MAX / B::UMIN }>,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: usize,
        const MAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
        B: Clone + UnsignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, unsigned::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<Type, unsigned::ClosedOpenInterval<{ MIN / B::UMAX }, { (MAX - 1) / B::UMIN + 1 }>>:
        Sized,
{
    type Output = Refinement<
        Type,
        unsigned::ClosedOpenInterval<{ MIN / B::UMAX }, { (MAX - 1) / B::UMIN + 1 }>,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod unsigned_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_lt_div_lt() {
        let a = Refinement::<u8, unsigned::LessThan<10>>::refine(6).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<10>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThan<10>> = a / b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_lte_div_lte() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(6).unwrap();
        let b = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<10>> = a / b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_lte_div_lt() {
        let a = Refinement::<u8, unsigned::LessThanEqual<10>>::refine(6).unwrap();
        let b = Refinement::<u8, unsigned::LessThan<11>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::LessThanEqual<10>> = a / b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_gt_div_gt() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<10>>::refine(13).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<0>> = a / b;
        assert_eq!(*c, 1);
    }

    #[test]
    fn test_gt_div_gte() {
        let a = Refinement::<u8, unsigned::GreaterThan<10>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<3>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::GreaterThan<3>> = a / b;
        assert_eq!(*c, 5);
    }

    #[test]
    fn test_gte_div_gte() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<10>>::refine(12).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThanEqual<3>>::refine(3).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<3>> = a / b;
        assert_eq!(*c, 4);
    }

    #[test]
    fn test_gte_div_gt() {
        let a = Refinement::<u8, unsigned::GreaterThanEqual<12>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GreaterThan<3>>::refine(4).unwrap();
        let c: Refinement<u8, unsigned::GreaterThanEqual<3>> = a / b;
        assert_eq!(*c, 3);
    }

    #[test]
    fn test_open_closed_interval_div() {
        let a = Refinement::<u8, unsigned::OpenClosedInterval<15, 20>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::OpenClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<u8, unsigned::OpenClosedInterval<1, 5>> = a / b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_closed_open_interval_div() {
        let a = Refinement::<u8, unsigned::ClosedOpenInterval<50, 100>>::refine(99).unwrap();
        let b = Refinement::<u8, unsigned::ClosedOpenInterval<5, 10>>::refine(5).unwrap();
        let c: Refinement<u8, unsigned::ClosedOpenInterval<5, 20>> = a / b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_open_interval_div() {
        let a = Refinement::<u8, unsigned::OpenInterval<15, 30>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::OpenInterval<3, 6>>::refine(5).unwrap();
        let c: Refinement<u8, unsigned::OpenInterval<2, 8>> = a / b;
        assert_eq!(*c, 3);
    }

    #[test]
    fn test_closed_interval_div() {
        let a = Refinement::<u8, unsigned::ClosedInterval<15, 50>>::refine(30).unwrap();
        let b = Refinement::<u8, unsigned::ClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<u8, unsigned::ClosedInterval<2, 16>> = a / b;
        assert_eq!(*c, 5);
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Div<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, signed::OpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::OpenInterval<
            { min_div(MIN + 1, MAX - 1, B::UMIN, B::UMAX) - 1 },
            { max_div(MIN + 1, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::OpenInterval<
            { min_div(MIN + 1, MAX - 1, B::UMIN, B::UMAX) - 1 },
            { max_div(MIN + 1, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Div<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, signed::ClosedInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::ClosedInterval<
            { min_div(MIN, MAX, B::UMIN, B::UMAX) },
            { max_div(MIN, MAX, B::UMIN, B::UMAX) },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::ClosedInterval<
            { min_div(MIN, MAX, B::UMIN, B::UMAX) },
            { max_div(MIN, MAX, B::UMIN, B::UMAX) },
        >,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Div<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, signed::OpenClosedInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::OpenClosedInterval<
            { min_div(MIN + 1, MAX, B::UMIN, B::UMAX) - 1 },
            { max_div(MIN + 1, MAX, B::UMIN, B::UMAX) },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::OpenClosedInterval<
            { min_div(MIN + 1, MAX, B::UMIN, B::UMAX) - 1 },
            { max_div(MIN + 1, MAX, B::UMIN, B::UMAX) },
        >,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const MIN: isize,
        const MAX: isize,
        Type: Clone + signed::SignedBoundable + Div<Output = Type>,
        B: Clone + SignedMinMax<Type> + Predicate<Type>,
    > Div<Refinement<Type, B>> for Refinement<Type, signed::ClosedOpenInterval<MIN, MAX>>
where
    Refinement<
        Type,
        signed::ClosedOpenInterval<
            { min_div(MIN, MAX - 1, B::UMIN, B::UMAX) },
            { max_div(MIN, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >: Sized,
{
    type Output = Refinement<
        Type,
        signed::ClosedOpenInterval<
            { min_div(MIN, MAX - 1, B::UMIN, B::UMAX) },
            { max_div(MIN, MAX - 1, B::UMIN, B::UMAX) + 1 },
        >,
    >;

    fn div(self, rhs: Refinement<Type, B>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod signed_tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_open_closed_interval_div() {
        let a = Refinement::<i8, signed::OpenClosedInterval<15, 20>>::refine(16).unwrap();
        let b = Refinement::<i8, signed::OpenClosedInterval<3, 6>>::refine(6).unwrap();
        let c: Refinement<i8, signed::OpenClosedInterval<1, 5>> = a / b;
        assert_eq!(*c, 2);
    }

    #[test]
    fn test_closed_open_interval_div() {
        let a = Refinement::<i8, signed::ClosedOpenInterval<50, 100>>::refine(99).unwrap();
        let b = Refinement::<i8, signed::ClosedOpenInterval<5, 10>>::refine(5).unwrap();
        let c: Refinement<i8, signed::ClosedOpenInterval<5, 20>> = a / b;
        assert_eq!(*c, 19);
    }

    #[test]
    fn test_open_interval_div() {
        let a = Refinement::<i8, signed::OpenInterval<-30, -15>>::refine(-16).unwrap();
        let b = Refinement::<i8, signed::OpenInterval<-10, -5>>::refine(-9).unwrap();
        let c: Refinement<i8, signed::OpenInterval<0, 5>> = a / b;
        assert_eq!(*c, 1);
    }

    #[test]
    fn test_closed_interval_div() {
        let a = Refinement::<i8, signed::ClosedInterval<-15, 50>>::refine(50).unwrap();
        let b = Refinement::<i8, signed::ClosedInterval<-6, -3>>::refine(-3).unwrap();
        let c: Refinement<i8, signed::ClosedInterval<-16, 5>> = a / b;
        assert_eq!(*c, -16);
    }
}
