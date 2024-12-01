#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use serde::Deserialize;

pub mod boolean;
pub mod boundable;
pub mod character;
#[cfg(feature = "implication")]
pub mod implication;
#[cfg(feature = "implication")]
pub use implication::*;

pub trait Predicate<T> {
    fn test(value: &T) -> bool;
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Refined<T>(T);

#[derive(Deserialize)]
#[serde(try_from = "Refined<T>")]
pub struct Refinement<T, P: Predicate<T>>(T, #[serde(skip)] PhantomData<P>);

// TODO: replace result types here with something better
impl<T, P: Predicate<T>> Refinement<T, P> {
    pub fn refine(value: T) -> Result<Self, String> {
        Self::try_from(Refined(value))
    }

    pub fn modify<F>(self, fun: F) -> Result<Self, String>
    where
        F: Fn(T) -> T,
    {
        Self::refine(fun(self.0))
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
