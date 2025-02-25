use crate::{boundable::*, Predicate, UnsignedBoundable};

mod add;
mod sub;

/// A type that has a statically knowable unsigned maximum value.
pub trait UnsignedMax<T: UnsignedBoundable>: Predicate<T> {
    /// The maximum value.
    const UMAX: usize;
}

impl<T: UnsignedBoundable, const MAX: usize> UnsignedMax<T> for unsigned::LessThan<MAX> {
    const UMAX: usize = MAX - 1;
}

impl<T: UnsignedBoundable, const MAX: usize> UnsignedMax<T> for unsigned::LessThanEqual<MAX> {
    const UMAX: usize = MAX;
}

impl<T: UnsignedBoundable, const VAL: usize> UnsignedMax<T> for unsigned::Equals<VAL> {
    const UMAX: usize = VAL;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMax<T>
    for unsigned::ClosedInterval<MIN, MAX>
{
    const UMAX: usize = MAX;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMax<T>
    for unsigned::OpenClosedInterval<MIN, MAX>
{
    const UMAX: usize = MAX;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMax<T>
    for unsigned::OpenInterval<MIN, MAX>
{
    const UMAX: usize = MAX - 1;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMax<T>
    for unsigned::ClosedOpenInterval<MIN, MAX>
{
    const UMAX: usize = MAX - 1;
}

/// A type that has a statically knowable unsigned minimum value.
pub trait UnsignedMin<T: UnsignedBoundable>: Predicate<T> {
    /// The minimum value.
    const UMIN: usize;
}

impl<T: UnsignedBoundable, const MIN: usize> UnsignedMin<T> for unsigned::GreaterThan<MIN> {
    const UMIN: usize = MIN + 1;
}

impl<T: UnsignedBoundable, const MIN: usize> UnsignedMin<T> for unsigned::GreaterThanEqual<MIN> {
    const UMIN: usize = MIN;
}

impl<T: UnsignedBoundable, const VAL: usize> UnsignedMin<T> for unsigned::Equals<VAL> {
    const UMIN: usize = VAL;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMin<T>
    for unsigned::ClosedInterval<MIN, MAX>
{
    const UMIN: usize = MIN;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMin<T>
    for unsigned::OpenClosedInterval<MIN, MAX>
{
    const UMIN: usize = MIN + 1;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMin<T>
    for unsigned::OpenInterval<MIN, MAX>
{
    const UMIN: usize = MIN + 1;
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMin<T>
    for unsigned::ClosedOpenInterval<MIN, MAX>
{
    const UMIN: usize = MIN;
}

/// A type that has a statically knowable unsigned minimum value and maximum value.
pub trait UnsignedMinMax<T: UnsignedBoundable>:
    Predicate<T> + UnsignedMin<T> + UnsignedMax<T>
{
}

impl<T: UnsignedBoundable, const VAL: usize> UnsignedMinMax<T> for unsigned::Equals<VAL> {}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMinMax<T>
    for unsigned::ClosedInterval<MIN, MAX>
{
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMinMax<T>
    for unsigned::OpenClosedInterval<MIN, MAX>
{
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMinMax<T>
    for unsigned::OpenInterval<MIN, MAX>
{
}

impl<T: UnsignedBoundable, const MIN: usize, const MAX: usize> UnsignedMinMax<T>
    for unsigned::ClosedOpenInterval<MIN, MAX>
{
}
