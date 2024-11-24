use std::marker::PhantomData;

use serde::Deserialize;

pub mod boundable;

pub trait Predicate<T> {
    fn test(value: &T) -> bool;
}

pub struct And<T, P1: Predicate<T>, P2: Predicate<T>>(
    PhantomData<T>,
    PhantomData<P1>,
    PhantomData<P2>,
);

impl<T, P1: Predicate<T>, P2: Predicate<T>> Predicate<T> for And<T, P1, P2> {
    fn test(value: &T) -> bool {
        P1::test(value) && P2::test(value)
    }
}

pub trait Container {
    fn size(&self) -> usize;
}

impl Container for String {
    fn size(&self) -> usize {
        self.len()
    }
}

pub struct NonEmpty;

impl<T: Container> Predicate<T> for NonEmpty {
    fn test(value: &T) -> bool {
        value.size() > 0
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
struct Refined<T>(T);

#[derive(Deserialize)]
#[serde(try_from = "Refined<T>")]
pub struct Refinement<T, P: Predicate<T>>(T, #[serde(skip)] PhantomData<P>);

impl<T, P: Predicate<T>> Refinement<T, P> {
    pub fn refine(value: T) -> Result<Self, String> {
        Self::try_from(Refined(value))
    }
}

impl<T, P: Predicate<T>> TryFrom<Refined<T>> for Refinement<T, P> {
    type Error = String;

    fn try_from(value: Refined<T>) -> Result<Self, Self::Error> {
        if P::test(&value.0) {
            Ok(Self(value.0, PhantomData))
        } else {
            Err(format!("Value out of bounds."))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Deserialize)]
//     #[serde(transparent)]
//     struct BaseName(Refinement<String, And<String, NonEmpty, Bounded<0, 16>>>);

//     #[test]
//     fn test_base_name_deserialize_success() {
//         let result: BaseName = serde_json::from_str("\"testing\"").unwrap();
//         assert_eq!(&result.0 .0, "testing");
//     }
// }
