use std::marker::PhantomData;

use crate::boundable::signed::*;
use crate::{Predicate, Refinement};

use super::Implies;

impl<F, T, Type> Implies<Refinement<Type, T>> for Refinement<Type, F>
where
    F: Predicate<Type> + Implies<T>,
    T: Predicate<Type>,
{
    fn imply(self) -> Refinement<Type, T> {
        Refinement(self.0, PhantomData)
    }
}

pub(crate) enum Assert<const CHECK: bool> {}

pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

impl<const F: isize, const T: isize> Implies<GreaterThan<T>> for GreaterThan<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> GreaterThan<T> {
        GreaterThan::<T>
    }
}

impl<const F: isize, const T: isize> Implies<GreaterThanEqual<T>> for GreaterThanEqual<F>
where
    Assert<{ F > T }>: IsTrue,
{
    fn imply(self) -> GreaterThanEqual<T> {
        GreaterThanEqual::<T>
    }
}

// TODO: GTE implication, other implications as well

impl<const F: isize, const T: isize> Implies<LessThan<T>> for LessThan<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> LessThan<T> {
        LessThan::<T>
    }
}

impl<const F: isize, const T: isize> Implies<LessThanEqual<T>> for LessThanEqual<F>
where
    Assert<{ F < T }>: IsTrue,
{
    fn imply(self) -> LessThanEqual<T> {
        LessThanEqual::<T>
    }
}

impl<const VAL: isize, const MIN: isize> Implies<GreaterThan<MIN>> for Equals<VAL>
where
    Assert<{ VAL > MIN }>: IsTrue,
{
    fn imply(self) -> GreaterThan<MIN> {
        GreaterThan::<MIN>
    }
}

impl<const VAL: isize, const MIN: isize> Implies<GreaterThanEqual<MIN>> for Equals<VAL>
where
    Assert<{ VAL >= MIN }>: IsTrue,
{
    fn imply(self) -> GreaterThanEqual<MIN> {
        GreaterThanEqual::<MIN>
    }
}

impl<const VAL: isize, const MAX: isize> Implies<LessThan<MAX>> for Equals<VAL>
where
    Assert<{ VAL < MAX }>: IsTrue,
{
    fn imply(self) -> LessThan<MAX> {
        LessThan::<MAX>
    }
}

impl<const VAL: isize, const MAX: isize> Implies<LessThanEqual<MAX>> for Equals<VAL>
where
    Assert<{ VAL <= MAX }>: IsTrue,
{
    fn imply(self) -> LessThanEqual<MAX> {
        LessThanEqual::<MAX>
    }
}
