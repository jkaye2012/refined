//! Demonstrates the effect of the `optimized` feature on the code generated by `refined`.

use refined::{boundable::unsigned::LessThan, prelude::*};

type Size = Refinement<usize, LessThan<12>>;

#[no_mangle]
fn month_name(s: &Size) -> &'static str {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    MONTHS[**s]
}

fn main() {
    let raw_index = core::env::args()
        .next_back()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let index = Size::refine(raw_index).unwrap();

    println!("{}", month_name(&index));
}
