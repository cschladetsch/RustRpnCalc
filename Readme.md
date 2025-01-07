# Rust RPN Calculator

A modular Reverse Polish Notation (RPN) calculator implemented in Rust. This calculator supports basic arithmetic operations, variables, coroutines, and contextual stacks.

## Features

- Basic arithmetic operations (+, -, *, /)
- Stack-based operations
- Variable assignment and retrieval
- Coroutine support with nested execution contexts
- Debug mode for viewing context stacks
- Colored output for better visualization
- Modular architecture for easy extension

## Installation

Clone the repository and build using Cargo:

```bash
git clone <your-repository-url>
cd RustRpn
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

### Basic Operations

The calculator uses RPN (Reverse Polish Notation), where operators follow their operands:

```
λ 5 3 +    # Adds 5 and 3
λ 10 2 -   # Subtracts 2 from 10
λ 4 3 *    # Multiplies 4 and 3
λ 15 3 /   # Divides 15 by 3
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
