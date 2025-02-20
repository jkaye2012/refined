use std::marker::PhantomData;
use std::ops::*;

use super::{Assert, IsTrue};
use crate::{boundable::*, Refinement};

mod unsigned_imp {
    use super::*;
    use crate::boundable::unsigned::*;

    macro_rules! constrain {
        ($constraint:expr) => {
            $constraint
        };
        () => {
            true
        };
    }

    macro_rules! math_impl {
        (Add, $a:ident, $b:ident, $expr:expr $(, $constraint:expr)?) => {
            impl<
                    const A: usize,
                    const B: usize,
                    Type: Clone + UnsignedBoundable + Add<Output = Type>,
                > Add<Refinement<Type, $b<B>>> for Refinement<Type, $a<A>>
            where
                Refinement<Type, $a<{ $expr }>>: Sized,
                Assert<{ constrain!($($constraint)?) }>: IsTrue,
            {
                type Output = Refinement<Type, $a<{ $expr }>>;

                fn add(self, rhs: Refinement<Type, $b<B>>) -> Self::Output {
                    Refinement(self.0 + rhs.0, PhantomData)
                }
            }
        };

        (Sub, $a:ident, $b:ident, $expr:expr) => {
            impl<
                    const A: usize,
                    const B: usize,
                    Type: Clone + UnsignedBoundable + Sub<Output = Type>,
                > Sub<Refinement<Type, $b<B>>> for Refinement<Type, $a<A>>
            where
                Refinement<Type, $a<$expr>>: Sized,
            {
                type Output = Refinement<Type, $a<$expr>>;

                fn sub(self, rhs: Refinement<Type, $b<B>>) -> Self::Output {
                    Refinement(self.0 - rhs.0, PhantomData)
                }
            }
        };

        (Mul, $a:ident, $b:ident, $expr:expr) => {
            impl<
                    const A: usize,
                    const B: usize,
                    Type: Clone + UnsignedBoundable + Mul<Output = Type>,
                > Mul<Refinement<Type, $b<B>>> for Refinement<Type, $a<A>>
            where
                Refinement<Type, $a<$expr>>: Sized,
            {
                type Output = Refinement<Type, $a<$expr>>;

                fn mul(self, rhs: Refinement<Type, $b<B>>) -> Self::Output {
                    Refinement(self.0 * rhs.0, PhantomData)
                }
            }
        };

        (Div, $a:ident, $b:ident, $expr:expr) => {
            impl<
                    const A: usize,
                    const B: usize,
                    Type: Clone + UnsignedBoundable + Div<Output = Type>,
                > Div<Refinement<Type, $b<B>>> for Refinement<Type, $a<A>>
            where
                Refinement<Type, $a<$expr>>: Sized,
            {
                type Output = Refinement<Type, $a<$expr>>;

                fn div(self, rhs: Refinement<Type, $b<B>>) -> Self::Output {
                    Refinement(self.0 / rhs.0, PhantomData)
                }
            }
        };
    }

    math_impl!(Add, LT, LT, A + B - 1);
    math_impl!(Add, LT, LTE, A + B);
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ (A - 1) * (B - 1) + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ (A - 1) * (B - 1) + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ (A - 1) * B + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ (A - 1) * B + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ (A - 1) * B + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LT<{ (A - 1) * B + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<A>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Assert<{ A > B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<A>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ A - B }>>: Sized,
    Assert<{ A > B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<{ A - B }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Assert<{ B > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<A>>;

    fn div(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Assert<{ B > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<A>>;

    fn div(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LT<A>>
where
    Refinement<Type, unsigned::LT<{ A / B }>>: Sized,
    Assert<{ B > 0 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LT<{ A / B }>>;

    fn div(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<A>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<A>>;

    fn sub(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A - B }>>: Sized,
    Assert<{ A > B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<{ A - B }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LT<{ (A) * (B - 1) + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ (A) * (B - 1) }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A * B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ A * B }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ (A - 1) * B + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::LTE<{ (A - 1) * B + 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}
impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LTE<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Assert<{ B >= 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<A>>;

    fn div(self, rhs: Refinement<Type, unsigned::LTE<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}
impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::LT<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Assert<{ B > 1 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<A>>;

    fn div(self, rhs: Refinement<Type, unsigned::LT<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::LTE<A>>
where
    Refinement<Type, unsigned::LTE<{ A / B }>>: Sized,
    Assert<{ B > 0 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::LTE<{ A / B }>>;

    fn div(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A + B + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ A + B + 1 }>>;

    fn add(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ (A + 1) * (B + 1) - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ (A + 1) * (B + 1) - 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ (A + 1) * B - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ (A + 1) * B - 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ (A + 1) * B - 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GT<{ (A + 1) * B - 1 }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}
impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A - B - 1 }>>: Sized,
    Assert<{ A > B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GT<{ A - B - 1 }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A - B }>>: Sized,
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GT<{ A - B }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A - B }>>: Sized,
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GT<{ A - B }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GT<A>>
{
    type Output = Refinement<Type, unsigned::GTE<0>>;

    fn div(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Assert<{ B > 0 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GTE<0>>;

    fn div(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::Equals<B>>> for Refinement<Type, unsigned::GT<A>>
where
    Refinement<Type, unsigned::GT<{ A / B - 1 }>>: Sized,
    Assert<{ B > 0 }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GT<{ A / B - 1 }>>;

    fn div(self, rhs: Refinement<Type, unsigned::Equals<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GTE<{ A + B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GTE<{ A + B }>>;

    fn add(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Add<Output = Type>,
    > Add<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GTE<{ A + B + 1 }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GTE<{ A + B + 1 }>>;

    fn add(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 + rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GTE<{ A - B - 1 }>>: Sized,
    Assert<{ A > B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GTE<{ A - B - 1 }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Sub<Output = Type>,
    > Sub<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GTE<{ A - B }>>: Sized,
    Assert<{ A >= B }>: IsTrue,
{
    type Output = Refinement<Type, unsigned::GTE<{ A - B }>>;

    fn sub(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 - rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GT<{ (A) * (B + 1) }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GTE<{ (A) * (B + 1) }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Mul<Output = Type>,
    > Mul<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GTE<A>>
where
    Refinement<Type, unsigned::GTE<{ A * B }>>: Sized,
{
    type Output = Refinement<Type, unsigned::GTE<{ A * B }>>;

    fn mul(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 * rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::GT<B>>> for Refinement<Type, unsigned::GTE<A>>
{
    type Output = Refinement<Type, unsigned::GTE<0>>;

    fn div(self, rhs: Refinement<Type, unsigned::GT<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

impl<
        const A: usize,
        const B: usize,
        Type: Clone + unsigned::UnsignedBoundable + Div<Output = Type>,
    > Div<Refinement<Type, unsigned::GTE<B>>> for Refinement<Type, unsigned::GTE<A>>
{
    type Output = Refinement<Type, unsigned::GTE<0>>;

    fn div(self, rhs: Refinement<Type, unsigned::GTE<B>>) -> Self::Output {
        Refinement(self.0 / rhs.0, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_unsigned_lt_add() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LT<20>>::refine(19).unwrap();
        let result: Refinement<u8, unsigned::LT<29>> = a + b;
        assert_eq!(*result, 28);
    }

    #[test]
    fn test_unsigned_lt_add_lte() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let result: Refinement<u8, unsigned::LT<40>> = a + b;
        assert_eq!(*result, 39);
    }

    #[test]
    fn test_unsigned_lt_add_eq() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a + b;
        assert_eq!(*result, 19);
    }

    #[test]
    fn test_unsigned_lt_mul() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<37>> = a * b;
        assert_eq!(*result, 36);
    }

    #[test]
    fn test_unsigned_lt_mul_lte() {
        let a = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<46>> = a * b;
        assert_eq!(*result, 45);
    }

    #[test]
    fn test_unsigned_lt_mul_eq() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<71>> = a * b;
        assert_eq!(*result, 70);
    }

    #[test]
    fn test_unsigned_lt_sub() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::LT<10>>::refine(9).unwrap();
        let result: Refinement<u8, unsigned::LT<15>> = a - b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_lt_sub_lte() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(19).unwrap();
        let b = Refinement::<u8, unsigned::LTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a - b;
        assert_eq!(*result, 9);
    }

    #[test]
    fn test_unsigned_lt_sub_eq() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<10>> = a - b;
        assert_eq!(*result, 9);
    }

    #[test]
    fn test_unsigned_lt_div() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a / b;
        assert_eq!(*result, 4);
    }

    #[test]
    fn test_unsigned_lt_div_lte() {
        let a = Refinement::<u8, unsigned::LT<20>>::refine(18).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LT<20>> = a / b;
        assert_eq!(*result, 4);
    }

    #[test]
    fn test_unsigned_lt_div_eq() {
        let a = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LT<3>> = a / b;
        assert_eq!(*result, 2);
    }

    #[test]
    fn test_unsigned_lte_add() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LTE<25>>::refine(20).unwrap();
        let result: Refinement<u8, unsigned::LTE<40>> = a + b;
        assert_eq!(*result, 35);
    }

    #[test]
    fn test_unsigned_lte_add_lt() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LT<25>>::refine(24).unwrap();
        let result: Refinement<u8, unsigned::LTE<40>> = a + b;
        assert_eq!(*result, 39);
    }

    #[test]
    fn test_unsigned_lte_add_eq() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<20>> = a + b;
        assert_eq!(*result, 19);
    }

    #[test]
    fn test_unsigned_lte_sub() {
        let a = Refinement::<u8, unsigned::LTE<20>>::refine(20).unwrap();
        let b = Refinement::<u8, unsigned::LTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::LTE<20>> = a - b;
        assert_eq!(*result, 10);
    }

    #[test]
    fn test_unsigned_lte_sub_lt() {
        let a = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::LT<15>>::refine(14).unwrap();
        let result: Refinement<u8, unsigned::LTE<25>> = a - b;
        assert_eq!(*result, 11);
    }

    #[test]
    fn test_unsigned_lte_sub_eq() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<10>> = a - b;
        assert_eq!(*result, 9);
    }

    #[test]
    fn test_unsigned_lte_mul() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<75>> = a * b;
        assert_eq!(*result, 75);
    }

    #[test]
    fn test_unsigned_lte_mul_lt() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<60>> = a * b;
        assert_eq!(*result, 60);
    }

    #[test]
    fn test_unsigned_lte_mul_eq() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<71>> = a * b;
        assert_eq!(*result, 70);
    }

    #[test]
    fn test_unsigned_lte_div() {
        let a = Refinement::<u8, unsigned::LTE<20>>::refine(20).unwrap();
        let b = Refinement::<u8, unsigned::LTE<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<20>> = a / b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_lte_div_lt() {
        let a = Refinement::<u8, unsigned::LTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::LT<5>>::refine(4).unwrap();
        let result: Refinement<u8, unsigned::LTE<25>> = a / b;
        assert_eq!(*result, 6);
    }

    #[test]
    fn test_unsigned_lte_div_eq() {
        let a = Refinement::<u8, unsigned::LTE<15>>::refine(14).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::LTE<3>> = a / b;
        assert_eq!(*result, 2);
    }

    #[test]
    fn test_unsigned_gt_add() {
        let a = Refinement::<u8, unsigned::GT<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GT<20>>::refine(21).unwrap();
        let result: Refinement<u8, unsigned::GT<31>> = a + b;
        assert_eq!(*result, 32);
    }

    #[test]
    fn test_unsigned_gt_add_gte() {
        let a = Refinement::<u8, unsigned::GT<15>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::GTE<25>>::refine(25).unwrap();
        let result: Refinement<u8, unsigned::GT<40>> = a + b;
        assert_eq!(*result, 41);
    }

    #[test]
    fn test_unsigned_gt_add_eq() {
        let a = Refinement::<u8, unsigned::GT<15>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GT<20>> = a + b;
        assert_eq!(*result, 21);
    }

    #[test]
    fn test_unsigned_gt_sub() {
        let a = Refinement::<u8, unsigned::GT<20>>::refine(21).unwrap();
        let b = Refinement::<u8, unsigned::GT<10>>::refine(11).unwrap();
        let result: Refinement<u8, unsigned::GT<9>> = a - b;
        assert_eq!(*result, 10);
    }

    #[test]
    fn test_unsigned_gt_sub_gte() {
        let a = Refinement::<u8, unsigned::GT<25>>::refine(26).unwrap();
        let b = Refinement::<u8, unsigned::GTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::GT<15>> = a - b;
        assert_eq!(*result, 16);
    }

    #[test]
    fn test_unsigned_gt_sub_eq() {
        let a = Refinement::<u8, unsigned::GT<15>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GT<10>> = a - b;
        assert_eq!(*result, 11);
    }

    #[test]
    fn test_unsigned_gt_mul() {
        let a = Refinement::<u8, unsigned::GT<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GT<5>>::refine(6).unwrap();
        let result: Refinement<u8, unsigned::GT<65>> = a * b;
        assert_eq!(*result, 66);
    }

    #[test]
    fn test_unsigned_gt_mul_gte() {
        let a = Refinement::<u8, unsigned::GT<10>>::refine(11).unwrap();
        let b = Refinement::<u8, unsigned::GTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GT<54>> = a * b;
        assert_eq!(*result, 55);
    }

    #[test]
    fn test_unsigned_gt_mul_eq() {
        let a = Refinement::<u8, unsigned::GT<15>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GT<79>> = a * b;
        assert_eq!(*result, 80);
    }

    #[test]
    fn test_unsigned_gt_div() {
        let a = Refinement::<u8, unsigned::GT<20>>::refine(21).unwrap();
        let b = Refinement::<u8, unsigned::GT<5>>::refine(6).unwrap();
        let result: Refinement<u8, unsigned::GTE<0>> = a / b;
        assert_eq!(*result, 3);
    }

    #[test]
    fn test_unsigned_gt_div_gte() {
        let a = Refinement::<u8, unsigned::GT<25>>::refine(26).unwrap();
        let b = Refinement::<u8, unsigned::GTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GTE<0>> = a / b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_gt_div_eq() {
        let a = Refinement::<u8, unsigned::GT<15>>::refine(16).unwrap();
        let b = Refinement::<u8, unsigned::Equals<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GT<2>> = a / b;
        assert_eq!(*result, 3);
    }

    #[test]
    fn test_unsigned_gte_add() {
        let a = Refinement::<u8, unsigned::GTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GTE<25>>::refine(25).unwrap();
        let result: Refinement<u8, unsigned::GTE<40>> = a + b;
        assert_eq!(*result, 40);
    }

    #[test]
    fn test_unsigned_gte_add_gt() {
        let a = Refinement::<u8, unsigned::GTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GT<25>>::refine(26).unwrap();
        let result: Refinement<u8, unsigned::GTE<41>> = a + b;
        assert_eq!(*result, 41);
    }

    #[test]
    fn test_unsigned_gte_sub() {
        let a = Refinement::<u8, unsigned::GTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::GTE<10>>::refine(10).unwrap();
        let result: Refinement<u8, unsigned::GTE<15>> = a - b;
        assert_eq!(*result, 15);
    }

    #[test]
    fn test_unsigned_gte_sub_gt() {
        let a = Refinement::<u8, unsigned::GTE<30>>::refine(30).unwrap();
        let b = Refinement::<u8, unsigned::GT<10>>::refine(11).unwrap();
        let result: Refinement<u8, unsigned::GTE<19>> = a - b;
        assert_eq!(*result, 19);
    }

    #[test]
    fn test_unsigned_gte_mul() {
        let a = Refinement::<u8, unsigned::GTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GTE<75>> = a * b;
        assert_eq!(*result, 75);
    }

    #[test]
    fn test_unsigned_gte_mul_gt() {
        let a = Refinement::<u8, unsigned::GTE<15>>::refine(15).unwrap();
        let b = Refinement::<u8, unsigned::GT<5>>::refine(6).unwrap();
        let result: Refinement<u8, unsigned::GTE<90>> = a * b;
        assert_eq!(*result, 90);
    }

    #[test]
    fn test_unsigned_gte_div() {
        let a = Refinement::<u8, unsigned::GTE<25>>::refine(25).unwrap();
        let b = Refinement::<u8, unsigned::GTE<5>>::refine(5).unwrap();
        let result: Refinement<u8, unsigned::GTE<0>> = a / b;
        assert_eq!(*result, 5);
    }

    #[test]
    fn test_unsigned_gte_div_gt() {
        let a = Refinement::<u8, unsigned::GTE<30>>::refine(30).unwrap();
        let b = Refinement::<u8, unsigned::GT<5>>::refine(6).unwrap();
        let result: Refinement<u8, unsigned::GTE<0>> = a / b;
        assert_eq!(*result, 5);
    }
}
