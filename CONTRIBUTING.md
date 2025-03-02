# Refined contribution guide

Thank you for considering a contribution to `refined`. This guide contains basic information on how
to easily get up and running, how to easily run and test any potential changes, and what is expected
of contributors in general.

## Development environment

Development of `refined` is driven by [Nix](https://nixos.org/). As a contributor, you don't need to
learn or even understand Nix to reap the benefits of its declarative environments, though you will
need to [install it](https://nixos.org/download/) and
[enable Flakes](https://nixos.wiki/wiki/Flakes) in order to use the supported tooling. **Any
development setup other than the Nix development environment is entirely unsupported.** You are
welcome to use whatever tools suit you personally, but I cannot support them.

Once Nix is installed and Flakes are enabled, dropping in to a fully-featured development
environment is easy:

```bash
nix develop
```

This will download all dependencies and configure an environment that you can use to run builds,
tests, end-to-end examples, etc. Examples of useful commands (all assuming that you're running
within the Nix development shell):

- Running a complete build: `cargo build --all-features`
- Running all tests: `cargo test --all-features`
- Building documentation: `cargo doc --all-features`
- Running the validation that will be run by GH actions: `nix flake check`

In addition to running within the development shell, any of these commands can be run "oneshot":

`nix develop --command bash -c "cargo test --all-features"` to run tests directly without activating
an interactive development shell.

## Documentation

Documentation is an extremely important element of library maintenance. Contributors are expected to
fully document their changes to at least the same level as existing `refined` functionality. PRs
without sufficient documentation **will not** be merged.

## Testing and validation

Once a pull request is opened, each push will be verified using
[GitHub Actions](https://docs.github.com/en/actions). To run the same validation locally that will
be run on the server, from the repository root simply run:

```bash
./verify.sh
```

PRs that do not pass all verfication will **never** be considered for merge.

## Pull request checklist

This informal checklist is meant to help new contributors understand the expectations that must be
met in order for their PRs to be merged. It does not have to be copied into the PR itself, though if
you'd like to, feel free!

- [ ] The change has either added or modified tests to verify its functionality
- [ ] The change is fully documented, including API docs and doctests
- [ ] All tests and verification are passing
- [ ] `CHANGELOG.md` has been updated to include relevant information in the `Unreleased` section
- [ ] The PR has a meaningful and sufficiently detailed description
