// src/calculator/display.rs
use colored::*;
use super::stack::Stack;
use super::value::Value;

pub struct Display {
    debug_mode: bool,
}

impl Display {
    pub fn new(debug_mode: bool) -> Self {
        Display { debug_mode }
    }

    pub fn display_stacks(&self, stack: &Stack) {
        println!("{}", "Data Stack:".bold().blue());
        if stack.data_stack.is_empty() {
            println!("{}", "[empty]".dimmed());
        } else {
            for (i, value) in stack.data_stack.iter().enumerate() {
                match value {
                    Value::Number(num) => println!("[{}] {}", i, format!("{:.2}", num).green()),
                    Value::Coroutine(coroutine) => println!("[{}] {}", i, format!("{{ {} }}", coroutine.join(" ")).yellow()),
                }
            }
        }
        
        if self.debug_mode {
            println!("{}", "Context Stack:".bold().purple());
            if stack.context_stack.is_empty() {
                println!("{}", "[empty]".dimmed());
            } else {
                for (i, context) in stack.context_stack.iter().enumerate() {
                    let formatted_context: String = context
                        .iter()
                        .map(|(k, v)| format!("{}: {:?}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!("[{}] {}", i, formatted_context.dimmed());
                }
            }
        }
    }
}
