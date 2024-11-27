#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use serde::Deserialize;

pub mod boolean;
pub mod boundable;

pub trait Predicate<T> {
    fn test(value: &T) -> bool;
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Refined<T>(T);

#[derive(Deserialize)]
#[serde(try_from = "Refined<T>")]
pub struct Refinement<T, P: Predicate<T>>(T, #[serde(skip)] PhantomData<P>);

impl<T, P: Predicate<T>> Refinement<T, P> {
    pub fn refine(value: T) -> Result<Self, String> {
        Self::try_from(Refined(value))
    }
}

impl<T, P: Predicate<T>> TryFrom<Refined<T>> for Refinement<T, P> {
    type Error = String;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        if P::test(&value.0) {
            Ok(Self(value.0, PhantomData))
        } else {
            Err(format!("Value out of bounds."))
        }
    }
}

impl<F, T, Type> Covers<Refinement<Type, T>> for Refinement<Type, F>
where
    F: Predicate<Type> + Covers<T>,
    T: Predicate<Type>,
{
    fn covered(self) -> Refinement<Type, T> {
        Refinement(self.0, PhantomData)
    }
}

pub trait Covers<T> {
    fn covered(self) -> T;
}

pub(crate) enum Assert<const CHECK: bool> {}

pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}
