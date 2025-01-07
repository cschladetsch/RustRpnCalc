pub mod operations;
pub mod stack;
pub mod value;

use crate::tokenizer::Token;
use crate::calculator::operations::Operations;
use crate::calculator::stack::{Stack, StackValue};
use colored::Colorize;

pub struct Calculator {
    stack: Stack,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            stack: Stack::new(),
        }
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    pub fn push_coroutine(&mut self, tokens: Vec<Token>) {
        self.stack.push_coroutine(tokens);
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    pub fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        self.stack.binary_op(op);
    }

    pub fn dup(&mut self) {
        self.stack.dup();
    }

    pub fn display_stack(&self) {
        for (i, value) in self.stack.iter().rev().enumerate() {
            match value {
                StackValue::Number(num) => println!(
                    "[{}] {}",
                    self.stack.len() - i - 1,
                    num.to_string().cyan()
                ),
                StackValue::Coroutine(tokens) => {
                    let coroutine_str = tokens
                        .iter()
                        .map(|token| format!("{:?}", token))
                        .collect::<Vec<_>>()
                        .join(" ");
                    println!(
                        "[{}] {}",
                        self.stack.len() - i - 1,
                        format!("{{{}}}", coroutine_str).yellow()
                    );
                }
            }
        }
    }

    pub fn execute_command(&mut self, token: Token) {
        match token {
            Token::Number(value) => self.push(value),
            Token::Plus => self.binary_op(Operations::add),
            Token::Minus => self.binary_op(Operations::subtract),
            Token::Multiply => self.binary_op(Operations::multiply),
            Token::Divide => self.binary_op(Operations::divide),
            Token::Dup => self.dup(),
            Token::Coroutine(tokens) => self.push_coroutine(tokens),
            Token::Exec => {
                if let Some(StackValue::Coroutine(tokens)) = self.pop() {
                    for token in tokens {
                        self.execute_command(token);
                    }
                } else {
                    eprintln!("Error: Top of the stack is not a coroutine.");
                }
            }
        }
    }
}

