# ohce

`ohce` echoes the reverse of what you input it.

Check out original log in [kata-log][ohce-kata].

[ohce-kata]: https://kata-log.rocks/ohce-kata

## Execution

Run by executing the following command:

```bash
cargo run <NAME>
```

Where `<NAME>` should be replaced by your name, if name is left blank, `ohce`
will exit prematurely displaying an error message.

## Testing

Run unit tests by executing:

```bash
cargo test
```

This runs all tests in parallel. To test a particular subset of tests run:

```bash
cargo test <TEST_NAME>
```

Where `<TEST_NAME>` should be replaced by the test that wants to be runned.

## Kata structure

The source code is inside the [`src/`](./src/) directory, there are two files
in it:

- [`main.rs`](./src/main.rs) contains the binary crate executed via `cargo run`.
- [`lib.rs`](./src/lib.rs) contains the library crate with all functions and tests.

