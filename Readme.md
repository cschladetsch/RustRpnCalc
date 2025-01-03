# RPN Coroutine Calculator

A Rust implementation of a Reverse Polish Notation (RPN) calculator that uses coroutines for operation handling and supports variable assignment.

## Features

- RPN-style calculation
- Variable assignment and usage
- Basic arithmetic operations (+, -, *, /)
- Stack manipulation (dup, swap, drop)
- All numbers are handled as double precision floating point

## Usage

### Building and Running
```bash
cargo build
cargo run
```

Or use the provided `r` script:
```bash
./r
```

### Basic Operations

Enter numbers and operators in RPN format:
```
> 2 3 +     # Push 2, push 3, add them
Stack: [5.0]

> 10 2 *    # Push 10, push 2, multiply them
Stack: [20.0]
```

### Variables

Assign and use variables:
```
> 2 'a =    # Store 2 in variable 'a'
> 3 'b =    # Store 3 in variable 'b'
> a b +     # Add the values of a and b
Stack: [5.0]
```

### Stack Operations
```
> 2 dup     # Duplicate top value
Stack: [2.0, 2.0]

> 2 3 swap  # Swap top two values
Stack: [3.0, 2.0]

> drop      # Remove top value
Stack: [2.0]
```

### Commands
- Numbers: Push onto stack
- `+`, `-`, `*`, `/`: Arithmetic operations
- `dup`: Duplicate top value
- `swap`: Swap top two values
- `drop`: Remove top value
- `'name = value`: Assign value to variable
- `name`: Push variable's value onto stack
- `q`: Quit program

### Coroutine Implementation

Each operation is implemented as a coroutine that:
1. Takes the current stack as input
2. Performs its operation
3. Returns a new stack state
4. Can be suspended or dropped using '...'
