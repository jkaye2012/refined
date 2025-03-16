use crate::{boundable::*, Predicate, SignedBoundable, UnsignedBoundable};
use alloc::string::String;

mod add;
mod div;
mod mul;
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

/// A type that has a statically knowable signed maximum value.
pub trait SignedMax<T: SignedBoundable>: Predicate<T> {
    /// The maximum value.
    const UMAX: isize;
}

impl<T: SignedBoundable, const MAX: isize> SignedMax<T> for signed::LessThan<MAX> {
    const UMAX: isize = MAX - 1;
}

impl<T: SignedBoundable, const MAX: isize> SignedMax<T> for signed::LessThanEqual<MAX> {
    const UMAX: isize = MAX;
}

impl<T: SignedBoundable, const VAL: isize> SignedMax<T> for signed::Equals<VAL> {
    const UMAX: isize = VAL;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMax<T>
    for signed::ClosedInterval<MIN, MAX>
{
    const UMAX: isize = MAX;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMax<T>
    for signed::OpenClosedInterval<MIN, MAX>
{
    const UMAX: isize = MAX;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMax<T>
    for signed::OpenInterval<MIN, MAX>
{
    const UMAX: isize = MAX - 1;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMax<T>
    for signed::ClosedOpenInterval<MIN, MAX>
{
    const UMAX: isize = MAX - 1;
}

/// A type that has a statically knowable signed minimum value.
pub trait SignedMin<T: SignedBoundable>: Predicate<T> {
    /// The minimum value.
    const UMIN: isize;
}

impl<T: SignedBoundable, const MIN: isize> SignedMin<T> for signed::GreaterThan<MIN> {
    const UMIN: isize = MIN + 1;
}

impl<T: SignedBoundable, const MIN: isize> SignedMin<T> for signed::GreaterThanEqual<MIN> {
    const UMIN: isize = MIN;
}

impl<T: SignedBoundable, const VAL: isize> SignedMin<T> for signed::Equals<VAL> {
    const UMIN: isize = VAL;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMin<T>
    for signed::ClosedInterval<MIN, MAX>
{
    const UMIN: isize = MIN;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMin<T>
    for signed::OpenClosedInterval<MIN, MAX>
{
    const UMIN: isize = MIN + 1;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMin<T>
    for signed::OpenInterval<MIN, MAX>
{
    const UMIN: isize = MIN + 1;
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMin<T>
    for signed::ClosedOpenInterval<MIN, MAX>
{
    const UMIN: isize = MIN;
}

/// A type that has a statically knowable signed minimum value and maximum value.
pub trait SignedMinMax<T: SignedBoundable>: Predicate<T> + SignedMin<T> + SignedMax<T> {}

impl<T: SignedBoundable, const VAL: isize> SignedMinMax<T> for signed::Equals<VAL> {}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMinMax<T>
    for signed::ClosedInterval<MIN, MAX>
{
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMinMax<T>
    for signed::OpenClosedInterval<MIN, MAX>
{
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMinMax<T>
    for signed::OpenInterval<MIN, MAX>
{
}

impl<T: SignedBoundable, const MIN: isize, const MAX: isize> SignedMinMax<T>
    for signed::ClosedOpenInterval<MIN, MAX>
{
}

pub const fn elem_min(a: isize, b: isize) -> isize {
    if a <= b {
        a
    } else {
        b
    }
}

/// Calculates the minimum bounds for an interval over multiplication.
pub const fn min_mul(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    elem_min(
        xmin * ymin,
        elem_min(xmin * ymax, elem_min(xmax * ymin, xmax * ymax)),
    )
}

/// Calculates the minimum bounds for an interval over division.
pub const fn min_div(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    elem_min(
        xmin / ymin,
        elem_min(xmin / ymax, elem_min(xmax / ymin, xmax / ymax)),
    )
}

pub const fn elem_max(a: isize, b: isize) -> isize {
    if a >= b {
        a
    } else {
        b
    }
}

/// Calculates the maximum bounds for an interval over multiplication.
pub const fn max_mul(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    elem_max(
        xmin * ymin,
        elem_max(xmin * ymax, elem_max(xmax * ymin, xmax * ymax)),
    )
}

/// Calculates the maximum bounds for an interval over division.
pub const fn max_div(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    elem_max(
        xmin / ymin,
        elem_max(xmin / ymax, elem_max(xmax / ymin, xmax / ymax)),
    )
}
