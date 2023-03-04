# Quit

This crate allows cleanly exiting a program using a custom exit code, without
the drawbacks of [`process::exit`]. Destructors will be called as usual, and
the stack will be completely unwound.

It is always required to attach [`#[main]`][attribute] to the main function.
Then, [`with_code`] can be called from almost anywhere in the program.
Restrictions are noted in the documentation for that function.

[![GitHub Build Status](https://github.com/dylni/quit/workflows/build/badge.svg?branch=master)](https://github.com/dylni/quit/actions?query=branch%3Amaster)

## Usage

Add the following lines to your "Cargo.toml" file:

```toml
[dependencies]
quit = "2.0"
```

See the [documentation] for available functionality and examples.

## Rust version support

The minimum supported Rust toolchain version is currently Rust 1.64.0.

Minor version updates may increase this version requirement. However, the
previous two Rust releases will always be supported. If the minimum Rust
version must not be increased, use a tilde requirement to prevent updating this
crate's minor version:

```toml
[dependencies]
quit = "~2.0"
```

## License

Licensing terms are specified in [COPYRIGHT].

Unless you explicitly state otherwise, any contribution submitted for inclusion
in this crate, as defined in [LICENSE-APACHE], shall be licensed according to
[COPYRIGHT], without any additional terms or conditions.

[attribute]: https://docs.rs/quit/*/quit/attr.main.html
[COPYRIGHT]: https://github.com/dylni/quit/blob/master/COPYRIGHT
[documentation]: https://docs.rs/quit
[`process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
[LICENSE-APACHE]: https://github.com/dylni/quit/blob/master/LICENSE-APACHE
[`with_code`]: https://docs.rs/quit/*/quit/fn.with_code.html
