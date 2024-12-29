# Refined

Simple [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for Rust.

## Quickstart

Refinement is a useful concept when you know that certain invariants are meant to hold for values
of a given type.

A common motivating example is ensuring that user input meets domain requirements. Using refinement
types is one way to apply the concept of
[parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) declaratively.

Say for example that you are modeling a `Frobnicator` that you expect your users to be able to send to you,
and that each `Frobnicator` should meet the following domain requirements:

* Has a name that is no more than 100 unicode characters long
* Has a size between 1 and 10 units

We could model that `Frobnicator` using refinement like so:

```rust
use refined::{Refinement, RefinementError, boundable::unsigned::{LessThanEqual, ClosedInterval}};

type FrobnictorName = Refinement<String, ClosedInterval<1, 10>;

type FrobnicatorSize = Refinement<u8, LessThanEqual<100>>;

struct Frobnicator {
  name: FrobnicatorName,
  size: FronicatorSize
}

impl Frobnicator {
  pub fn new(name: String, size: u8) -> Result<Frobnicator, RefinementError> {
    let name = FrobnicatorName::refine(name)?;
    let size = FrobnicatorSize::refine(size)?;

    Self {
      name,
      size
    }
  }
}

assert!(Frobnicator::new("Good name".to_string(), 99).is_ok());
assert!(Frobnicator::new("Bad name, too long".to_string(), 99).is_err());
assert!(Frobnicator::new("Good name".to_string(), 123).is_err());
```

## Optional features

### Serde support

### Implication

## Caveats

It's been argued that [names are not type safety](https://lexi-lambda.github.io/blog/2020/11/01/names-are-not-type-safety/).
Strictly speaking, the refinement types implemented in this crate are really refinement
in name only, but refinement of core types is a pragmatic and performant approach relative to trying to work with
[Church encoding](https://en.wikipedia.org/wiki/Church_encoding) or other similar ideas. While it's true that names might
not be _as_ type safe as a "first principles" encoding of the invariant, simple refinement in this manner is certainly
safe enough for most use cases, especially when we're able to "plug the holes" in the type system through a library
such as this.

