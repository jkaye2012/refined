//! Basic [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for the Rust standard library.
//!
//! Refinement in this context is the process of imbuing types with predicates, allowing maintainers to see immediately
//! that types must be constrained with certain invariants and ensuring that those invariants hold at run time. This
//! allows types to be "narrowed" to a subset of their possible values. For a gentle introduction, you can refer to
//! [my blog post announcing the release of the library](https://jordankaye.dev/posts/refined/).
//!
//! In addition to the [Predicate] implementations provided for the standard library, `refined` also
//! provides a simple mechanism for defining your own refinement types.
//!
//! Most users will be interested primarily in the [Refinement] struct, which allows a [Predicate] to be
//! applied to values of a type and ensures that the predicate always holds.
//!
//! # Examples
//!
//! In addition to the examples included here, you can also refer to the
//! [examples on GitHub](https://github.com/jkaye2012/refined/tree/main/examples) for complete end-to-end examples
//! that could you easily build and run yourself.
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
//! #[derive(Debug)]
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
//! assert_eq!(Frobnicator::new("Bad name, too long".to_string(), 99).unwrap_err().to_string(),
//!            "refinement violated: must be greater than or equal to 1 and must be less than or equal to 10");
//! assert_eq!(Frobnicator::new("Good name".to_string(), 123).unwrap_err().to_string(),
//!            "refinement violated: must be less than or equal to 100");
//! ```
//!
//! ## Named refinement
//!
//! As you can see in the error messages above, there are two possible fields that could have led to the error in refinement,
//! but it isn't readily apparent which field caused the error by reading the error message. While this isn't a problem
//! when using libraries like [serde_path_to_error](https://docs.rs/serde_path_to_error/latest/serde_path_to_error/), this
//! can be important functionality to have in your own error messages if you're using basic serde functionality.
//!
//! If this is something that you need, consider using [NamedRefinement] instead of [Refinement].
//!
//! ```
//! use refined::{NamedRefinement, RefinementError, boundable::unsigned::{LessThanEqual, ClosedInterval}, type_string, TypeString};
//!
//! type_string!(Name, "name");
//! type FrobnicatorName = NamedRefinement<Name, String, ClosedInterval<1, 10>>;
//!
//! type_string!(Size, "size");
//! type FrobnicatorSize = NamedRefinement<Size, u8, LessThanEqual<100>>;
//!
//! #[derive(Debug)]
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
//! assert_eq!(Frobnicator::new("Bad name, too long".to_string(), 99).unwrap_err().to_string(),
//!            "refinement violated: name must be greater than or equal to 1 and must be less than or equal to 10");
//! assert_eq!(Frobnicator::new("Good name".to_string(), 123).unwrap_err().to_string(),
//!            "refinement violated: size must be less than or equal to 100");
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
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
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
//!
//! This design leads to some interesting emergent properties; for example, the "compatibility" of
//! range comparison over equality is enforced at compile time:
//!
//! ```
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//!
//! use refined::{Refinement, boundable::unsigned::OpenInterval, Implies};
//!
//! let bigger_range: Refinement<u8, OpenInterval<1, 100>> = Refinement::refine(50).unwrap();
//! let smaller_range: Refinement<u8, OpenInterval<25, 75>> = Refinement::refine(50).unwrap();
//! let incompatible_range: Refinement<u8, OpenInterval<101, 200>> = Refinement::refine(150).unwrap();
//! // assert_eq!(bigger_range, smaller_range); // Fails to compile, type mismatch
//! // assert_eq!(bigger_ragne, incompatible_range) // Fails to compile, invalid implication
//! assert_eq!(bigger_range, smaller_range.imply()); // Works!
//! ```
//!
//! Note that the order matters here; the smaller range refinement can be implied to the larger range,
//! but the opposite is logically invalid.
//!
//! # Provided refinements
//!
//! `refined` comes packaged with a large number of refinements over commonly used `std` types. The refinements
//! are grouped into modules based on the type of refinement that they provide.
//!
//! Here's a quick reference of what is currently available:
//!
//! * [UnsignedBoundable]: types that can be reduced to an unsigned size so that their size can be bounded. Examples
//!   include `String`, `u8`, `u64`, or any `std` container-like type that implements a `len()` method
//! * [SignedBoundable]: types that can be reduced to a signed size so that their size can be bounded. Examples include
//!   `i8`, `i64`, and `isize`
//! * [boolean]: "combinator" refinements that allow other refinements to be combined with one another. Examples include
//!   [And](boolean::And) and [Or](boolean::Or)
//! * [character]: refinements of [char]. Examples include [IsLowercase](character::IsLowercase) and [IsWhitespace](character::IsWhitespace)
//! * [string]: refinements of any type that implements [AsRef\<str\>](AsRef). Examples include [Contains](string::Contains) and
//!   [Trimmed](string::Trimmed)
//!
//! # Features
//!
//! * `serde`: enabled by default; allows [Refinement] to be serialized and deserialized using the `serde` library.
//!   This functionality was actually my main motivation for writing the crate in the first place, but technically
//!   the serde dependency is not required for the core functionality of the trait, so it can be disabled
//! * `implication`: enabling implication allows the use of the [Implies] trait; this is behind an off-by-default
//!   feature because it requires [generic_const_exprs](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html),
//!   which is both unstable and incomplete. The functionality is very useful, but its stability cannot be guaranteed
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

#[cfg(feature = "implication")]
pub mod implication;
#[cfg(feature = "implication")]
pub use implication::*;

/// A string lifted into a context where it can be used as a type.
///
/// Most string predicates require type-level strings, but currently strings are not supported
/// as const generic trait bounds. `TypeString` is a workaround for this limitation.
pub trait TypeString {
    const VALUE: &'static str;
}

/// Creates a [type-level string](TypeString).
///
/// `$name` is the name of a type to create to hold the type-level string.
/// `$value` is the string that should be lifted into the type system.
///
/// Note that use of this macro requires that [TypeString] is in scope.
///
/// # Example
///
/// ```
/// use refined::{type_string, TypeString};
/// type_string!(FooBar, "very stringy");
/// assert_eq!(FooBar::VALUE, "very stringy");
/// ```
#[macro_export]
macro_rules! type_string {
    ($name:ident, $value:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name;

        impl TypeString for $name {
            const VALUE: &'static str = $value;
        }
    };
}

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

impl<N: TypeString + Clone, T: Clone, P: Predicate<T> + Clone> From<NamedRefinement<N, T, P>>
    for Refined<T>
{
    fn from(value: NamedRefinement<N, T, P>) -> Self {
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

impl<T: Clone, P: Predicate<T> + Clone> std::ops::Deref for Refinement<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
/// use refined::{type_string, TypeString, NamedRefinement, boundable::unsigned::GreaterThan};
///
/// type_string!(Example, "example name");
///
/// type BoundedLong = NamedRefinement<Example, u64, GreaterThan<100>>;
///
/// assert_eq!(&BoundedLong::refine(99).unwrap_err().to_string(), "refinement violated: example name must be greater than 100");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(try_from = "Refined<T>", into = "Refined<T>")
)]
pub struct NamedRefinement<N: TypeString + Clone, T: Clone, P: Predicate<T> + Clone>(
    T,
    PhantomData<P>,
    PhantomData<N>,
);

impl<N: TypeString + Clone, T: Clone + Display, P: Predicate<T> + Clone> Display
    for NamedRefinement<N, T, P>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl<N: TypeString + Clone, T: Clone, P: Predicate<T> + Clone> std::ops::Deref
    for NamedRefinement<N, T, P>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An [Error] that can result from failed refinement.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefinementError(String);

impl Display for RefinementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "refinement violated: {}", self.0)
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

impl<N: TypeString + Clone, T: Clone, P: Predicate<T> + Clone> NamedRefinement<N, T, P> {
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

impl<N: TypeString + Clone, T: Clone, P: Predicate<T> + Clone> TryFrom<Refined<T>>
    for NamedRefinement<N, T, P>
{
    type Error = RefinementError;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        if P::test(&value.0) {
            Ok(Self(value.0, PhantomData, PhantomData))
        } else {
            Err(RefinementError(format!("{} {}", N::VALUE, P::error())))
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use crate::*;

    type_string!(Test, "test");

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

    #[test]
    fn test_named_refinement_deserialize_success() {
        let value = serde_json::from_str::<
            NamedRefinement<Test, u8, boundable::unsigned::LessThan<5>>,
        >("4")
        .unwrap();
        assert_eq!(*value, 4);
    }

    #[test]
    fn test_named_refinement_deserialize_failure() {
        let err =
            serde_json::from_str::<NamedRefinement<Test, u8, boundable::unsigned::LessThan<5>>>(
                "5",
            )
            .unwrap_err();
        assert_eq!(
            format!("{}", err),
            "refinement violated: test must be less than 5"
        );
    }

    #[test]
    fn test_named_refinement_serialize() {
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            4,
            PhantomData,
            PhantomData,
        );
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "4");
    }

    #[test]
    fn test_named_refinement_modify_success() {
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            3,
            PhantomData,
            PhantomData,
        );
        let modified = value.modify(|x| x + 1).unwrap();
        assert_eq!(*modified, 4);
    }

    #[test]
    fn test_named_refinement_modify_failure() {
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            4,
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
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            4,
            PhantomData,
            PhantomData,
        );
        let replaced = value.replace(3).unwrap();
        assert_eq!(*replaced, 3);
    }

    #[test]
    fn test_named_refinement_replace_failure() {
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            4,
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
        let value = NamedRefinement::<Test, u8, boundable::unsigned::LessThan<5>>(
            4,
            PhantomData,
            PhantomData,
        );
        let extracted = value.extract();
        assert_eq!(extracted, 4);
    }
}
