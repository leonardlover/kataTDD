# kataTDD
Short katas for Test-Driven-Development written in Rust.

## Dependencies

The only dependency is the programming language [Rust][rust]. It comes
with unit-testing functionality out-of-the-box via it's package manager, Cargo.

To install Rust on Unix systems (Linux, macOS) it's recommended to use `rustup`,
this is done by running the following command.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To install Rust on Windows check out the official [installation options][install].

After installation check that Cargo is installed by running

```bash
cargo --version
```

[rust]: https://www.rust-lang.org
[install]: https://forge.rust-lang.org/infra/other-installation-methods.html

## Project structure

The root directory contains many folders, each one is a distinct kata. A kata
is a Rust crate and it contains the actual executable in `src/main.rs` and it
contains tests in the `src/lib.rs` file. Each kata also has a `README.md` file
explaining what the kata is about and execution instructions.

## Test execution

First, `cd` into the desired kata, and then run

```bash
cargo test
```

This will execute all tests in the crate in parallel, to execute them one after
the other do this instead

```bash
cargo test -- --test-threads=1
```

This is not required in general, but it's good to know.

