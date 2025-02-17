use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Predicate, Refined, RefinementError, RefinementOps};

pub trait StatefulPredicate<T>: Default + Predicate<T> {
    fn test(&self, value: &T) -> bool {
        <Self as Predicate<T>>::test(value)
    }

    fn error(&self) -> String {
        <Self as Predicate<T>>::error()
    }
}

/// A refinement of a type `T` certifying that the [StatefulPredicate] `P` holds.
///
/// This is useful in situations where the additional space required to hold the
/// predicate is outweighed by the cost of applying the predicate statelessly.
/// Whenever possible, [Refinement] should be preferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct StatefulRefinement<T: Clone, P: StatefulPredicate<T> + Clone>(T, P);

impl<T: Clone, P: StatefulPredicate<T> + Clone> RefinementOps<T> for StatefulRefinement<T, P> {
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

#[cfg(test)]
mod tests {}
