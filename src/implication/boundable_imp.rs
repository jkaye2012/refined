use std::marker::PhantomData;

use crate::boundable::*;
use crate::{Predicate, Refinement};

use super::Implies;

impl<F, T, Type: Clone> Implies<Refinement<Type, T>> for Refinement<Type, F>
where
    F: Predicate<Type> + Implies<T> + Clone,
    T: Predicate<Type> + Clone,
{
    fn imply(self) -> Refinement<Type, T> {
        Refinement(self.0, PhantomData)
    }
}

pub(crate) enum Assert<const CHECK: bool> {}

pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

impl<const F: isize, const T: isize> Implies<signed::GreaterThan<T>> for signed::GreaterThan<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThan<T> {
        signed::GreaterThan::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::GreaterThan<T>> for signed::GreaterThanEqual<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThan<T> {
        signed::GreaterThan::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::GreaterThanEqual<T>> for signed::GreaterThan<F>
where
    Assert<{ F >= T }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThanEqual<T> {
        signed::GreaterThanEqual::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::GreaterThanEqual<T>>
    for signed::GreaterThanEqual<F>
where
    Assert<{ F >= T }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThanEqual<T> {
        signed::GreaterThanEqual::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::LessThan<T>> for signed::LessThan<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> signed::LessThan<T> {
        signed::LessThan::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::LessThanEqual<T>> for signed::LessThan<F>
where
    Assert<{ F <= T }>: IsTrue,
{
    fn imply(self) -> signed::LessThanEqual<T> {
        signed::LessThanEqual::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::LessThanEqual<T>> for signed::LessThanEqual<F>
where
    Assert<{ F <= T }>: IsTrue,
{
    fn imply(self) -> signed::LessThanEqual<T> {
        signed::LessThanEqual::<T>
    }
}

impl<const F: isize, const T: isize> Implies<signed::LessThan<T>> for signed::LessThanEqual<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> signed::LessThan<T> {
        signed::LessThan::<T>
    }
}

impl<const VAL: isize, const MIN: isize> Implies<signed::GreaterThan<MIN>> for signed::Equals<VAL>
where
    Assert<{ VAL > MIN }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThan<MIN> {
        signed::GreaterThan::<MIN>
    }
}

impl<const VAL: isize, const MIN: isize> Implies<signed::GreaterThanEqual<MIN>>
    for signed::Equals<VAL>
where
    Assert<{ VAL >= MIN }>: IsTrue,
{
    fn imply(self) -> signed::GreaterThanEqual<MIN> {
        signed::GreaterThanEqual::<MIN>
    }
}

impl<const VAL: isize, const MAX: isize> Implies<signed::LessThan<MAX>> for signed::Equals<VAL>
where
    Assert<{ VAL < MAX }>: IsTrue,
{
    fn imply(self) -> signed::LessThan<MAX> {
        signed::LessThan::<MAX>
    }
}

impl<const VAL: isize, const MAX: isize> Implies<signed::LessThanEqual<MAX>> for signed::Equals<VAL>
where
    Assert<{ VAL <= MAX }>: IsTrue,
{
    fn imply(self) -> signed::LessThanEqual<MAX> {
        signed::LessThanEqual::<MAX>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::GreaterThan<T>> for unsigned::GreaterThan<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThan<T> {
        unsigned::GreaterThan::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::GreaterThanEqual<T>>
    for unsigned::GreaterThan<F>
where
    Assert<{ F >= T }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThanEqual<T> {
        unsigned::GreaterThanEqual::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::GreaterThanEqual<T>>
    for unsigned::GreaterThanEqual<F>
where
    Assert<{ F >= T }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThanEqual<T> {
        unsigned::GreaterThanEqual::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::GreaterThan<T>>
    for unsigned::GreaterThanEqual<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThan<T> {
        unsigned::GreaterThan::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::LessThan<T>> for unsigned::LessThan<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThan<T> {
        unsigned::LessThan::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::LessThanEqual<T>> for unsigned::LessThan<F>
where
    Assert<{ F <= T }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThanEqual<T> {
        unsigned::LessThanEqual::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::LessThanEqual<T>>
    for unsigned::LessThanEqual<F>
where
    Assert<{ F <= T }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThanEqual<T> {
        unsigned::LessThanEqual::<T>
    }
}

impl<const F: usize, const T: usize> Implies<unsigned::LessThan<T>> for unsigned::LessThanEqual<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThan<T> {
        unsigned::LessThan::<T>
    }
}

impl<const VAL: usize, const MIN: usize> Implies<unsigned::GreaterThan<MIN>>
    for unsigned::Equals<VAL>
where
    Assert<{ VAL > MIN }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThan<MIN> {
        unsigned::GreaterThan::<MIN>
    }
}

impl<const VAL: usize, const MIN: usize> Implies<unsigned::GreaterThanEqual<MIN>>
    for unsigned::Equals<VAL>
where
    Assert<{ VAL >= MIN }>: IsTrue,
{
    fn imply(self) -> unsigned::GreaterThanEqual<MIN> {
        unsigned::GreaterThanEqual::<MIN>
    }
}

impl<const VAL: usize, const MAX: usize> Implies<unsigned::LessThan<MAX>> for unsigned::Equals<VAL>
where
    Assert<{ VAL < MAX }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThan<MAX> {
        unsigned::LessThan::<MAX>
    }
}

impl<const VAL: usize, const MAX: usize> Implies<unsigned::LessThanEqual<MAX>>
    for unsigned::Equals<VAL>
where
    Assert<{ VAL <= MAX }>: IsTrue,
{
    fn imply(self) -> unsigned::LessThanEqual<MAX> {
        unsigned::LessThanEqual::<MAX>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_signed_gt_implication() {
        let g = Refinement::<i32, signed::GreaterThan<15>>::refine(20).unwrap();
        let _: Refinement<i32, signed::GreaterThan<10>> = g.imply();
        let _: Refinement<i32, signed::GreaterThanEqual<15>> = g.imply();
        // let r: Refinement<i32, signed::GreaterThan<20>> = g.imply();
    }

    #[test]
    fn test_signed_gte_implication() {
        let g = Refinement::<i32, signed::GreaterThanEqual<15>>::refine(20).unwrap();
        let _: Refinement<i32, signed::GreaterThan<14>> = g.imply();
        let _: Refinement<i32, signed::GreaterThanEqual<15>> = g.imply();
        // let _: Refinement<i32, signed::GreaterThanEqual<16>> = g.imply();
    }

    #[test]
    fn test_signed_lt_implication() {
        let l = Refinement::<i32, signed::LessThan<15>>::refine(10).unwrap();
        let _: Refinement<i32, signed::LessThan<16>> = l.imply();
        let _: Refinement<i32, signed::LessThanEqual<15>> = l.imply();
        // let _: Refinement<i32, signed::LessThan<10>> = l.imply();
    }

    #[test]
    fn test_signed_lte_implication() {
        let l = Refinement::<i32, signed::LessThanEqual<15>>::refine(10).unwrap();
        let _: Refinement<i32, signed::LessThan<16>> = l.imply();
        let _: Refinement<i32, signed::LessThanEqual<15>> = l.imply();
        // let _: Refinement<i32, signed::LessThanEqual<14>> = l.imply();
    }

    #[test]
    fn test_signed_eq_gt_implication() {
        let e = Refinement::<i32, signed::Equals<15>>::refine(15).unwrap();
        let _: Refinement<i32, signed::GreaterThan<14>> = e.imply();
        let _: Refinement<i32, signed::GreaterThanEqual<15>> = e.imply();
        // let _: Refinement<i32, signed::GreaterThan<16>> = e.imply();
    }

    #[test]
    fn test_signed_eq_gte_implication() {
        let e = Refinement::<i32, signed::Equals<15>>::refine(15).unwrap();
        let _: Refinement<i32, signed::GreaterThan<14>> = e.imply();
        let _: Refinement<i32, signed::GreaterThanEqual<15>> = e.imply();
        // let _: Refinement<i32, signed::GreaterThanEqual<16>> = e.imply();
    }

    #[test]
    fn test_signed_eq_lt_implication() {
        let e = Refinement::<i32, signed::Equals<15>>::refine(15).unwrap();
        let _: Refinement<i32, signed::LessThan<16>> = e.imply();
        let _: Refinement<i32, signed::LessThanEqual<15>> = e.imply();
        // let _: Refinement<i32, signed::LessThan<10>> = e.imply();
    }

    #[test]
    fn test_signed_eq_lte_implication() {
        let e = Refinement::<i32, signed::Equals<15>>::refine(15).unwrap();
        let _: Refinement<i32, signed::LessThan<16>> = e.imply();
        let _: Refinement<i32, signed::LessThanEqual<15>> = e.imply();
        // let _: Refinement<i32, signed::LessThanEqual<14>> = e.imply();
    }

    #[test]
    fn test_unsigned_gt_implication() {
        let g = Refinement::<usize, unsigned::GreaterThan<15>>::refine(20).unwrap();
        let _: Refinement<usize, unsigned::GreaterThan<10>> = g.imply();
        let _: Refinement<usize, unsigned::GreaterThanEqual<15>> = g.imply();
        // let _: Refinement<usize, unsigned::GreaterThan<16>> = g.imply();
    }

    #[test]
    fn test_unsigned_gte_implication() {
        let g = Refinement::<usize, unsigned::GreaterThanEqual<15>>::refine(20).unwrap();
        let _: Refinement<usize, unsigned::GreaterThan<14>> = g.imply();
        let _: Refinement<usize, unsigned::GreaterThanEqual<15>> = g.imply();
        // let _: Refinement<usize, unsigned::GreaterThanEqual<16>> = g.imply();
    }

    #[test]
    fn test_unsigned_lt_implication() {
        let l = Refinement::<usize, unsigned::LessThan<15>>::refine(10).unwrap();
        let _: Refinement<usize, unsigned::LessThan<16>> = l.imply();
        let _: Refinement<usize, unsigned::LessThanEqual<15>> = l.imply();
        // let _: Refinement<usize, unsigned::LessThan<10>> = l.imply();
    }

    #[test]
    fn test_unsigned_lte_implication() {
        let l = Refinement::<usize, unsigned::LessThanEqual<15>>::refine(10).unwrap();
        let _: Refinement<usize, unsigned::LessThan<16>> = l.imply();
        let _: Refinement<usize, unsigned::LessThanEqual<15>> = l.imply();
        // let _: Refinement<usize, unsigned::LessThanEqual<14>> = l.imply();
    }

    #[test]
    fn test_unsigned_eq_gt_implication() {
        let e = Refinement::<usize, unsigned::Equals<15>>::refine(15).unwrap();
        let _: Refinement<usize, unsigned::GreaterThan<14>> = e.imply();
        let _: Refinement<usize, unsigned::GreaterThanEqual<15>> = e.imply();
        // let _: Refinement<usize, unsigned::GreaterThan<16>> = e.imply();
    }

    #[test]
    fn test_unsigned_eq_gte_implication() {
        let e = Refinement::<usize, unsigned::Equals<15>>::refine(15).unwrap();
        let _: Refinement<usize, unsigned::GreaterThan<14>> = e.imply();
        let _: Refinement<usize, unsigned::GreaterThanEqual<15>> = e.imply();
        // let _: Refinement<usize, unsigned::GreaterThanEqual<16>> = e.imply();
    }

    #[test]
    fn test_unsigned_eq_lt_implication() {
        let e = Refinement::<usize, unsigned::Equals<15>>::refine(15).unwrap();
        let _: Refinement<usize, unsigned::LessThan<16>> = e.imply();
        let _: Refinement<usize, unsigned::LessThanEqual<15>> = e.imply();
        // let _: Refinement<usize, unsigned::LessThan<10>> = e.imply();
    }

    #[test]
    fn test_unsigned_eq_lte_implication() {
        let e = Refinement::<usize, unsigned::Equals<15>>::refine(15).unwrap();
        let _: Refinement<usize, unsigned::LessThan<16>> = e.imply();
        let _: Refinement<usize, unsigned::LessThanEqual<15>> = e.imply();
        // let _: Refinement<usize, unsigned::LessThanEqual<14>> = e.imply();
    }
}
