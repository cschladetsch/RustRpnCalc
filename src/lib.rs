pub mod calculator;
use crate::calculator::{Calculator, operations::Operations};
use crate::calculator::stack::StackValue;

pub fn process_token(calculator: &mut Calculator, token: Token) {
    match token {
        Token::Number(value) => {
            calculator.push(value);
        }
        Token::Plus => {
            calculator.binary_op(Operations::add);
        }
        Token::Minus => {
            calculator.binary_op(Operations::subtract);
        }
        Token::Multiply => {
            calculator.binary_op(Operations::multiply);
        }
        Token::Divide => {
            calculator.binary_op(Operations::divide);
        }
        Token::Dup => {
            calculator.dup();
        }
        Token::Coroutine(tokens) => {
            calculator.push_coroutine(tokens);
        }
        Token::Exec => {
            if let Some(StackValue::Coroutine(tokens)) = calculator.pop() {
                for token in tokens {
                    calculator.execute_command(token);
                }
            } else {
                eprintln!("Error: Top of the stack is not a coroutine.");
            }
        }
    }
}

