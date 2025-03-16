//! Convenience re-exports for the most common `refined` functionality.

#[cfg(feature = "serde")]
pub use crate::NamedSerde;
pub use crate::{
    type_string, Named, Predicate, Refinement, RefinementError, RefinementOps, StatefulPredicate,
    StatefulRefinementOps, TypeString,
};

#[cfg(feature = "implication")]
pub use crate::Implies;
