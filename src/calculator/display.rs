use crate::calculator::stack::Stack;
use colored::*;

pub struct Display;

impl Display {
    pub fn render(stack: &Stack) {
        for (i, value) in stack.values().iter().rev().enumerate() {
            println!("[{}] {}", stack.values().len() - i - 1, value.to_string().cyan());
        }
        println!("[0]");
    }
}

