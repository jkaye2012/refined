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
//! applied to values of a type and ensures that the predicate always holds. To access most of the functionality
//! available for [Refinement], you'll also need to import the [RefinementOps] trait (or, [StatefulRefinementOps]
//! if you're sure that you require [stateful refinement](stateful-refinement)).
//!
//! You may find it easiest to import the required types using the [prelude] module. Note that the prelude does
//! not include any predicates, only the basic type machinery required for refinement in general.
//!
//! # Examples
//!
//! In addition to the examples included here, you can also refer to the
//! [examples on GitHub](https://github.com/jkaye2012/refined/tree/main/examples) for complete end-to-end examples
//! that could you easily build and run yourself.
//!
//! ## Basic usage
//!
//! This examples demonstrates the "lowest level" raw usage of `refined` for simple refinement. Note that use
//! of the [prelude] is not required, though it will be used for brevity in most other examples.
//!
//! ```
//! use refined::{Refinement, RefinementOps, RefinementError, boundable::unsigned::{LessThanEqual, ClosedInterval}};
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
//! ## Stateful refinement
//!
//! While most type refinements can (and should) be implemented statelessly, it is possible to refine types in
//! ways that are more efficient/ergonomic using runtime state. For these cases, [StatefulRefinementOps] and
//! [StatefulPredicate] are provided.
//!
//! Because all [StatefulPredicate] are also [Predicate], you can move seamlessly between stateful and stateless
//! certification without the underlying refinement type being aware of how it was materialized. This means that
//! the `serde` feature functions transparently with stateful predicates, but it's important to be aware that the
//! `Serialize` and `Deserialize` implementations will use the stateless variants (as there's no way to easily
//! "inject" the predicate state into the serde process).
//!
//! The `regex` feature provides a good motivation for when it could make sense to use [StatefulRefinementOps]; compiling
//! the regular expression can be an expensive operation, often more expensive than certifying the predicate itself. We
//! can use the same [Regex](string::Regex) predicate both statefully and stateless as mentioned above:
//!
//! ```
//! use refined::{prelude::*, string::Regex};
//!
//! type_string!(AllZs, "^z+$");
//! type OopsAllZs = Refinement<String, Regex<AllZs>>;
//!
//! // Stateless refinement as usual, requires re-compiling the regex for every certification
//! assert!(OopsAllZs::refine("zzzzz".to_string()).is_ok());
//!
//! // Stateful refinement, we carry around the pre-compiled regex so that it can be re-used
//! let all_zs = Regex::<AllZs>::default();
//! assert!(OopsAllZs::refine_with_state(&all_zs, "zzzzz".to_string()).is_ok());
//! assert!(OopsAllZs::refine_with_state(&all_zs, "zazzy".to_string()).is_err());
//! ```
//!
//! ## Named refinement
//!
//! As you can see in the error messages above, there are two possible fields that could have led to the error in refinement,
//! but it isn't readily apparent which field caused the error by reading the error message. While this isn't a problem
//! when using libraries like [serde_path_to_error](https://docs.rs/serde_path_to_error/latest/serde_path_to_error/), this
//! can be important functionality to have in your own error messages if you're using basic serde functionality.
//!
//! If this is something that you need, consider using [Named], or [NamedSerde] if using `serde`.
//!
//! ```
//! use refined::{prelude::*, boundable::unsigned::{LessThanEqual, ClosedInterval}};
//!
//! type_string!(Name, "name");
//! type FrobnicatorName = Named<Name, Refinement<String, ClosedInterval<1, 10>>>;
//!
//! type_string!(Size, "size");
//! type FrobnicatorSize = Named<Size, Refinement<u8, LessThanEqual<100>>>;
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
//! use refined::{Refinement, RefinementOps, boundable::unsigned::LessThan};
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
//! assert_eq!(bad.unwrap_err().to_string(), "refinement violated: must be less than 100 at line 1 column 33");
//! ```
//!
//! If using named refinement, only [NamedSerde] will work in serde implementations:
//!
//! ```
//! use refined::{Refinement, RefinementOps, NamedSerde, boundable::unsigned::LessThan, type_string, TypeString};
//! use serde::{Serialize, Deserialize};
//! use serde_json::{from_str, to_string};
//!
//! type_string!(ExampleFieldName, "john");
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Example {
//!   name: String,
//!   size: NamedSerde<ExampleFieldName, Refinement<u8, LessThan<100>>>
//! }
//!
//! let good: Result<Example, _> =  from_str(r#"{"name":"Good example","size":99}"#);
//! assert!(good.is_ok());
//! let bad: Result<Example, _> =  from_str(r#"{"name":"Bad example","size":123}"#);
//! assert!(bad.is_err());
//! assert_eq!(bad.unwrap_err().to_string(), "refinement violated: john must be less than 100 at line 1 column 33");
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
//! use refined::{Refinement, RefinementOps, boundable::unsigned::LessThan, Implies};
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
//! use refined::{prelude::*, boundable::unsigned::OpenInterval};
//!
//! let bigger_range: Refinement<u8, OpenInterval<1, 100>> = Refinement::refine(50).unwrap();
//! let smaller_range: Refinement<u8, OpenInterval<25, 75>> = Refinement::refine(50).unwrap();
//! let incompatible_range: Refinement<u8, OpenInterval<101, 200>> = Refinement::refine(150).unwrap();
//! // assert_eq!(bigger_range, smaller_range); // Fails to compile, type mismatch
//! // assert_eq!(bigger_range, incompatible_range) // Fails to compile, invalid implication
//! assert_eq!(bigger_range, smaller_range.imply()); // Works!
//! ```
//!
//! Note that the order matters here; the smaller range refinement can be implied to the larger range,
//! but the opposite is logically invalid.
//!
//! ## Arithmetic
//!
//! ```
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//!
//! use refined::{prelude::*, boundable::unsigned::ClosedInterval};
//!
//! type SkillLevel = Refinement<u8, ClosedInterval<1, 10>>;
//!
//! /// A couple's aggregate skill level is the addition of their individual skill levels
//! fn couple_skill(a: SkillLevel, b: SkillLevel) -> Refinement<u8, ClosedInterval<2, 20>> {
//!    a + b // The addition here doesn't require a runtime bounds check
//! }
//!
//! let tom_skill = SkillLevel::refine(9).unwrap();
//! let sally_skill = SkillLevel::refine(6).unwrap();
//!
//! assert_eq!(*couple_skill(tom_skill, sally_skill), 15);
//! ```
//!
//! ```
//! #![allow(incomplete_features)]
//! #![feature(generic_const_exprs)]
//!
//! use refined::{prelude::*, boundable::signed::LessThan};
//!
//! type LT100 = Refinement<i16, LessThan<100>>;
//! type LT50 = Refinement<i16, LessThan<50>>;
//! let result: Refinement<i16, LessThan<149>> = LT100::refine(99).unwrap() + LT50::refine(49).unwrap();
//! assert_eq!(*result, 148);
//! ````
//!
//! # Provided refinements
//!
//! `refined` comes packaged with a large number of refinements over commonly used `std` types. The refinements
//! are grouped into modules based on the type of refinement that they provide.
//!
//! Here's a quick reference of what is currently available:
//!
//! * [boundable::unsigned] contains refinements for anything that implements [UnsignedBoundable];
//!   these are types that can be reduced to an unsigned size so that their size can be bounded. Examples
//!   include `String`, `u8`, `u64`, or any `std` container-like type that implements a `len()` method
//! * [boundable::signed] contains refinements for anything that implements [SignedBoundable];
//!   these are types that can be reduced to a signed size so that their size can be bounded. Examples include
//!   `i8`, `i64`, and `isize`
//! * [boolean] contains "combinator" refinements that allow other refinements to be combined with one another. Examples include
//!   [And](boolean::And) and [Or](boolean::Or)
//! * [character] contains refinements of [char]. Examples include [IsLowercase](character::IsLowercase) and [IsWhitespace](character::IsWhitespace)
//! * [string] contains refinements of any type that implements [AsRef\<str\>](AsRef). Examples include [Contains](string::Contains),
//!   [Trimmed](string::Trimmed), and [Regex](string::Regex)
//!
//! # Features
//!
//! ## `serde`
//!
//! Enabled by default; allows [Refinement] to be serialized and deserialized using the `serde` library.
//! This functionality was actually my main motivation for writing the crate in the first place, but technically
//! the serde dependency is not required for the core functionality of the trait, so it can be disabled
//!
//! ## `regex`
//!
//! Enabling regex allows the use of the [Regex](string::Regex) predicate. This carries a dependency on the [regex] crate
//!
//! ## `implication`
//!
//! Enabling implication allows the use of the [Implies] trait; this is behind an off-by-default
//! feature because it requires [generic_const_exprs](https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html),
//! which is both unstable and incomplete. The functionality is very useful, but its stability cannot be guaranteed
//!
//! ## `arithmetic`
//!
//! Enabling arithmetic provides implementations of many of the [std::ops] traits for relevant [Refinement]
//! types. Enabling this feature also automatically enables `implication` and correspondingly requires `generic_const_exprs`
//! as detailed above.
//!
//! Because the relationship of refined types
//! allows for the immediate computation of the resulting bounds, refined arithmetic should have no additional overhead
//! compared to raw arithmetic operations. Runtime bounds checking is not required.
//!
//! Following the types that implement arithmetic can be difficult. The support for bounds across different types is not perfect,
//! and may be improved in the future. Currently, support is provided for the four primary arithmetic operations
//! ([std::ops::Add], [std::ops::Sub], [std::ops::Mul], and [std::ops::Div]) for all meaningful combinations of both
//! signed and unsigned boundable ranges. For unsigned ranges, this means all operations are implemented for all range types. For
//! signed ranges, addition is implemented for all range types, while subtraction, multiplication, and division are implemented
//! only for ranges with both minimum _and_ maximum bounds.
//!
//! For example, [boundable::unsigned::LessThan] can be added, subtracted, multiplied, or divided with any type
//! that satisfies [implication::UnsignedMax], while [boundable::unsigned::GreaterThan] instead supports operations
//! against [implication::UnsignedMin]. The range types support operations against one another via [implication::UnsignedMinMax].
//!
//! Similarly, the signed variants are [implication::SignedMin], [implication::SignedMax], and [implication::SignedMinMax].
//!
//! See the examples above for more intuition.
#![cfg_attr(
    feature = "implication",
    allow(incomplete_features),
    feature(generic_const_exprs)
)]
#![feature(doc_cfg)]

use std::fmt::Display;

use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod boolean;
pub mod boundable;
pub mod character;
pub mod prelude;
pub mod string;

mod refinement;
pub use refinement::*;

pub use boundable::signed::SignedBoundable;
pub use boundable::unsigned::UnsignedBoundable;

#[doc(cfg(feature = "implication"))]
#[cfg(feature = "implication")]
pub mod implication;
#[doc(cfg(feature = "implication"))]
#[cfg(feature = "implication")]
pub use implication::*;

/// A string lifted into a context where it can be used as a type.
///
/// Most string predicates require type-level strings, but currently strings are not supported
/// as const generic trait bounds. `TypeString` is a workaround for this limitation.
pub trait TypeString: Default {
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
pub struct Refined<T>(T);

/// An [Error] that can result from failed refinement.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefinementError(String);

impl Display for RefinementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "refinement violated: {}", self.0)
    }
}

/// Operations that can be made available on all types of refinement.
pub trait RefinementOps:
    TryFrom<Refined<Self::T>, Error = RefinementError> + std::ops::Deref<Target = Self::T>
{
    type T;

    /// Attempts to refine a runtime value with the type's imbued predicate.
    fn refine(value: Self::T) -> Result<Self, RefinementError> {
        Self::try_from(Refined(value))
    }

    /// Attempts a modification of a refined value, re-certifying that the predicate
    /// still holds after the modification is complete.
    fn modify<F>(self, fun: F) -> Result<Self, RefinementError>
    where
        F: FnOnce(Self::T) -> Self::T,
    {
        Self::refine(fun(self.extract()))
    }

    /// Attempts a replacement of a refined value, re-certifying that the predicate
    /// holds for the new value.
    fn replace(self, value: Self::T) -> Result<Self, RefinementError> {
        Self::refine(value)
    }

    /// Destructively removes the refined value from the `Refinement` wrapper.
    ///
    /// For a non-destructive version, use the [std::ops::Deref] implementation instead.
    fn extract(self) -> Self::T;
}

/// Operations that can be made available on all types of stateful refinement.
pub trait StatefulRefinementOps<T, P: StatefulPredicate<T>>: RefinementOps<T = T> {
    /// Attempts to refine a runtime value with the type's imbued predicate, statefully.
    fn refine_with_state(predicate: &P, value: T) -> Result<Self, RefinementError>;

    /// Attempts a modification of a refined value, re-certifying that the stateful predicate
    /// still holds after the modification is complete.
    fn modify_with_state<F>(self, predicate: &P, fun: F) -> Result<Self, RefinementError>
    where
        F: FnOnce(<Self as RefinementOps>::T) -> <Self as RefinementOps>::T,
    {
        Self::refine_with_state(predicate, fun(self.extract()))
    }

    /// Attempts a replacement of a refined value, re-certifying that the stateful predicate
    /// holds for the new value.
    fn replace_with_state(
        self,
        predicate: &P,
        value: <Self as RefinementOps>::T,
    ) -> Result<Self, RefinementError> {
        Self::refine_with_state(predicate, value)
    }
}
