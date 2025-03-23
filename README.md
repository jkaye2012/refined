# Refined

![Crates.io Version](https://img.shields.io/crates/v/refined)
![Release Status](https://img.shields.io/github/actions/workflow/status/jkaye2012/refined/publish.yml)
![Crates.io License](https://img.shields.io/crates/l/refined)

Simple [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for Rust.

A basic introduction to the library is
[available on my blog](https://jordankaye.dev/posts/refined/).

For detailed information, please see the
[documentation on docs.rs](https://docs.rs/refined/latest/refined).

## Features

- [Serde integration](https://docs.rs/refined/latest/refined/#serde-support)
- [Logical implication for most predicates](https://docs.rs/refined/latest/refined/#implication)
- [Zero-overhead arithmetic](https://docs.rs/refined/latest/refined/#arithmetic)
- [Stateful refinement](https://docs.rs/refined/latest/refined/#stateful-refinement)
- [Run-time performance optimization](https://docs.rs/refined/latest/refined/#optimized)

## Example

```rust
use refined::{prelude::*, boolean::And, boundable::unsigned::{ClosedInterval, NonZero}, string::Trimmed};
use serde::{Serialize, Deserialize};
use serde_json::{json, from_value};

type MovieRating = Refinement<u8, ClosedInterval<1, 10>>;
type NonEmptyString = Refinement<String, And<Trimmed, NonZero>>;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
  title: NonEmptyString,
  director: NonEmptyString,
  rating: MovieRating
}

fn main() {
  let movie: Movie = from_value(json!({
    "title": "V for Vendetta",
    "director": "James McTeigue",
    "rating": 10
  })).unwrap();

  let malformed_movie: Movie = from_value(json!({
    "title": "Missing a director",
    "director": "",
    "rating": 1
  }));
  assert!(malformed_movie.is_err());
}
```

## Quickstart

The [basic usage example on docs.rs](https://docs.rs/refined/latest/refined/#basic-usage) is a
minimal example that should be easy to follow.

You can also use the [examples](https://github.com/jkaye2012/refined/tree/main/examples) to get
started. Each example is a complete cargo project of its own. They are meant to be run with
`cargo run` so that you can view their output and reference it against the code.

## FAQ

### What is the difference between `refined` and other similar libraries?

There are a number of pre-existing libraries with a similar aim to `refined`. While I make no
assertion that `refined` is in any way "superior" to these other libraries when it comes to the
functionality that they provide, I had three principles in mind during development that I believe
are not met by any other library:

- Simplicity: a design that anyone should be able to look at and understand. This immediately rules
  out any approach that relies upon proc macros
- Maintainability: it should be simple to keep the library up to date, add functionality, fix bugs,
  etc. Other developers should be able to contribute to the project without difficulty
- Extensbility: downstream consumers of the library should be able to easily add their own
  extensions without requiring contribution to the core `refined` library
- Ease of use: downstream consumers should be able to get up and running quickly and easy. For the
  most common use cases, functionality should be provided by the library directly

A direct comparison against some of the more popular options:

- [nutype](https://github.com/greyblake/nutype): entirely built around proc macros. I think this is
  a very cool project, and the proc macro approach _might_ be more powerful than what `refined` is
  able to achieve, but there is too much "magic" involved for my liking. I'd like my types to be
  easy to understand and modify
- [refined_type](https://github.com/tomoikey/refined_type) and
  [deranged](https://github.com/jhpratt/deranged): requires explicit implementations for _every_
  rule and type combination; for me, this is a significant impediment to both maintainability and
  extensibility. Macro-based "combiners" also fall outside of my goals for `refined`
- [prae](https://github.com/teenjuna/prae): more magical even than nutype. Again, a cool library,
  but I do not want to write a macro DSL to define my types
- [ranged_integers](https://github.com/disiamylborane/ranged_integers) and
  [light_ranged_integers](https://gitlab.com/MassiminoilTrace/light-ranged-integers): some really
  cool ideas around automatically choosing the most efficient storage given the user's required
  range and "op modes" that allow the user to specify how refinement should behave in the event of
  failure; however, as a result, these libraries support refinement only of integer types.
  `ranged_integers` also relies heavily on `unsafe` behavior for its core functionality, which is
  not something that I want for `refined` (where all `unsafe` behavior is opt-in behind feature
  flags)
- [refinement](https://github.com/2bdkid/refinement): a very simple library providing basic
  `Refinement` and `Predicate` functionality; doesn't expose `Predicate` implementations that
  downstream users can rely upon, so the goals here are not very similar to `refined`, which aims to
  be batteries included for the most common use cases

Ultimately, it comes down to a matter of style and taste. All of these libraries function well, and
the same end goal can be achieved using any of them. The real question for users is "Which style of
interaction with a library do you prefer?".

There is also the pending feature for
[pattern types](https://github.com/rust-lang/rust/issues/123646) built into the language. An
introduction of this proposal can be found in
[this gist](https://gist.github.com/joboet/0cecbce925ee2ad1ee3e5520cec81e30). Pattern types fulfill
many of the goals of `refined`; depending on the details of the feature once it's released, it's
possible that a subset of the use cases for `refined` should be deprecated in favor of pattern
types. It seems unlikely, however, that some of the more advanced features of `refined` will be
implemented with pattern types; `implication` and `arithmetic` in particular I feel are unlikely to
be enshrined directly in `std`.

That being said, it's difficult to predict exactly how and when pattern types will land, so this
will require more thought as the feature progresses. It's possible, for example, that the core of
`refined` could be re-written using pattern types while still providing the more advanced
functionality that the library already supports. This would allow downstream users to rely on a
consistent API in their code regardless of the details of how pattern types evolve over time.
