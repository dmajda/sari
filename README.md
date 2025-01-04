# Sari

Sari is a simple arithmetic expression evaluator written in Rust. It can be used
as a library or as a command-line tool.

I wrote Sari as part of learning Rust. In the future, I’d like to use it as a
playground for exploring parsers, compilers, and various related technologies.
It’s not very useful besides that.

## Installation

To use Sari as a library, add it as a dependency to your project:

```console
$ cargo add sari
```

To use it as a command-line tool, install it:

```console
$ cargo install sari
```

## Usage

### Library

To evaluate an expression, use the `sari::eval` function:

```rust
use sari::Error;

let result = sari::eval("(1 + 2) * 3");
assert_eq!(result, Ok(9));

let result = sari::eval("(1 + 2");
assert_eq!(result, Err(Error::new("expected `)`")));

let result = sari::eval("1 / 0");
assert_eq!(result, Err(Error::new("division by zero")));
```

For more details, see the [API documentation][sari-docs].

### Command line

To evaluate one or more expressions, pass them to the `sari` binary as
arguments:

```console
$ sari '(1 + 2) * 3'
9
$ sari '(1 + 2) * 3' '(4 + 5) * 6' '(7 + 8) * 9'
9
54
135
$ sari '(1 + 2'
expected `)`
$ sari '1 / 0'
division by zero
```

## Expressions

The expressions consist of integers combined using `+`, `-`, `*`, and `/` binary
operators (with the usual precedence and associativity) and grouped using
parentheses. These elements can be separated by whitespace.

The expressions use wrapping 32-bit signed arithmetic. Division by zero is an
error.

## License

This project is licensed under the [Apache License, Version 2.0](LICENSE-APACHE)
and the [MIT License](LICENSE-MIT). You may choose either license at your
option.

[sari-docs]: https://docs.rs/sari/latest/sari/
