use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Predicate, Refined, RefinementError, RefinementOps};

use super::Refinement;

/// An assertion that must hold for an instance of a type to be considered statefully refined.
///
/// Compared to [Predicate], the difference is that stateful predicates are "materialized" and
/// may carry state along with them to be re-used across what would otherwise be independent
/// tests.
pub trait StatefulPredicate<T>: Default + Predicate<T> {
    /// Whether a value satisfies the stateful predicate.
    fn test(&self, value: &T) -> bool {
        <Self as Predicate<T>>::test(value)
    }

    /// An error message to display when the stateful predicate doesn't hold.
    fn error(&self) -> String {
        <Self as Predicate<T>>::error()
    }
}

/// A refinement of a type `T` certifying that the [StatefulPredicate] `P` holds.
///
/// This is useful in situations where the additional space required to hold the
/// predicate is outweighed by the cost of applying the predicate statelessly.
/// Whenever possible, [Refinement](super::Refinement) should be preferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct StatefulRefinement<T: Clone, P: StatefulPredicate<T> + Clone>(T, P);

impl<T: Clone, P: StatefulPredicate<T> + Clone> RefinementOps for StatefulRefinement<T, P> {
    type T = T;

    fn extract(self) -> T {
        self.0
    }
}

impl<T: Clone + Display, P: StatefulPredicate<T> + Clone> Display for StatefulRefinement<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T: Clone, P: StatefulPredicate<T> + Clone> std::ops::Deref for StatefulRefinement<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone, P: StatefulPredicate<T> + Clone> From<StatefulRefinement<T, P>> for Refined<T> {
    fn from(value: StatefulRefinement<T, P>) -> Self {
        Refined(value.0)
    }
}

impl<T: Clone, P: StatefulPredicate<T> + Clone> TryFrom<Refined<T>> for StatefulRefinement<T, P> {
    type Error = RefinementError;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        let predicate = P::default();
        if predicate.test(&value.0) {
            Ok(Self(value.0, predicate))
        } else {
            Err(RefinementError(predicate.error()))
        }
    }
}

impl<T: Clone, P: StatefulPredicate<T> + Clone> From<Refinement<T, P>>
    for StatefulRefinement<T, P>
{
    fn from(value: Refinement<T, P>) -> Self {
        Self(value.extract(), P::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_stateful_refinement_deserialize_success() {
        let value =
            serde_json::from_str::<StatefulRefinement<u8, boundable::unsigned::LessThan<5>>>("4")
                .unwrap();
        assert_eq!(*value, 4);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_stateful_refinement_deserialize_failure() {
        let err =
            serde_json::from_str::<StatefulRefinement<u8, boundable::unsigned::LessThan<5>>>("5")
                .unwrap_err();
        assert_eq!(
            format!("{}", err),
            "refinement violated: must be less than 5"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_stateful_refinement_serialize() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(4, Default::default());
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }

    #[test]
    fn test_stateful_refinement_modify_success() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(3, Default::default());
        let modified = value.modify(|x| x + 1).unwrap();
        assert_eq!(*modified, 4);
    }

    #[test]
    fn test_stateful_refinement_modify_failure() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(4, Default::default());
        let modified = value.modify(|x| x + 1).unwrap_err();
        assert_eq!(
            format!("{}", modified),
            "refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_stateful_refinement_replace_success() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(4, Default::default());
        let replaced = value.replace(3).unwrap();
        assert_eq!(*replaced, 3);
    }

    #[test]
    fn test_stateful_refinement_replace_failure() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(4, Default::default());
        let replaced = value.replace(5).unwrap_err();
        assert_eq!(
            format!("{}", replaced),
            "refinement violated: must be less than 5"
        );
    }

    #[test]
    fn test_stateful_refinement_extract() {
        let value =
            StatefulRefinement::<u8, boundable::unsigned::LessThan<5>>(4, Default::default());
        let extracted = value.extract();
        assert_eq!(extracted, 4);
    }
}
