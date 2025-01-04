// src/calc.rs
use crate::parser::Parser;
use std::collections::HashMap;

pub struct Calculator {
    pub variables: HashMap<String, f64>,
    pub stack: Vec<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            variables: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, input: &str) {
        let parser = Parser::new(input);

        // Process numbers
        for number in parser.parse_numbers() {
            self.stack.push(number);
        }

        // Process variables
        for (key, value) in parser.parse_variables() {
            if let Ok(value) = value.parse::<f64>() {
                self.variables.insert(key, value);
            }
        }

        // Process coroutines
        for coroutine in parser.parse_coroutines() {
            println!("Executing coroutine: {:?}", coroutine);
        }

        self.display_state();
    }

    fn display_state(&self) {
        println!("Stack: {:?}", self.stack);
        println!("Variables: {:?}", self.variables);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let mut calc = Calculator::new();
        calc.execute("1 2 3 'x=42 { a b + }");
        assert_eq!(calc.stack, vec![1.0, 2.0, 3.0]);
        assert_eq!(calc.variables.get("x"), Some(&42.0));
    }
}

