mod named;

use std::{fmt::Display, marker::PhantomData};

pub use named::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Predicate, Refined, RefinementError, RefinementOps, StatefulRefinementOps};

#[cfg(feature = "implication")]
use crate::Implies;

/// A refinement of a type `T` certifying that the [Predicate] `P` holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct Refinement<T: Clone, P: Predicate<T> + Clone>(T, PhantomData<P>);

impl<T: Clone, P: Predicate<T> + Clone> RefinementOps for Refinement<T, P> {
    type T = T;

    fn extract(self) -> T {
        self.0
    }
}

impl<T: Clone + Display, P: Predicate<T> + Clone> Display for Refinement<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T: Clone, P: Predicate<T> + Clone> std::ops::Deref for Refinement<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone, P: Predicate<T> + Clone> From<Refinement<T, P>> for Refined<T> {
    fn from(value: Refinement<T, P>) -> Self {
        Refined(value.0)
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

pub trait StatefulPredicate<T>: Default + Predicate<T> {
    fn test(&self, value: &T) -> bool;

    fn error(&self) -> String {
        <Self as Predicate<T>>::error()
    }
}

impl<T: Clone, P: StatefulPredicate<T> + Clone> StatefulRefinementOps<T, P> for Refinement<T, P> {
    fn refine_with_state(predicate: &P, value: T) -> Result<Self, RefinementError> {
        if predicate.test(&value) {
            Ok(Self(value, PhantomData))
        } else {
            Err(RefinementError(predicate.error()))
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
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
            "refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_refinement_serialize() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }

    #[test]
    fn test_refinement_modify_success() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(3, PhantomData);
        let modified = value.modify(|x| x + 1).unwrap();
        assert_eq!(*modified, 4);
    }

    #[test]
    fn test_refinement_modify_failure() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let modified = value.modify(|x| x + 1).unwrap_err();
        assert_eq!(
            format!("{}", modified),
            "refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_refinement_replace_success() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let replaced = value.replace(3).unwrap();
        assert_eq!(*replaced, 3);
    }

    #[test]
    fn test_refinement_replace_failure() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let replaced = value.replace(5).unwrap_err();
        assert_eq!(
            format!("{}", replaced),
            "refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_refinement_extract() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let extracted = value.extract();
        assert_eq!(extracted, 4);
    }
}
