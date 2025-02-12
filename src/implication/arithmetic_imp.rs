use std::marker::PhantomData;

use crate::{boundable::*, Refinement};

impl<
        const AMAX: usize,
        const BMAX: usize,
        Type: Clone + unsigned::UnsignedBoundable + std::ops::Add<Output = Type>,
    > std::ops::Add<Refinement<Type, unsigned::LT<BMAX>>> for Refinement<Type, unsigned::LT<AMAX>>
where
    Refinement<Type, unsigned::LT<{ AMAX + BMAX - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ AMAX + BMAX - 1 }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LT<BMAX>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
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
}
