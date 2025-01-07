use crate::tokenizer::Token;

#[derive(Clone)]
pub enum StackValue {
    Number(f64),
    Coroutine(Vec<Token>),
}

pub struct Stack {
    values: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: Vec::new() }
    }

    pub fn push(&mut self, value: f64) {
        self.values.push(StackValue::Number(value));
    }

    pub fn push_coroutine(&mut self, tokens: Vec<Token>) {
        self.values.push(StackValue::Coroutine(tokens));
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.values.pop()
    }

    pub fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(StackValue::Number(b)), Some(StackValue::Number(a))) =
            (self.values.pop(), self.values.pop())
        {
            self.values.push(StackValue::Number(op(a, b)));
        } else {
            eprintln!("Error: Binary operation requires two numbers.");
        }
    }

    pub fn dup(&mut self) {
        if let Some(top) = self.values.last() {
            self.values.push(top.clone());
        }
    }

    pub fn iter(&self) -> std::slice::Iter<StackValue> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
