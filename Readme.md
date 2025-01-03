# RPN Calculator with Coroutines

A Rust implementation of a Reverse Polish Notation (RPN) calculator that uses coroutines for operation handling. All numbers are handled as double-precision floating point values.

## Features

- RPN-style input with space-separated tokens
- Double-precision floating point arithmetic
- Stack-based operations
- Coroutine-based execution model
- Support for basic arithmetic: +, -, *, /
- Stack manipulation: dup, swap, drop
- Coroutine chaining with the # operator

## Quick Start

```bash
cargo build
cargo run
```

## Usage

Enter numbers and operators separated by spaces. Numbers are pushed onto the stack, and operators work on the stack values.

### Basic Operations

```
> 1 2 3    # Pushes 1.0, 2.0, 3.0 onto stack
Stack: [1.0, 2.0, 3.0]
> + +      # Add twice: first 2+3, then 1+5
Stack: [6.0]
```

### Commands

- Numbers: Push onto stack
- `+`: Add top two numbers
- `-`: Subtract top number from second top
- `*`: Multiply top two numbers
- `/`: Divide second top by top
- `dup`: Duplicate top number
- `swap`: Swap top two numbers
- `drop`: Remove top number
- `...`: Drop current coroutine
- `#`: Chain coroutines (squares the top number)
- `stack`: Show current stack contents
- `q`: Quit program

### Examples

```
# Basic arithmetic
> 10 5 -
Stack: [5.0]

# Multiple operations
> 2 3 4 + *
Stack: [14.0]   # 2 * (3 + 4)

# Stack manipulation
> 5 dup
Stack: [5.0, 5.0]
```

## Implementation Details

- Uses Rust's type system to ensure type safety
- All numbers are f64 (double-precision floating point)
- Implements coroutine-based operation handling
- Stack-based execution model matches RPN semantics
