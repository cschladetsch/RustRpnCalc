mod stack_calculator;

use stack_calculator::StackCalculatorFramework;

fn main() {
    let mut calculator = StackCalculatorFramework::create_calculator();
    StackCalculatorFramework::run_calculator_repl(&mut calculator);
}

