# Quit

This crate allows cleanly exiting a program using a custom exit code, without
the drawbacks of [`exit`]. Destructors will be called as usual, and the stack
will be unwound to the main function.

It is always required to attach [`main`] to the main function. Then,
[`with_code`] can be called from almost anywhere in the program. Restrictions
are noted in the documentation for that function.

[![GitHub Build Status](https://github.com/dylni/quit/workflows/build/badge.svg?branch=master)](https://github.com/dylni/quit/actions?query=branch%3Amaster)

## Usage

Add the following lines to your "Cargo.toml" file:

```toml
[dependencies]
quit = "1.1"
```

See the [documentation] for available functionality and examples.

## Rust version support

The minimum supported Rust toolchain version is currently Rust 1.32.0.

## License

Licensing terms are specified in [COPYRIGHT].

Unless you explicitly state otherwise, any contribution submitted for inclusion
in this crate, as defined in [LICENSE-APACHE], shall be licensed according to
[COPYRIGHT], without any additional terms or conditions.

[COPYRIGHT]: https://github.com/dylni/quit/blob/master/COPYRIGHT
[documentation]: https://docs.rs/quit
[`exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
[LICENSE-APACHE]: https://github.com/dylni/quit/blob/master/LICENSE-APACHE
[`main`]: https://docs.rs/quit/*/quit/attr.main.html
[`with_code`]: https://docs.rs/quit/*/quit/fn.with_code.html
