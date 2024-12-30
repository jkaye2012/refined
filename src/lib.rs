//! Basic [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for the Rust standard library.
//!
//! Refinement is the process of imbuing types with predicates, allowing maintainers to see immediately
//! that types must be constrained with certain invariants, and ensuring that those invariants hold at
//! run time.
//!
//! In addition to the [Predicate] implementations provided for the standard library, `refined` also
//! provides a simple mechanism for defining your own refinement types.
//!
//! Most users will be interested primarily in the [Refinement] struct, which allows a [Predicate] to be
//! applied to values of a type and ensures that the predicate always holds.
//!
//! # Features
//!
//! * `serde`: enabled by default; allows [Refinement] to be serialized and deserialized using the `serde` library.
//!   This functionality was actually my main motivation for writing the crate in the first place, but technically
//!   the serde dependency is not required for the core functionality of the trait, so it can be disabled
//! * `implication`: enabling implication allows the use of the [Implies] trait; this is behind an off-by-default
//!   feature because it requires [generic_const_exprs](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html),
//!   which is both unstable and incomplete. The functionality is very useful, but its stability cannot be guaranteed
//!
//! # Examples
//!
//! ## Basic usage
//!
//! ```
//! use refined::{Refinement, RefinementError, boundable::unsigned::{LessThanEqual, ClosedInterval}};
//!
//! type FrobnicatorName = Refinement<String, ClosedInterval<1, 10>>;
//!
//! type FrobnicatorSize = Refinement<u8, LessThanEqual<100>>;
//!
//! struct Frobnicator {
//!   name: FrobnicatorName,
//!   size: FrobnicatorSize
//! }
//!
//! impl Frobnicator {
//!   pub fn new(name: String, size: u8) -> Result<Frobnicator, RefinementError> {
//!     let name = FrobnicatorName::refine(name)?;
//!     let size = FrobnicatorSize::refine(size)?;
//!
//!     Ok(Self {
//!       name,
//!       size
//!     })
//!   }
//! }
//!
//! assert!(Frobnicator::new("Good name".to_string(), 99).is_ok());
//! assert!(Frobnicator::new("Bad name, too long".to_string(), 99).is_err());
//! assert!(Frobnicator::new("Good name".to_string(), 123).is_err());
//! ```
//!
//! ## Serde support
//!
//! ```
//! use refined::{Refinement, boundable::unsigned::LessThan};
//! use serde::{Serialize, Deserialize};
//! use serde_json::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Example {
//!   name: String,
//!   size: Refinement<u8, LessThan<100>>
//! }
//!
//! let good: Result<Example, _> =  from_str(r#"{"name":"Good example","size":99}"#);
//! assert!(good.is_ok());
//! let bad: Result<Example, _> =  from_str(r#"{"name":"Bad example","size":123}"#);
//! assert!(bad.is_err());
//! ```
//!
//! ## Implication
//!
//! Note that enabling `incomplete_features` and `generic_const_exprs` is **required** for
//! the [Implies] trait bounds to be met.
//!
//! ```
//! #![allow(incomplete_features), feature(generic_const_exprs)]
//!
//! use refined::{Refinement, boundable::unsigned::LessThan, Implies};
//!
//! fn takes_lt_100(value: Refinement<u8, LessThan<100>>) -> String {
//!   format!("{}", value)
//! }
//!
//! let lt_50: Refinement<u8, LessThan<50>> = Refinement::refine(49).unwrap();
//! let ex: Refinement<u8, LessThan<51>> = lt_50.imply();
//! let result = takes_lt_100(lt_50.imply());
//! assert_eq!(result, "49");
//! ```
#![cfg_attr(
    feature = "implication",
    allow(incomplete_features),
    feature(generic_const_exprs)
)]

use std::fmt::Display;
use std::marker::PhantomData;

use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod boolean;
pub mod boundable;
pub mod character;
pub mod string;

pub use boundable::signed::SignedBoundable;
pub use boundable::unsigned::UnsignedBoundable;
pub use string::TypeString;

#[cfg(feature = "implication")]
pub mod implication;
#[cfg(feature = "implication")]
pub use implication::*;

/// An assertion that must hold for an instance of a type to be considered refined.
pub trait Predicate<T> {
    /// Whether a value satisfies the predicate.
    fn test(value: &T) -> bool;

    /// An error message to display when the predicate doesn't hold.
    fn error() -> String;
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

#[cfg(feature = "implication")]
impl<F, T, Type: Clone> Implies<Refinement<Type, T>> for Refinement<Type, F>
where
    F: Predicate<Type> + Implies<T> + Clone,
    T: Predicate<Type> + Clone,
{
    fn imply(self) -> Refinement<Type, T> {
        Refinement(self.0, PhantomData)
    }
}

impl<T: Clone + Display, P: Predicate<T> + Clone> Display for Refinement<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// An [Error] that can result from failed refinement.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefinementError(String);

impl Display for RefinementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Refinement violated: {}", self.0)
    }
}

impl<T: Clone, P: Predicate<T> + Clone> Refinement<T, P> {
    /// Attempts to refine a runtime value with the type's imbued predicate.
    pub fn refine(value: T) -> Result<Self, RefinementError> {
        Self::try_from(Refined(value))
    }

    /// Attempts a modification of a refined value, re-certifying that the predicate
    /// still holds after the modification is complete.
    pub fn modify<F>(self, fun: F) -> Result<Self, RefinementError>
    where
        F: FnOnce(T) -> T,
    {
        Self::refine(fun(self.0))
    }

    /// Attempts a replacement of a refined value, re-certifying that the predicate
    /// holds for the new value.
    pub fn replace(self, value: T) -> Result<Self, RefinementError> {
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
    type Error = RefinementError;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        if P::test(&value.0) {
            Ok(Self(value.0, PhantomData))
        } else {
            Err(RefinementError(P::error()))
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

    #[test]
    fn test_refinement_deserialize_failure() {
        let err = serde_json::from_str::<Refinement<u8, boundable::unsigned::LessThan<5>>>("5")
            .unwrap_err();
        assert_eq!(
            format!("{}", err),
            "Refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_refinement_serialize() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }
}
