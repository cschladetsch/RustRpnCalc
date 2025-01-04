// src/parser.rs
use std::collections::HashMap;

pub struct Parser {
    pub tokens: Vec<String>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        // Tokenize the input string
        let tokens = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        Parser { tokens }
    }

    pub fn parse_variables(&self) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        for token in &self.tokens {
            if token.starts_with("'") {
                // Extract variable name and value
                let parts: Vec<&str> = token[1..].split('=').collect();
                if parts.len() == 2 {
                    variables.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        variables
    }

    pub fn parse_coroutines(&self) -> Vec<Vec<String>> {
        let mut coroutines = Vec::new();
        let mut current = Vec::new();
        let mut in_coroutine = false;

        for token in &self.tokens {
            if token == "{" {
                in_coroutine = true;
                current.clear();
            } else if token == "}" {
                in_coroutine = false;
                coroutines.push(current.clone());
            } else if in_coroutine {
                current.push(token.clone());
            }
        }

        coroutines
    }

    pub fn parse_numbers(&self) -> Vec<f64> {
        self.tokens
            .iter()
            .filter_map(|token| token.parse::<f64>().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_variables() {
        let parser = Parser::new("'x=42 'y=hello");
        let vars = parser.parse_variables();
        assert_eq!(vars.get("x"), Some(&"42".to_string()));
        assert_eq!(vars.get("y"), Some(&"hello".to_string()));
    }

    #[test]
    fn test_parse_coroutines() {
        let parser = Parser::new("{ a b + } { c d * }");
        let coroutines = parser.parse_coroutines();
        assert_eq!(coroutines.len(), 2);
        assert_eq!(coroutines[0], vec!["a", "b", "+"]);
        assert_eq!(coroutines[1], vec!["c", "d", "*"]);
    }

    #[test]
    fn test_parse_numbers() {
        let parser = Parser::new("1 2 3.5 'x=42");
        let numbers = parser.parse_numbers();
        assert_eq!(numbers, vec![1.0, 2.0, 3.5]);
    }
}

