//! Basic [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for the Rust standard library.
//!
//! In addition to the [Predicate] implementations provided for the standard library, `refined` also
//! provides a simple mechanism for defining your own refinement types.
//!
//! # Features
//!
//! # Examples
#![cfg_attr(feature = "implication", allow(incomplete_features))]
#![cfg_attr(feature = "implication", feature(generic_const_exprs))]

use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod boolean;
pub mod boundable;
pub mod character;
pub mod string;

pub use boundable::signed::SignedBoundable;
pub use boundable::unsigned::UnsignedBoundable;

#[cfg(feature = "implication")]
pub mod implication;
#[cfg(feature = "implication")]
pub use implication::*;

/// An assertion that must hold for an instance of a type to be considered refined.
pub trait Predicate<T> {
    /// Whether a value satisfies the predicate.
    fn test(value: &T) -> bool;
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(transparent))]
struct Refined<T>(T);

impl<T: Clone, P: Predicate<T> + Clone> From<Refinement<T, P>> for Refined<T> {
    fn from(value: Refinement<T, P>) -> Self {
        Refined(value.0)
    }
}

/// A refinement of a type `T` certifying that the [Predicate] `P` holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct Refinement<T: Clone, P: Predicate<T> + Clone>(T, PhantomData<P>);

// TODO: replace result types here with something better
impl<T: Clone, P: Predicate<T> + Clone> Refinement<T, P> {
    /// Attempts to refine a runtime value with the type's imbued predicate.
    pub fn refine(value: T) -> Result<Self, String> {
        Self::try_from(Refined(value))
    }

    /// Attempts a modification of a refined value, re-certifying that the predicate
    /// still holds after the modification is complete.
    pub fn modify<F>(self, fun: F) -> Result<Self, String>
    where
        F: FnOnce(T) -> T,
    {
        Self::refine(fun(self.0))
    }

    /// Attempts a replacement of a refined value, re-certifying that the predicate
    /// holds for the new value.
    pub fn replace(self, value: T) -> Result<Self, String> {
        Self::refine(value)
    }

    /// Destructively removes the refined value from the `Refinement` wrapper.
    ///
    /// For a non-destructive version, use the [std::ops::Deref] implementation instead.
    pub fn extract(self) -> T {
        self.0
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
