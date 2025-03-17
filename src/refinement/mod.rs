mod named;

use core::{fmt::Display, marker::PhantomData};

pub use named::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    Predicate, Refined, RefinementError, RefinementOps, StatefulPredicate, StatefulRefinementOps,
};

#[cfg(feature = "implication")]
use crate::Implies;

/// A refinement of a type `T` certifying that the [Predicate] `P` holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Refinement<T, P: Predicate<T>>(pub(crate) T, pub(crate) PhantomData<P>);

#[cfg(feature = "serde")]
impl<T: Serialize, P: Predicate<T>> Serialize for Refinement<T, P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>, P: Predicate<T>> Deserialize<'de> for Refinement<T, P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let refined = Refined::<T>::deserialize(deserializer)?;
        Ok(Self::try_from(refined).map_err(serde::de::Error::custom)?)
    }
}

impl<T, P: Predicate<T>> RefinementOps for Refinement<T, P> {
    type T = T;

    fn take(self) -> T {
        #[cfg(feature = "optimized")]
        unsafe {
            P::optimize(&self.0);
        }
        self.0
    }

    fn extract(self) -> T {
        #[cfg(feature = "optimized")]
        unsafe {
            P::optimize(&self.0);
        }
        self.0
    }
}

impl<T: Display, P: Predicate<T>> Display for Refinement<T, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<T, P: Predicate<T>> core::ops::Deref for Refinement<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(feature = "optimized")]
        unsafe {
            P::optimize(&self.0);
        }
        &self.0
    }
}

impl<T, P: Predicate<T>> From<Refinement<T, P>> for Refined<T> {
    fn from(value: Refinement<T, P>) -> Self {
        Refined(value.0)
    }
}

impl<T, P: Predicate<T>> TryFrom<Refined<T>> for Refinement<T, P> {
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
impl<F, T, Type> Implies<Refinement<Type, T>> for Refinement<Type, F>
where
    F: Predicate<Type> + Implies<T>,
    T: Predicate<Type>,
{
    fn imply(self) -> Refinement<Type, T> {
        Refinement(self.0, PhantomData)
    }
}

impl<T, P: StatefulPredicate<T>> StatefulRefinementOps<T, P> for Refinement<T, P> {
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
    use alloc::format;

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
    fn test_refinement_take() {
        let value = Refinement::<u8, boundable::unsigned::LessThan<5>>(4, PhantomData);
        let extracted = value.take();
        assert_eq!(extracted, 4);
    }
}
