# Refined

![Crates.io Version](https://img.shields.io/crates/v/refined)
![Release Status](https://img.shields.io/github/actions/workflow/status/jkaye2012/refined/publish.yml)
![Crates.io License](https://img.shields.io/crates/l/refined)

Simple [refinement types](https://en.wikipedia.org/wiki/Refinement_type) for Rust.

A basic introduction to the library is
[available on my blog](https://jordankaye.dev/posts/refined/).

For detailed information, please see the
[documentation on docs.rs](https://docs.rs/refined/latest/refined).

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

A direct comparison against some of the more popular options:

- [nutype](https://github.com/greyblake/nutype): entirely built around proc macros. I think this is
  a very cool project, and the proc macro approach _might_ be more powerful than what `refined` is
  able to achieve, but there is too much "magic" involved for my liking. I'd like my types to be
  easy to understand and modify
- [refined_type](https://github.com/tomoikey/refined_type): requires explicit implementations for
  _every_ rule and type combination; for me, this is a significant impediment to both
  maintainability and extensibility. Macro-based "combiners" also fall outside of my goals for
  `refined`
- [prae](https://github.com/teenjuna/prae): more magical even than nutype. Again, a cool library,
  but I do not want to write a macro DSL to define my types

Ultimately, it comes down to a matter of style and taste. All of these libraries function well, and
the same end goal can be achieved using any of them. The real question for users is "Which style of
interaction with a library do you prefer?".
