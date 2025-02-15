use std::marker::PhantomData;
use std::ops::*;

use super::{Assert, IsTrue};
use crate::{boundable::*, Refinement};

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Refinement<Type, unsigned::LT<{ AMAX + BMAX - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ AMAX + BMAX - 1 }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Refinement<Type, unsigned::LT<{ AMAX + BMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ AMAX + BMAX }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Refinement<Type, unsigned::LT<{ (AMAX - 1) * (BMAX - 1) + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ (AMAX - 1) * (BMAX - 1) + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Refinement<Type, unsigned::LT<{ (AMAX - 1) * BMAX + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ (AMAX - 1) * BMAX + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Assert<{ AMAX >= BMAX }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<AMAX>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Assert<{ AMAX > BMAX }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<AMAX>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Assert<{ BMAX > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<AMAX>>;

    fn div(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Assert<{ BMAX > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<AMAX>>;

    fn div(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Refinement<Type, unsigned::LTE<{ AMAX + BMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ AMAX + BMAX }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Refinement<Type, unsigned::LTE<{ AMAX + BMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ AMAX + BMAX }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Assert<{ AMAX >= BMAX }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<AMAX>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Assert<{ AMAX >= BMAX }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<AMAX>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Refinement<Type, unsigned::LT<{ (AMAX) * (BMAX - 1) + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ (AMAX) * (BMAX - 1) }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Refinement<Type, unsigned::LTE<{ AMAX * BMAX }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ AMAX * BMAX }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Assert<{ BMAX > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<AMAX>>;

    fn div(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LTE<BMAX>>> for Refinement<Type, unsigned::LTE<AMAX>>
where
    Assert<{ BMAX >= 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<AMAX>>;

    fn div(self, rhs: Refinement<Type, unsigned::LTE<BMAX>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_unsigned_lt_add() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LT<20>>::refine(19).unwrap();
        let result: Refinement<u8, unsigned::LT<29>> = a + b;
        assert_eq!(*result, 28);
    }

    #[test]
    fn test_unsigned_lt_add_lte() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let result: Refinement<u8, unsigned::LT<40>> = a + b;
        assert_eq!(*result, 39);
    }

    #[test]
    fn test_unsigned_lt_mul() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<37>> = a * b;
        assert_eq!(*result, 36);
    }

    #[test]
    fn test_unsigned_lt_mul_lte() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<46>> = a * b;
        assert_eq!(*result, 45);
    }

    #[test]
    fn test_unsigned_lt_sub() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let result: Refinement<u8, unsigned::LT<15>> = a - b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_lt_sub_lte() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(19).unwrap();
        let b = Refinement::<u8, unsigned::LTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a - b;
        assert_eq!(*result, 9);
    }

    #[test]
    fn test_unsigned_lt_div() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a / b;
        assert_eq!(*result, 4);
    }

    #[test]
    fn test_unsigned_lt_div_lte() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a / b;
        assert_eq!(*result, 4);
    }

    #[test]
    fn test_unsigned_lte_add() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LTE<25>>::refine(20).unwrap();
        let result: Refinement<u8, unsigned::LTE<40>> = a + b;
        assert_eq!(*result, 35);
    }

    #[test]
    fn test_unsigned_lte_add_lt() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LT<25>>::refine(24).unwrap();
        let result: Refinement<u8, unsigned::LTE<40>> = a + b;
        assert_eq!(*result, 39);
    }

    #[test]
    fn test_unsigned_lte_sub() {
        let a = Refinement::<u8, unsigned::LTE<20>>::refine(20).unwrap();
        let b = Refinement::<u8, unsigned::LTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::LTE<20>> = a - b;
        assert_eq!(*result, 10);
    }

    #[test]
    fn test_unsigned_lte_sub_lt() {
        let a = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let result: Refinement<u8, unsigned::LTE<25>> = a - b;
        assert_eq!(*result, 11);
    }

    #[test]
    fn test_unsigned_lte_mul() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<75>> = a * b;
        assert_eq!(*result, 75);
    }

    #[test]
    fn test_unsigned_lte_mul_lt() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<60>> = a * b;
        assert_eq!(*result, 60);
    }

    // write tests for unsigned_lte_div and unsigned_lte_div_lt
    #[test]
    fn test_unsigned_lte_div() {
        let a = Refinement::<u8, unsigned::LTE<20>>::refine(20).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<20>> = a / b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_lte_div_lt() {
        let a = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<25>> = a / b;
        assert_eq!(*result, 6);
    }
}
