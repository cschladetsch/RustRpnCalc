use colored::Colorize; // This trait brings methods like `cyan()` into scope
pub mod operations;
pub mod stack;
pub mod value;

use crate::tokenizer::Token;
use crate::calculator::operations::Operations;
use crate::calculator::stack::Stack;

pub struct Calculator {
    stack: Stack,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            stack: Stack::new(),
        }
    }

    pub fn execute_command(&mut self, token: Token) {
        match token {
            Token::Number(value) => self.stack.push(value),
            Token::Plus => self.stack.binary_op(Operations::add),
            Token::Minus => self.stack.binary_op(Operations::subtract),
            Token::Multiply => self.stack.binary_op(Operations::multiply),
            Token::Divide => self.stack.binary_op(Operations::divide),
            Token::Dup => self.stack.dup(),
            Token::Coroutine(tokens) => {
                for token in tokens {
                    self.execute_command(token);
                }
            }
        }
    }

	pub fn display_stack(&self) {
		for (i, value) in self.stack.iter().rev().enumerate() {
			println!(
				"[{}] {}",
				self.stack.len() - i - 1,
				value.to_string().cyan()
			);
		}
	}
}

