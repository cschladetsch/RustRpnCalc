// src/calculator/stack.rs
use std::collections::HashMap;
use super::value::Value;

pub struct Stack {
    pub data_stack: Vec<Value>,
    pub context_stack: Vec<HashMap<String, Value>>,
    pub variables: HashMap<String, Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
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

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn push_context(&mut self) {
        self.context_stack.push(self.variables.clone());
        self.variables = HashMap::new();
    }

    pub fn pop_context(&mut self) {
        if let Some(previous) = self.context_stack.pop() {
            self.variables = previous;
        }
    }
}
