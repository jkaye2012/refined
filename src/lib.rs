#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

pub mod boolean;
pub mod boundable;
pub mod character;
pub mod string;

#[cfg(feature = "implication")]
pub mod implication;
#[cfg(feature = "implication")]
pub use implication::*;

pub trait Predicate<T> {
    fn test(value: &T) -> bool;
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(transparent))]
struct Refined<T>(T);

impl<T: Clone, P: Predicate<T> + Clone> From<Refinement<T, P>> for Refined<T> {
    fn from(value: Refinement<T, P>) -> Self {
        Refined(value.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct Refinement<T: Clone, P: Predicate<T> + Clone>(T, PhantomData<P>);

// TODO: replace result types here with something better
impl<T: Clone, P: Predicate<T> + Clone> Refinement<T, P> {
    pub fn refine(value: T) -> Result<Self, String> {
        Self::try_from(Refined(value))
    }

    pub fn modify<F>(self, fun: F) -> Result<Self, String>
    where
        F: FnOnce(T) -> T,
    {
        Self::refine(fun(self.0))
    }
}

impl<T: Clone, P: Predicate<T> + Clone> std::ops::Deref for Refinement<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone, P: Predicate<T> + Clone> TryFrom<Refined<T>> for Refinement<T, P> {
    type Error = String;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        if P::test(&value.0) {
            Ok(Self(value.0, PhantomData))
        } else {
            Err(format!("Value out of bounds."))
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use crate::*;

    #[test]
    fn test_refinement_deserialize_success() {
        let value =
            serde_json::from_str::<Refinement<u8, boundable::unsigned::LessThan<5>>>("4").unwrap();
        assert_eq!(*value, 4);
    }

    // TODO: can `path-to-error` be somehow integrated natively?

    #[test]
    fn test_refinement_deserialize_failure() {
        let value = serde_json::from_str::<Refinement<u8, boundable::unsigned::LessThan<5>>>("5");
        assert!(value.is_err());
    }

    #[test]
    fn test_refinement_serialize() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }
}
