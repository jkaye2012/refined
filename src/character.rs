use crate::Predicate;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsControl;

impl Predicate<char> for IsControl {
    fn test(value: &char) -> bool {
        value.is_control()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsDigit;

impl Predicate<char> for IsDigit {
    fn test(value: &char) -> bool {
        value.is_ascii_digit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsLowercase;

impl Predicate<char> for IsLowercase {
    fn test(value: &char) -> bool {
        value.is_lowercase()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsUppercase;

impl Predicate<char> for IsUppercase {
    fn test(value: &char) -> bool {
        value.is_uppercase()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsNumeric;

impl Predicate<char> for IsNumeric {
    fn test(value: &char) -> bool {
        value.is_numeric()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsWhitespace;

impl Predicate<char> for IsWhitespace {
    fn test(value: &char) -> bool {
        value.is_whitespace()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsHexDigit;

impl Predicate<char> for IsHexDigit {
    fn test(value: &char) -> bool {
        value.is_ascii_hexdigit()
    }
}

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
