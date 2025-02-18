use std::{marker::PhantomData, ops::Deref};

use crate::{
    Refined, RefinementError, RefinementOps, StatefulPredicate, StatefulRefinementOps, TypeString,
};

/// A named refinement over a refinement type `R`.
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
/// type ExampleBounded = Named<Example, BoundedLong>;
///
/// assert_eq!(&ExampleBounded::refine(99).unwrap_err().to_string(), "refinement violated: example name must be greater than 100");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Named<N: TypeString + Clone, R: Clone + RefinementOps>(R, PhantomData<N>);

impl<N: TypeString + Clone, R: Clone + RefinementOps> Deref for Named<N, R> {
    type Target = R::T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<N: TypeString + Clone, R: Clone + RefinementOps> AsRef<R> for Named<N, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<N: TypeString + Clone, R: Clone + RefinementOps> TryFrom<Refined<R::T>> for Named<N, R> {
    type Error = RefinementError;

    fn try_from(value: Refined<R::T>) -> Result<Self, Self::Error> {
        match R::refine(value.0) {
            Ok(value) => Ok(Self(value, PhantomData)),
            Err(err) => Err(RefinementError(format!("{} {}", N::VALUE, err.0))),
        }
    }
}

impl<N: TypeString + Clone, R: Clone + RefinementOps> From<Named<N, R>> for Refined<R::T> {
    fn from(value: Named<N, R>) -> Self {
        Refined(value.extract())
    }
}

impl<N: TypeString + Clone, R: Clone + RefinementOps> RefinementOps for Named<N, R> {
    type T = R::T;

    fn extract(self) -> Self::T {
        self.0.extract()
    }
}

impl<N: TypeString + Clone, T, P: StatefulPredicate<T>, R: Clone + StatefulRefinementOps<T, P>>
    StatefulRefinementOps<T, P> for Named<N, R>
{
    fn refine_with_state(predicate: &P, value: T) -> Result<Self, RefinementError> {
        match R::refine_with_state(predicate, value) {
            Ok(value) => Ok(Self(value, PhantomData)),
            Err(err) => Err(RefinementError(format!("{} {}", N::VALUE, err.0))),
        }
    }
}

#[cfg(feature = "serde")]
mod named_serde {
    use super::*;
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    /// A named refinement over a refinement type `R`. Supports [serde].
    ///
    /// See [Named] for more information and examples. The only difference between the two structs
    /// is serde support.
    #[derive(
        Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
    )]
    #[serde(try_from = "Refined<R::T>", into = "Refined<R::T>")]
    pub struct NamedSerde<N: TypeString + Clone, R: Clone + RefinementOps>(R, PhantomData<N>)
    where
        R::T: Serialize + DeserializeOwned;

    impl<N: TypeString + Clone, R: Clone + RefinementOps> Deref for NamedSerde<N, R>
    where
        R::T: Serialize + DeserializeOwned,
    {
        type Target = R::T;

        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }

    impl<N: TypeString + Clone, R: Clone + RefinementOps> AsRef<R> for NamedSerde<N, R>
    where
        R::T: Serialize + DeserializeOwned,
    {
        fn as_ref(&self) -> &R {
            &self.0
        }
    }

    impl<N: TypeString + Clone, R: Clone + RefinementOps> TryFrom<Refined<R::T>> for NamedSerde<N, R>
    where
        R::T: Serialize + DeserializeOwned,
    {
        type Error = RefinementError;

        fn try_from(value: Refined<R::T>) -> Result<Self, Self::Error> {
            match R::refine(value.0) {
                Ok(value) => Ok(Self(value, PhantomData)),
                Err(err) => Err(RefinementError(format!("{} {}", N::VALUE, err.0))),
            }
        }
    }

    impl<N: TypeString + Clone, R: Clone + RefinementOps> From<NamedSerde<N, R>> for Refined<R::T>
    where
        R::T: Serialize + DeserializeOwned,
    {
        fn from(value: NamedSerde<N, R>) -> Self {
            Refined(value.extract())
        }
    }

    impl<N: TypeString + Clone, R: Clone + RefinementOps> RefinementOps for NamedSerde<N, R>
    where
        R::T: Serialize + DeserializeOwned,
    {
        type T = R::T;

        fn extract(self) -> Self::T {
            self.0.extract()
        }
    }

    impl<
            N: TypeString + Clone,
            T: Serialize + DeserializeOwned,
            P: StatefulPredicate<T>,
            R: Clone + StatefulRefinementOps<T, P>,
        > StatefulRefinementOps<T, P> for NamedSerde<N, R>
    {
        fn refine_with_state(predicate: &P, value: T) -> Result<Self, RefinementError> {
            match R::refine_with_state(predicate, value) {
                Ok(value) => Ok(Self(value, PhantomData)),
                Err(err) => Err(RefinementError(format!("{} {}", N::VALUE, err.0))),
            }
        }
    }
}

#[cfg(feature = "serde")]
pub use named_serde::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    type_string!(Test, "test");

    #[cfg(feature = "serde")]
    #[test]
    fn test_named_refinement_deserialize_success() {
        let value = serde_json::from_str::<
            NamedSerde<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>,
        >("4")
        .unwrap();
        assert_eq!(*value, 4);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_named_refinement_deserialize_failure() {
        let err = serde_json::from_str::<
            NamedSerde<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>,
        >("5")
        .unwrap_err();
        assert_eq!(
            format!("{}", err),
            "refinement violated: test must be less than 5"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_named_refinement_serialize() {
        let value = NamedSerde::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>::refine(4)
            .unwrap();
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }

    #[test]
    fn test_named_refinement_modify_success() {
        let value = Named::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(3).unwrap(),
            PhantomData,
        );
        let modified = value.modify(|x| x + 1).unwrap();
        assert_eq!(*modified, 4);
    }

    #[test]
    fn test_named_refinement_modify_failure() {
        let value = Named::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
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
        let value = Named::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
        );
        let replaced = value.replace(3).unwrap();
        assert_eq!(*replaced, 3);
    }

    #[test]
    fn test_named_refinement_replace_failure() {
        let value = Named::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
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
        let value = Named::<Test, Refinement<u8, boundable::unsigned::LessThan<5>>>(
            Refinement::refine(4).unwrap(),
            PhantomData,
        );
        let extracted = value.extract();
        assert_eq!(extracted, 4);
    }
}
