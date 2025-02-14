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
}
