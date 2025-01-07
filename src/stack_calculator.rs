use std::collections::HashMap;
use std::io::{self, Write};
use colored::*;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Coroutine(Vec<String>),
}

pub struct StackCalculator {
    data_stack: Vec<Value>,
    context_stack: Vec<HashMap<String, Value>>,
    variables: HashMap<String, Value>,
    debug_mode: bool,
}

impl StackCalculator {
    pub fn new(debug_mode: bool) -> Self {
        StackCalculator {
            data_stack: Vec::new(),
            context_stack: Vec::new(),
            variables: HashMap::new(),
            debug_mode,
        }
    }

    // ... [previous methods remain the same until display_stacks]

    fn display_stacks(&self) {
        println!("{}", "Data Stack:".bold().blue());
        if self.data_stack.is_empty() {
            println!("{}", "[empty]".dimmed());
        } else {
            for (i, value) in self.data_stack.iter().enumerate() {
                match value {
                    Value::Number(num) => println!("[{}] {}", i, format!("{:.2}", num).green()),
                    Value::Coroutine(coroutine) => println!("[{}] {}", i, format!("{{ {} }}", coroutine.join(" ")).yellow()),
                }
            }
        }
        
        if self.debug_mode {
            println!("{}", "Context Stack:".bold().purple());
            if self.context_stack.is_empty() {
                println!("{}", "[empty]".dimmed());
            } else {
                for (i, context) in self.context_stack.iter().enumerate() {
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

pub struct StackCalculatorFramework;

impl StackCalculatorFramework {
    pub fn create_calculator(args: &[String]) -> StackCalculator {
        let debug_mode = args.contains(&String::from("--debug"));
        StackCalculator::new(debug_mode)
    }

    pub fn run_calculator_repl(calculator: &mut StackCalculator) {
        let stdin = io::stdin();
        loop {
            print!("{}", "λ ".yellow());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            if stdin.read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.eq_ignore_ascii_case("exit") {
                    break;
                }
                let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
                calculator.execute(&tokens);
            }
        }
    }
}
