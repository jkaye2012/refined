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

* Has a name that is no more than 255 unicode characters long
* Has a size between 1 and 10 units

We could model that `Frobnicator` using refinement like so:

```rust
use refined::{Refinement, boundable::unsigned::{LessThan, ClosedInterval}};

type FrobnictorName = Refinement<u8, ClosedInterval<1, 10>;

type FrobnicatorSize = Refinement<String, LessThan<256>>;

struct Frobnicator {
  name: FrobnicatorName,
  size: FronicatorSize
}
```

## Optional features

### Serde support

### Implication

## Caveats

It's been argued that [names are not type safety](https://lexi-lambda.github.io/blog/2020/11/01/names-are-not-type-safety/).
Strictly speaking, the refinement types implemented in this crate are really refinement in name only, but Rust also doesn't
have the same number of "holes" that Haskell does as referred to in that article. I also feel that refinement of core types
is a pragmatic and performant approach relative to trying to work with [Church encoding](https://en.wikipedia.org/wiki/Church_encoding)
or other similar ideas.

So, while it's true that names might not be _as_ type safe as a "first principles" encoding of the invariant, I certainly
think they're safe enough!

