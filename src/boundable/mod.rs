//! Refinement of types that can be represented by an integral value.
//!
//! There are a large number of types that can be reduced to some integral value.
//! Signed and unsigned numeric types are the most obvious, but most "container"
//! types can be considered similarly by reduction to their length.
//!
//! Due to current restrictions of Rust's type system (namely, that const generics
//! don't support any form of polymorphism), signed and unsigned bounding must be
//! implemented independently. The signed numerics are implemented using signed bounds,
//! while all other types are implemented using unsigned bounds.
pub mod signed;
pub mod unsigned;
