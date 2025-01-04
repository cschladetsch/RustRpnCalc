use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Coroutine(Vec<String>),
}

pub struct StackCalculator {
    data_stack: Vec<Value>,
    context_stack: Vec<HashMap<String, Value>>,
    variables: HashMap<String, Value>,
}

impl StackCalculator {
    pub fn new() -> Self {
        StackCalculator {
            data_stack: Vec::new(),
            context_stack: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.data_stack.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.data_stack.pop()
    }

    pub fn execute(&mut self, tokens: &[String]) {
        for token in tokens {
            match token.as_str() {
                "+" => self.binary_op(|a, b| Value::Number(a + b)),
                "-" => self.binary_op(|a, b| Value::Number(a - b)),
                "*" => self.binary_op(|a, b| Value::Number(a * b)),
                "/" => self.binary_op(|a, b| {
                    if b == 0.0 {
                        eprintln!("Error: Division by zero.");
                        Value::Number(a)
                    } else {
                        Value::Number(a / b)
                    }
                }),
                "{" => {
                    if let Some(end_index) = self.find_closing_brace(&tokens) {
                        let coroutine_tokens = tokens[1..end_index - 1].to_vec();
                        self.push(Value::Coroutine(coroutine_tokens));
                    } else {
                        eprintln!("Error: Unmatched opening brace.");
                    }
                }
                _ if token.starts_with("'") => {
                    let var_name = &token[1..];
                    if let Some(value) = self.pop() {
                        self.variables.insert(var_name.to_string(), value);
                    } else {
                        eprintln!("Error: No value to assign to variable '{}'.", var_name);
                    }
                }
                _ if self.variables.contains_key(token) => {
                    if let Some(value) = self.variables.get(token).cloned() {
                        self.push(value);
                    }
                }
                _ => match token.parse::<f64>() {
                    Ok(num) => self.push(Value::Number(num)),
                    Err(_) => eprintln!("Unknown token: {}", token),
                },
            }
            self.display_stacks();
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(f64, f64) -> Value,
    {
        if self.data_stack.len() < 2 {
            eprintln!("Error: operation requires two numbers.");
            return;
        }

        let b = self.pop().unwrap();
        let a = self.pop().unwrap();

        match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                self.push(op(a_val, b_val));
            }
            (a_val, b_val) => {
                eprintln!("Error: operation requires two numbers.");
                self.push(a_val);
                self.push(b_val);
            }
        }
    }

    fn find_closing_brace(&self, tokens: &[String]) -> Option<usize> {
        let mut depth = 0;
        for (i, token) in tokens.iter().enumerate() {
            match token.as_str() {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn execute_coroutine(&mut self) {
        while let Some(Value::Coroutine(tokens)) = self.pop() {
            self.context_stack.push(self.variables.clone());
            self.variables = HashMap::new();

            let mut local_calculator = StackCalculator {
                data_stack: self.data_stack.clone(),
                context_stack: self.context_stack.clone(),
                variables: self.variables.clone(),
            };

            local_calculator.execute(&tokens);

            self.data_stack = local_calculator.data_stack;
            self.context_stack.pop().map(|previous| {
                self.variables = previous;
            });
        }
    }

    fn display_stacks(&self) {
        println!("Data Stack:");
        if self.data_stack.is_empty() {
            println!("[empty]");
        } else {
            for (i, value) in self.data_stack.iter().rev().enumerate() {
                match value {
                    Value::Number(num) => println!("[{}] {:.2}", i, num),
                    Value::Coroutine(coroutine) => println!("[{}] {{ {} }}", i, coroutine.join(" ")),
                }
            }
        }
        println!("Context Stack:");
        if self.context_stack.is_empty() {
            println!("[empty]");
        } else {
            for (i, context) in self.context_stack.iter().rev().enumerate() {
                println!("[{}] {:?}", i, context);
            }
        }
    }
}

pub struct StackCalculatorFramework;

impl StackCalculatorFramework {
    pub fn create_calculator() -> StackCalculator {
        StackCalculator::new()
    }

    pub fn run_calculator_repl(calculator: &mut StackCalculator) {
        println!("Welcome to the Stack-Based Calculator Framework! Enter 'exit' to quit.");
        let stdin = io::stdin();
        loop {
            print!("> ");
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

