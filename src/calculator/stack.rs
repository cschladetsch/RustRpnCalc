pub struct Stack {
    values: Vec<f64>, // Underlying storage for the stack
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: Vec::new() }
    }

    pub fn push(&mut self, value: f64) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<f64> {
        self.values.pop()
    }

    pub fn dup(&mut self) {
        if let Some(&top) = self.values.last() {
            self.values.push(top);
        }
    }

    pub fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(b), Some(a)) = (self.pop(), self.pop()) {
            self.push(op(a, b));
        }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        self.values.iter() // Return an iterator over the stack
    }

    pub fn len(&self) -> usize {
        self.values.len() // Return the number of elements in the stack
    }
}

