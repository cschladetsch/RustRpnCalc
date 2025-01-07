// src/calculator/operations.rs
use super::stack::Stack;
use super::value::Value;

pub struct Operations;

impl Operations {
    pub fn binary_op<F>(stack: &mut Stack, op: F) -> Result<(), String>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        if stack.data_stack.len() < 2 {
            return Err("Operation requires two numbers".to_string());
        }

        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                stack.push(Value::Number(op(a_val, b_val)));
                Ok(())
            }
            (a_val, b_val) => {
                stack.push(a_val);
                stack.push(b_val);
                Err("Operation requires two numbers".to_string())
            }
        }
    }

    pub fn find_closing_brace(tokens: &[String]) -> Option<usize> {
        let mut depth = 0;
        for (i, token) in tokens.iter().enumerate() {
            match token.as_str() {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
