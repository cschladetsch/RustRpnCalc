use std::io::{self, Write};
use stack_calculator::calculator::Calculator;
use stack_calculator::tokenizer::Tokenizer;
use colored::*;

fn main() {
    let mut calculator = Calculator::new();
    println!("{}", "Rust RPN Calculator. Type 'exit' to quit.".bright_green());

    loop {
        print!("{}", "λ ".yellow());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        while let Some(token) = tokenizer.next_token() {
            calculator.execute_command(token);
        }

        calculator.display_stack();
    }

    println!("{}", "Goodbye!".bright_red());
}

