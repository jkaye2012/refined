//! Logical type implication.

pub trait Implies<T> {
    fn imply(self) -> T;
}

mod boolean_imp;
mod boundable_imp;
