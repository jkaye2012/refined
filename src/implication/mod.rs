pub trait Implies<T> {
    fn imply(self) -> T;
}

mod boolean_imp;
mod boundable_imp;

pub use boolean_imp::*;
pub use boundable_imp::*;
