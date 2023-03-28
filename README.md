# Clp-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

coincbc-sys crate is a *-sys crate. The package provides Low-level bindings to the [Cbc] library.

By this package, you don't need to worry about installing Cbc in the system, and it's a package for **all platforms**.

Cbc (Coin-or linear programming) is an open-source linear programming solver. It is primarily meant to be used as a callable library, but a basic, stand-alone executable version is also available.

## Usage
Just add the following to your `Cargo.toml`:

```toml
[dependencies]
coincbc-sys = "0.2"
```

## Library Linking
if you want to know the detail about how it compile or link the Cbc, please see [Cbc-src].

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[Cbc]: https://github.com/coin-or/Cbc
[Cbc-src]: https://github.com/Maroon502/cbc-src

[documentation-img]: https://docs.rs/coincbc-sys/badge.svg
[documentation-url]: https://docs.rs/coincbc-sys
[package-img]: https://img.shields.io/crates/v/coincbc-sys.svg
[package-url]: https://crates.io/crates/coincbc-sys
[license-img]: https://img.shields.io/crates/l/coincbc-sys.svg
[license-url]: https://github.com/Maroon502/coincbc-sys/blob/master/LICENSE.md