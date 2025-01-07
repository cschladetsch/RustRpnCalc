// === src/calculator/mod.rs ===
use super::tokenizer::{Token, Tokenizer};
pub mod display;
pub mod operations;
pub mod stack;
pub mod value;

use self::display::Display;
use self::operations::Operations;
use self::stack::Stack;
use self::value::Value;

pub struct Calculator {
    stack: Stack,
    display: Display,
}

impl Calculator {
    pub fn new(debug_mode: bool) -> Self {
        Calculator {
            stack: Stack::new(),
            display: Display::new(debug_mode),
        }
    }

    pub fn execute(&mut self, input: &str) {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        
        let mut coroutine_tokens = Vec::new();
        let mut in_coroutine = false;
        let mut changed = false;

        for token in tokens {
            match token {
                Token::OpenBrace => {
                    in_coroutine = true;
                }
                Token::CloseBrace => {
                    in_coroutine = false;
                    self.stack.push(Value::Coroutine(coroutine_tokens.clone()));
                    coroutine_tokens.clear();
                    changed = true;
                }
                token if in_coroutine => {
                    coroutine_tokens.push(token.to_string());
                }
                Token::Number(n) => {
                    self.stack.push(Value::Number(n));
                    changed = true;
                }
                Token::Plus => {
                    Operations::binary_op(&mut self.stack, |a, b| a + b)
                        .unwrap_or_else(|e| eprintln!("Error: {}", e));
                    changed = true;
                }
                Token::Minus => {
                    Operations::binary_op(&mut self.stack, |a, b| a - b)
                        .unwrap_or_else(|e| eprintln!("Error: {}", e));
                    changed = true;
                }
                Token::Multiply => {
                    Operations::binary_op(&mut self.stack, |a, b| a * b)
                        .unwrap_or_else(|e| eprintln!("Error: {}", e));
                    changed = true;
                }
                Token::Divide => {
                    Operations::binary_op(&mut self.stack, |a, b| {
                        if b == 0.0 {
                            eprintln!("Error: Division by zero.");
                            a
                        } else {
                            a / b
                        }
                    }).unwrap_or_else(|e| eprintln!("Error: {}", e));
                    changed = true;
                }
                Token::Exec => {
                    self.execute_coroutine();
                    changed = true;
                }
                Token::Assign(name) => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.set_variable(name, value);
                    } else {
                        eprintln!("Error: No value to assign to variable '{}'.", name);
                    }
                    changed = true;
                }
                Token::Variable(name) => {
                    if let Some(value) = self.stack.get_variable(&name) {
                        self.stack.push(value.clone());
                        changed = true;
                    } else {
                        eprintln!("Unknown variable: {}", name);
                    }
                }
            }
        }

        // Only display if something changed and we're not in a coroutine
        if changed && !in_coroutine {
            self.display.display_stacks(&self.stack);
        }
    }

    fn execute_coroutine(&mut self) {
        if let Some(Value::Coroutine(tokens)) = self.stack.pop() {
            self.stack.push_context();
            let input = tokens.join(" ");
            self.execute(&input);
            self.stack.pop_context();
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(n) => n.to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Multiply => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::OpenBrace => "{".to_string(),
            Token::CloseBrace => "}".to_string(),
            Token::Variable(name) => name.clone(),
            Token::Assign(name) => format!("'{}", name),
            Token::Exec => "exec".to_string(),
        }
    }
}
