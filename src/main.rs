use std::collections::{VecDeque, HashMap};
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
        }
    }
}

struct Calculator {
    coro_stack: VecDeque<Coroutine>,
    value_stack: Vec<f64>,
    variables: HashMap<String, f64>,
}

impl Calculator {
    fn new() -> Self {
        Calculator {
            coro_stack: VecDeque::new(),
            value_stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    fn create_coro(&mut self, op: Operation) {
        let mut coro = Coroutine::new(op);
        let CoroResult::Stack(new_stack) = coro.execute(self.value_stack.clone());
        self.value_stack = new_stack;
        self.coro_stack.push_back(coro);
    }

    fn drop_current(&mut self) {
        if !self.value_stack.is_empty() {
            self.value_stack.pop();
        }
    }

    fn set_var(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    fn get_var(&self, name: &str) -> Option<f64> {
        let result = self.variables.get(name).copied();
        result
    }

    fn show_state(&self) {
        println!("Stack: {:?}", self.value_stack);
        if !self.variables.is_empty() {
            println!("Variables: {}", 
                self.variables.iter()
                    .map(|(k,v)| format!("{}={:.1}", k, v))
                    .collect::<Vec<_>>()
                    .join(" "));
        }
    }
}

fn main() {
    let mut calc = Calculator::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().trim_start_matches('>').trim();
                if input.is_empty() { continue; }

                match input {
                    "q" => break,
                    "..." => calc.drop_current(),
                    input => {
                        let tokens: Vec<&str> = input.split_whitespace().collect();
                        let mut i = 0;
                        while i < tokens.len() {
                            // Handle variable assignment
                            if i + 2 < tokens.len() && tokens[i].starts_with('\'') && tokens[i+1] == "=" {
                                let var_name = tokens[i].trim_matches('\'');
                                let value_token = tokens[i+2];
                                
                                if let Ok(value) = value_token.parse::<f64>() {
                                    calc.set_var(var_name, value);
                                    i += 3;
                                    continue;
                                } else if let Some(val) = calc.get_var(value_token) {
                                    calc.set_var(var_name, val);
                                    i += 3;
                                    continue;
                                } else {
                                    println!("Invalid value for assignment: {}", value_token);
                                    i += 3;
                                    continue;
                                }
                            }

                            // Handle regular operators and numbers
                            match tokens[i] {
                                "+" => calc.create_coro(Operation::Add),
                                "-" => calc.create_coro(Operation::Sub),
                                "*" => calc.create_coro(Operation::Mul),
                                "/" => calc.create_coro(Operation::Div),
                                "dup" => calc.create_coro(Operation::Dup),
                                "swap" => calc.create_coro(Operation::Swap),
                                "drop" => calc.create_coro(Operation::Drop),
                                token => {
                                    // Skip handling token if it's part of a variable assignment
                                    if (i + 1 < tokens.len() && tokens[i+1] == "=") || token == "=" {
                                        i += 1;
                                        continue;
                                    }
                                    
                                    if let Some(val) = calc.get_var(token) {
                                        calc.create_coro(Operation::Push(val));
                                    } else if let Ok(num) = token.parse::<f64>() {
                                        calc.create_coro(Operation::Push(num));
                                    } else if !token.starts_with('\'') && token != "=" {
                                        println!("Invalid input or undefined variable: {}", token);
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                }
                calc.show_state();
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
