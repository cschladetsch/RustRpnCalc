pub mod tokenizer;
pub mod calculator;

use crate::tokenizer::Token;

pub struct Calculator {
    stack: Vec<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { stack: Vec::new() }
    }

	pub fn execute_command(&mut self, token: Token) {
		match token {
			Token::Number(value) => self.stack.push(value),
			Token::Plus => self.binary_op(|a, b| a + b),
			Token::Minus => self.binary_op(|a, b| a - b),
			Token::Multiply => self.binary_op(|a, b| a * b),
			Token::Divide => self.binary_op(|a, b| a / b),
			Token::Dup => {
				if let Some(&top) = self.stack.last() {
					self.stack.push(top);
				} else {
					eprintln!("Error: Stack is empty, cannot duplicate.");
				}
			}
			Token::Coroutine(tokens) => {
				for token in tokens {
					self.execute_command(token); // Execute each token in the coroutine
				}
			}
		}
	}

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(op(a, b));
        } else {
            eprintln!("Error: Not enough operands on the stack.");
        }
    }

    pub fn stack(&self) -> &[f64] {
        &self.stack
    }
}

