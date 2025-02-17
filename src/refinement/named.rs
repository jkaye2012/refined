use std::{marker::PhantomData, ops::Deref};

use serde::de::DeserializeOwned;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Refined, RefinementError, RefinementOps, TypeString};

/// A named refinement of a type `T` certifying that the [Predicate] `P` holds.
///
/// Named refinements are useful when more precise error messages are required.
/// When using features like `serde`, this is generally unnecessary because we
/// can instead rely on tools like [path_to_error](https://github.com/dtolnay/path-to-error)
/// rather than building the name into the type itself.
///
/// # Example
///
/// ```
/// use refined::{type_string, TypeString, Named, Refinement, RefinementOps, boundable::unsigned::GreaterThan};
///
/// type_string!(Example, "example name");
///
/// type BoundedLong = Refinement<u64, GreaterThan<100>>;
/// type ExampleBounded = Named<Example, u64, BoundedLong>;
///
/// assert_eq!(&ExampleBounded::refine(99).unwrap_err().to_string(), "refinement violated: example name must be greater than 100");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct Named<
    N: TypeString + Clone,
    T: Clone + DeserializeOwned + Serialize,
    R: Clone + RefinementOps<T>,
>(R, PhantomData<N>, PhantomData<T>);

impl<
        N: TypeString + Clone,
        T: Clone + DeserializeOwned + Serialize,
        R: Deref<Target = T> + Clone + RefinementOps<T>,
    > Deref for Named<N, T, R>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<
        N: TypeString + Clone,
        T: Clone + DeserializeOwned + Serialize,
        R: Deref<Target = T> + Clone + RefinementOps<T>,
    > AsRef<R> for Named<N, T, R>
{
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<
        N: TypeString + Clone,
        T: Clone + DeserializeOwned + Serialize,
        R: Clone + RefinementOps<T>,
    > TryFrom<Refined<T>> for Named<N, T, R>
{
    type Error = RefinementError;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        match R::refine(value.0) {
            Ok(value) => Ok(Self(value, PhantomData, PhantomData)),
            Err(err) => Err(RefinementError(format!("{} {}", N::VALUE, err.0))),
        }
    }
}

impl<
        N: TypeString + Clone,
        T: Clone + DeserializeOwned + Serialize,
        R: Clone + RefinementOps<T>,
    > From<Named<N, T, R>> for Refined<T>
{
    fn from(value: Named<N, T, R>) -> Self {
        Refined(value.extract())
    }
}

impl<
        N: TypeString + Clone,
        T: Clone + DeserializeOwned + Serialize,
        R: Clone + RefinementOps<T>,
    > RefinementOps<T> for Named<N, T, R>
{
    fn extract(self) -> T {
        self.0.extract()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    type_string!(Test, "test");

    #[test]
    fn test_named_refinement_deserialize_success() {
        let value = serde_json::from_str::<
            Named<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>,
        >("4")
        .unwrap();
        assert_eq!(*value, 4);
    }

    #[test]
    fn test_named_refinement_deserialize_failure() {
        let err = serde_json::from_str::<
            Named<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>,
        >("5")
        .unwrap_err();
        assert_eq!(
            format!("{}", err),
            "refinement violated: test must be less than 5"
        );
    }

    #[test]
    fn test_named_refinement_serialize() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
            PhantomData,
        );
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }

    #[test]
    fn test_named_refinement_modify_success() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(3).unwrap(),
            PhantomData,
            PhantomData,
        );
        let modified = value.modify(|x| x + 1).unwrap();
        assert_eq!(*modified, 4);
    }

    #[test]
    fn test_named_refinement_modify_failure() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
            PhantomData,
        );
        let modified = value.modify(|x| x + 1).unwrap_err();
        assert_eq!(
            format!("{}", modified),
            "refinement violated: test must be less than 5"
        );
    }

    #[test]
    fn test_named_refinement_replace_success() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
            PhantomData,
        );
        let replaced = value.replace(3).unwrap();
        assert_eq!(*replaced, 3);
    }

    #[test]
    fn test_named_refinement_replace_failure() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
            PhantomData,
        );
        let replaced = value.replace(5).unwrap_err();
        assert_eq!(
            format!("{}", replaced),
            "refinement violated: test must be less than 5"
        );
    }

    #[test]
    fn test_named_refinement_extract() {
        let value = Named::<Test, u8, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
            PhantomData,
        );
        let extracted = value.extract();
        assert_eq!(extracted, 4);
    }
}
