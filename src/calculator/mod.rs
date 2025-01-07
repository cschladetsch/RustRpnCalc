// src/calculator/mod.rs
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

    pub fn execute(&mut self, tokens: &[String]) {
        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            match token.as_str() {
                "+" => Operations::binary_op(&mut self.stack, |a, b| a + b)
                    .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                "-" => Operations::binary_op(&mut self.stack, |a, b| a - b)
                    .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                "*" => Operations::binary_op(&mut self.stack, |a, b| a * b)
                    .unwrap_or_else(|e| eprintln!("Error: {}", e)),
                "/" => Operations::binary_op(&mut self.stack, |a, b| {
                    if b == 0.0 {
                        eprintln!("Error: Division by zero.");
                        a
                    } else {
                        a / b
                    }
                }).unwrap_or_else(|e| eprintln!("Error: {}", e)),
                "{" => {
                    if let Some(end_index) = Operations::find_closing_brace(&tokens[index..]) {
                        let coroutine_tokens = tokens[index + 1..index + end_index - 1].to_vec();
                        self.stack.push(Value::Coroutine(coroutine_tokens));
                        index += end_index - 1;
                    } else {
                        eprintln!("Error: Unmatched opening brace.");
                    }
                }
                "exec" => self.execute_coroutine(),
                _ if token.starts_with("'") => {
                    let var_name = &token[1..];
                    if let Some(value) = self.stack.pop() {
                        self.stack.set_variable(var_name.to_string(), value);
                    } else {
                        eprintln!("Error: No value to assign to variable '{}'.", var_name);
                    }
                }
                _ => {
                    // First check if it's a variable
                    if let Some(value) = self.stack.get_variable(token) {
                        self.stack.push(value.clone());
                    } else {
                        // If not a variable, try to parse as number
                        match token.parse::<f64>() {
                            Ok(num) => self.stack.push(Value::Number(num)),
                            Err(_) => eprintln!("Unknown token: {}", token),
                        }
                    }
                }
            }
            self.display.display_stacks(&self.stack);
            index += 1;
        }
    }

    fn execute_coroutine(&mut self) {
        if let Some(Value::Coroutine(tokens)) = self.stack.pop() {
            self.stack.push_context();
            self.execute(&tokens);
            self.stack.pop_context();
        }
    }
}
