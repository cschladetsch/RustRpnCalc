# RPN Calculator with Coroutines

A Rust implementation of a Reverse Polish Notation (RPN) calculator where each operation is implemented as a coroutine (a computation that can be suspended and resumed). 

## What is RPN?

RPN (Reverse Polish Notation) is a way of writing arithmetic expressions without parentheses. Instead of writing `3 + 4`, you write `3 4 +`. The numbers go onto a stack, and operators work with the numbers on the stack.

Examples:
- `3 4 +` equals 7 (push 3, push 4, add them)
- `5 2 *` equals 10 (push 5, push 2, multiply them)
- `10 5 2 + -` equals 3 (push 10, push 5, push 2, add 5&2 to get 7, subtract 7 from 10)

## What are Coroutines in this Context?

In this calculator, each operation (addition, multiplication, etc.) is a coroutine. A coroutine:
1. Takes the current stack as input
2. Performs its operation
3. Returns a new stack 
4. Can be suspended using '...' to return to a previous state

The coroutine approach means each operation:
- Maintains its own stack state
- Can be composed with other operations
- Can be suspended and resumed

## Building and Running

```bash
cargo build
cargo run
```

## Commands

All numbers are double-precision floating point.

Basic operations:
- Numbers: Push onto stack
- `+`: Add top two numbers
- `-`: Subtract top number from second top
- `*`: Multiply top two numbers
- `/`: Divide second top by top

Stack operations:
- `dup`: Duplicate top number
- `swap`: Swap top two numbers
- `drop`: Remove top number

Coroutine control:
- `...`: Drop current coroutine and return to previous state
- `stack`: Show current stack contents
- `q`: Quit program

## Example Usage

```
> 3 4 5     # Push three numbers
Stack: [3.0, 4.0, 5.0]

> +         # Add top two numbers (4 + 5)
Stack: [3.0, 9.0]

> +         # Add again (3 + 9)
Stack: [12.0]
```
