// src/main.rs
use std::env;
use stack_calculator::repl::framework::Framework;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut calculator = Framework::create_calculator(&args);
    Framework::run_repl(&mut calculator);
}
