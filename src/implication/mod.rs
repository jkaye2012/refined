//! Logical type implication.

/// Enables implication from a source refinement into a target refinement when the source can be guaranteed
/// to satisfy the predicate of the target.
///
/// For example, if we have a `Refinement<u8, LessThan<100>>`, then we can be sure that the value contained
/// within the refinement is less than 100. This also logically implies that the value is less than 101,
/// 105, or indeed any value greater than 100.
///
/// With the `implication` feature enabled (and the corresponding language feature enabled, `generic_const_exprs`),
/// `Implies` allows us to encode this relationship like so:
///
/// ```
/// #![allow(incomplete_features)]
/// #![feature(generic_const_exprs)]
///
/// use refined::{Refinement, RefinementOps, boundable::unsigned::LessThan, Implies};
///
/// fn takes_lt_100(value: Refinement<u8, LessThan<100>>) -> String {
///   format!("{}", value)
/// }
///
/// let lt_50: Refinement<u8, LessThan<50>> = Refinement::refine(49).unwrap();
/// let ex: Refinement<u8, LessThan<51>> = lt_50.imply();
/// let result = takes_lt_100(lt_50.imply());
/// assert_eq!(result, "49");
/// ```
///
/// ## Implementation
///
/// While it is possible to create your own `Implies` implementations, doing so will require some type-level
/// machinery that isn't exposed by `std`, and that `refined` also keeps private. Because `generic_const_exprs`
/// is unstable, there is a good chance that the implementation details behind `implication` may change
/// significantly in the future, and I do not wish this these changes to require a major version release.
///
/// See [the source](https://github.com/jkaye2012/refined/blob/main/src/implication/boundable_imp.rs#L5-L18)
/// for an example of how the internals work currently. To create your own implementation, you need some version
/// of `Assert` and `IsTrue`. The idea is that the generic const expr in the `Implies` implementation trait
/// bound will only resolve to a valid `IsTrue` implementation when the logical predicate is satisfied.
pub trait Implies<T> {
    fn imply(self) -> T;
}

pub(crate) enum Assert<const CHECK: bool> {}

pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

mod boolean_imp;
mod boundable_imp;

#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
#[cfg(feature = "arithmetic")]
mod arithmetic;

#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
#[cfg(feature = "arithmetic")]
pub use arithmetic::*;
