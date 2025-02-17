//! [char] refinement.
//!
//! # Example
//!
//! ```
//! use refined::{Refinement, RefinementOps, character::IsDigit};
//!
//! type Test = Refinement<char, IsDigit>;
//!
//! assert!(Test::refine('0').is_ok());
//! assert!(Test::refine('a').is_err());
//! ```
use crate::{Predicate, StatefulPredicate};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsControl;

impl Predicate<char> for IsControl {
    fn test(value: &char) -> bool {
        value.is_control()
    }

    fn error() -> String {
        String::from("must be a control character")
    }
}

impl StatefulPredicate<char> for IsControl {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsDigit;

impl Predicate<char> for IsDigit {
    fn test(value: &char) -> bool {
        value.is_ascii_digit()
    }

    fn error() -> String {
        String::from("must be a digit")
    }
}

impl StatefulPredicate<char> for IsDigit {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsLowercase;

impl Predicate<char> for IsLowercase {
    fn test(value: &char) -> bool {
        value.is_lowercase()
    }

    fn error() -> String {
        String::from("must be a lowercase character")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsUppercase;

impl Predicate<char> for IsUppercase {
    fn test(value: &char) -> bool {
        value.is_uppercase()
    }

    fn error() -> String {
        String::from("must be an uppercase character")
    }
}

impl StatefulPredicate<char> for IsLowercase {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsNumeric;

impl Predicate<char> for IsNumeric {
    fn test(value: &char) -> bool {
        value.is_numeric()
    }

    fn error() -> String {
        String::from("must be a numeric character")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsWhitespace;

impl Predicate<char> for IsWhitespace {
    fn test(value: &char) -> bool {
        value.is_whitespace()
    }

    fn error() -> String {
        String::from("must be a whitespace character")
    }
}

impl StatefulPredicate<char> for IsNumeric {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct IsHexDigit;

impl Predicate<char> for IsHexDigit {
    fn test(value: &char) -> bool {
        value.is_ascii_hexdigit()
    }

    fn error() -> String {
        String::from("must be a valid hex character")
    }
}

impl StatefulPredicate<char> for IsHexDigit {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_is_control() {
        type Test = Refinement<char, IsControl>;
        assert!(Test::refine('\u{009C}').is_ok());
        assert!(Test::refine('0').is_err());
    }

    #[test]
    fn test_is_digit() {
        type Test = Refinement<char, IsDigit>;
        assert!(Test::refine('a').is_err());
        assert!(Test::refine('0').is_ok());
    }

    #[test]
    fn test_is_lowercase() {
        type Test = Refinement<char, IsLowercase>;
        assert!(Test::refine('A').is_err());
        assert!(Test::refine('a').is_ok());
    }

    #[test]
    fn test_is_uppercase() {
        type Test = Refinement<char, IsUppercase>;
        assert!(Test::refine('A').is_ok());
        assert!(Test::refine('a').is_err());
    }

    #[test]
    fn test_is_numeric() {
        type Test = Refinement<char, IsNumeric>;
        assert!(Test::refine('A').is_err());
        assert!(Test::refine('0').is_ok());
    }

    #[test]
    fn test_is_whitespace() {
        type Test = Refinement<char, IsWhitespace>;
        assert!(Test::refine(' ').is_ok());
        assert!(Test::refine('a').is_err());
    }

    #[test]
    fn test_is_hex_digit() {
        type Test = Refinement<char, IsHexDigit>;
        assert!(Test::refine('F').is_ok());
        assert!(Test::refine('G').is_err());
    }
}
