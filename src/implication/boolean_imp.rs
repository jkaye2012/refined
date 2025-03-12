use core::marker::PhantomData;

use crate::boolean::*;

use super::Implies;

impl<F1, T1, F2, T2> Implies<And<T1, T2>> for And<F1, F2>
where
    F1: Implies<T1>,
    F2: Implies<T2>,
{
    fn imply(self) -> And<T1, T2> {
        And::<T1, T2>(PhantomData, PhantomData)
    }
}
