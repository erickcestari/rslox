# rslox
👻 | Rust version of the Lox interpreter

## Overview
This repository contains a Rust implementation of the Lox interpreter, a language introduced in the book "Crafting Interpreters" by Robert Nystrom. The interpreter supports various features of the Lox language, including variables, expressions, and control flow.

### Project Structure
```
.gitignore
Cargo.lock
Cargo.toml
examples/
    environment.lox
    logical.lox
    print.lox
    unary.lox
    variables.lox
src/
    environment.rs
    expression.rs
    interpreter.rs
    lib.rs
    literal.rs
    main.rs
    parser.rs
    scanner.rs
    statement.rs
    token_kind.rs
    token.rs
target/
tests/
    binary_op.rs
```

examples/: Contains example Lox programs.

src/: Contains the source code for the Lox interpreter.

tests/: Contains unit tests for the interpreter.

## Getting Started

### Prerequisites
* Rust (latest stable version)

### Building the Project
To build the project, run:
```
cargo build
```

### Running the Interpreter
You can run the interpreter with a Lox file:
```
cargo run examples/print.lox
```

Or start a REPL:
```
cargo run
```
### Running Tests
To run the tests, use:
```
cargo test
```

## Usage
You can find example Lox programs in the examples directory. For instance, to run the print.lox example:

```
cargo run -- examples/print.lox
```

### License
This project is licensed under the MIT License. See the LICENSE file for details.