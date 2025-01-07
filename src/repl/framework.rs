// === src/repl/framework.rs ===
use std::io::{self, Write};
use colored::*;
use crate::calculator::Calculator;

pub struct Framework;

impl Framework {
    pub fn create_calculator(args: &[String]) -> Calculator {
        let debug_mode = args.contains(&String::from("--debug"));
        Calculator::new(debug_mode)
    }

    pub fn run_repl(calculator: &mut Calculator) {
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
                calculator.execute(input);
            }
        }
    }
}
