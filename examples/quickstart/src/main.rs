//! Demonstrates basic usage of `refined`.

use refined::{
    boundable::unsigned::{ClosedInterval, LessThanEqual},
    type_string, NamedRefinement, RefinementError, TypeString,
};

type_string!(Name, "name");
type_string!(Size, "size");

type FrobnicatorName = NamedRefinement<Name, String, ClosedInterval<1, 10>>;

type FrobnicatorSize = NamedRefinement<Size, u8, LessThanEqual<100>>;

#[derive(Debug)]
struct Frobnicator {
    name: FrobnicatorName,
    size: FrobnicatorSize,
}

impl Frobnicator {
    pub fn new(name: String, size: u8) -> Result<Frobnicator, RefinementError> {
        let name = FrobnicatorName::refine(name)?;
        let size = FrobnicatorSize::refine(size)?;

        Ok(Self { name, size })
    }
}

fn main() {
    println!(
        "Good name, good age: {:?}",
        Frobnicator::new("Good name".to_string(), 99).is_ok()
    );
    println!(
        "Bad name, too long: {:?}",
        Frobnicator::new("Bad name, too long".to_string(), 99)
    );
    println!(
        "Bad age, too large: {:?}",
        Frobnicator::new("Good name".to_string(), 123)
    );
}
