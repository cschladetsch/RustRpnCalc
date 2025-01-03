use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Clone)]
enum CoroResult {
    Stack(Vec<f64>),
}

#[derive(Clone)]
enum Operation {
    Push(f64),
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Drop,
    Compose(Box<Coroutine>, Box<Coroutine>),
}

#[derive(Clone)]
struct Coroutine {
    operation: Operation,
    stack: Vec<f64>,
}

impl Coroutine {
    fn new(operation: Operation) -> Self {
        Coroutine {
            operation,
            stack: Vec::new(),
        }
    }

    fn execute(&mut self, input_stack: Vec<f64>) -> CoroResult {
        self.stack = input_stack;
        
        match &self.operation {
            Operation::Push(val) => {
                self.stack.push(*val);
                CoroResult::Stack(self.stack.clone())
            },
            Operation::Add => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                    CoroResult::Stack(self.stack.clone())
                } else {
                    println!("Error: Stack underflow");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Sub => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                    CoroResult::Stack(self.stack.clone())
                } else {
                    println!("Error: Stack underflow");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Mul => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                    CoroResult::Stack(self.stack.clone())
                } else {
                    println!("Error: Stack underflow");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Div => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if b != 0.0 {
                        self.stack.push(a / b);
                        CoroResult::Stack(self.stack.clone())
                    } else {
                        println!("Error: Division by zero");
                        self.stack.push(a);
                        self.stack.push(b);
                        CoroResult::Stack(self.stack.clone())
                    }
                } else {
                    println!("Error: Stack underflow");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Dup => {
                if let Some(&val) = self.stack.last() {
                    self.stack.push(val);
                    CoroResult::Stack(self.stack.clone())
                } else {
                    println!("Error: Stack empty");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Swap => {
                if self.stack.len() >= 2 {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                    CoroResult::Stack(self.stack.clone())
                } else {
                    println!("Error: Stack underflow");
                    CoroResult::Stack(self.stack.clone())
                }
            },
            Operation::Drop => {
                self.stack.pop();
                CoroResult::Stack(self.stack.clone())
            },
            Operation::Compose(first, second) => {
                let mut first_coro = (**first).clone();
                match first_coro.execute(self.stack.clone()) {
                    CoroResult::Stack(new_stack) => {
                        let mut second_coro = (**second).clone();
                        second_coro.execute(new_stack)
                    },
                }
            },
        }
    }
}

struct RPNCalculator {
    coro_stack: VecDeque<Coroutine>,
    value_stack: Vec<f64>,
}

impl RPNCalculator {
    fn new() -> Self {
        RPNCalculator {
            coro_stack: VecDeque::new(),
            value_stack: Vec::new(),
        }
    }

    fn create_coro(&mut self, op: Operation) {
        let mut coro = Coroutine::new(op);
        match coro.execute(self.value_stack.clone()) {
            CoroResult::Stack(new_stack) => {
                self.value_stack = new_stack;
                self.coro_stack.push_back(coro);
            },
        }
    }

    fn drop_current(&mut self) {
        self.coro_stack.pop_back();
        if let Some(coro) = self.coro_stack.back_mut() {
            match coro.execute(Vec::new()) {
                CoroResult::Stack(new_stack) => {
                    self.value_stack = new_stack;
                },
            }
        } else {
            self.value_stack.clear();
        }
    }

    fn print_stack(&self) {
        println!("Stack: {:?}", self.value_stack);
    }
}

fn main() {
    let mut calc = RPNCalculator::new();
    println!("RPN Calculator with Coroutines");
    println!("Commands:");
    println!("  number - Push number onto stack");
    println!("  + - Add");
    println!("  - - Subtract");
    println!("  * - Multiply");
    println!("  / - Divide");
    println!("  dup - Duplicate top value");
    println!("  swap - Swap top two values");
    println!("  drop - Drop top value");
    println!("  ... - Drop current coroutine");
    println!("  # - Chain two coroutines (squares the top number)");
    println!("  stack - Show full stack");
    println!("  q - Quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "q" => break,
            "..." => calc.drop_current(),
            "stack" => calc.print_stack(),
            "#" => {
                let coro1 = Coroutine::new(Operation::Dup);
                let coro2 = Coroutine::new(Operation::Mul);
                calc.create_coro(Operation::Compose(
                    Box::new(coro1),
                    Box::new(coro2)
                ));
            },
            _ => {
                // Handle space-separated tokens
                for token in input.split_whitespace() {
                    match token {
                        "+" => calc.create_coro(Operation::Add),
                        "-" => calc.create_coro(Operation::Sub),
                        "*" => calc.create_coro(Operation::Mul),
                        "/" => calc.create_coro(Operation::Div),
                        "dup" => calc.create_coro(Operation::Dup),
                        "swap" => calc.create_coro(Operation::Swap),
                        "drop" => calc.create_coro(Operation::Drop),
                        num => {
                            if let Ok(n) = num.parse::<f64>() {
                                calc.create_coro(Operation::Push(n));
                            } else {
                                println!("Invalid input: {}", num);
                            }
                        }
                    }
                }
            }
        }

        calc.print_stack();
    }
}
