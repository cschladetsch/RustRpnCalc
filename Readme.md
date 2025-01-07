# Rust RPN Calculator

A modular Reverse Polish Notation (RPN) calculator implemented in Rust. This calculator supports basic arithmetic operations, variables, coroutines, and contextual stacks.

## Features

![image](/resources/Untitled3.png)

- Basic arithmetic operations (+, -, *, /)
- Stack-based operations with clear indexed output:
  ```
  λ 1 2 3
  Data Stack:
  [0] 1.00
  [1] 2.00
  [2] 3.00
  ```
- Variable assignment and retrieval:
  ```
  λ 42 'x
  Data Stack:
  [empty]
  λ x
  Data Stack:
  [0] 42.00
  ```
- Coroutine support with nested execution contexts
- Detailed debug mode for viewing context stacks:
  ```
  λ 5 'x { x 2 * } 'double exec
  Data Stack:
  [0] 10.00
  Context Stack:
  [0] x: Number(5)
  ```
- Colored output for better visualization
  - Blue for stack headers
  - Green for numbers
  - Yellow for coroutines
  - Purple for context stack (in debug mode)
- Modular architecture for easy extension

## Installation

Clone the repository and build using Cargo:

```bash
git clone git@github.com:cschladetsch/RustRpnCalc.git
cd RustRpnCalc
cargo build --release
```

## Usage

Run the calculator in normal mode:
```bash
cargo run
```

Run with debug mode to see context stacks:
```bash
cargo run -- --debug
```

### Demo

Here's a comprehensive demonstration of the calculator's features:

```
λ 1 2 3
Data Stack:
[0] 1.00
[1] 2.00
[2] 3.00

λ +
Data Stack:
[0] 1.00
[1] 5.00

λ *
Data Stack:
[0] 5.00

λ 10
Data Stack:
[0] 5.00
[1] 10.00

λ 'value
Data Stack:
[0] 5.00

λ value
Data Stack:
[0] 5.00
[1] 10.00

λ { 2 * } 'double
Data Stack:
[0] 5.00
[1] 10.00

λ double exec
Data Stack:
[0] 5.00
[1] 20.00

λ +
Data Stack:
[0] 25.00
```

When running in debug mode (`--debug`), you'll also see the context stack:

```
λ 5 'x { x 2 * } 'double exec
Data Stack:
[0] 10.00
Context Stack:
[0] x: Number(5)

λ double exec
Data Stack:
[0] 20.00
Context Stack:
[0] x: Number(5)
```

### Variables

Assign values to variables using the ' prefix:
```
λ 42 'x    # Assigns 42 to variable x
λ x        # Pushes value of x onto stack
```

### Coroutines

Define and execute coroutines:
```
λ { 2 3 + } 'add    # Define a coroutine
λ add exec          # Execute the coroutine
```

## Project Structure

```
src/
├── main.rs                 # Entry point
├── lib.rs                  # Library exports
├── calculator/
│   ├── mod.rs             # Calculator module
│   ├── stack.rs           # Stack operations
│   ├── operations.rs      # Mathematical operations
│   ├── display.rs         # Output formatting
│   └── value.rs           # Value types
└── repl/
    ├── mod.rs             # REPL module
    └── framework.rs       # REPL implementation
```

## Dependencies

- `colored` - Terminal coloring support

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details
