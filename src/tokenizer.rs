// === src/tokenizer.rs ===
#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenBrace,
    CloseBrace,
    Variable(String),
    Assign(String),  // 'x for variable assignment
    Exec,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut num_str = String::new();
        let mut has_decimal = false;

        // Handle negative numbers
        if let Some('-') = self.peek() {
            num_str.push(self.advance()?);
        }

        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => {
                    num_str.push(self.advance()?);
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    num_str.push(self.advance()?);
                }
                _ => break,
            }
        }

        if num_str.is_empty() || num_str == "-" {
            None
        } else {
            match num_str.parse::<f64>() {
                Ok(num) => Some(Token::Number(num)),
                Err(_) => None,
            }
        }
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.advance()?);
            } else {
                break;
            }
        }

        if ident.is_empty() {
            None
        } else {
            match ident.as_str() {
                "exec" => Some(Token::Exec),
                _ => Some(Token::Variable(ident)),
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();
            
            if let Some(ch) = self.peek() {
                match ch {
                    '{' => {
                        self.advance();
                        tokens.push(Token::OpenBrace);
                    }
                    '}' => {
                        self.advance();
                        tokens.push(Token::CloseBrace);
                    }
                    '+' => {
                        self.advance();
                        tokens.push(Token::Plus);
                    }
                    '-' => {
                        // Check if it's a negative number or minus operator
                        if let Some(next_ch) = self.input.get(self.position + 1) {
                            if next_ch.is_digit(10) {
                                if let Some(token) = self.read_number() {
                                    tokens.push(token);
                                }
                            } else {
                                self.advance();
                                tokens.push(Token::Minus);
                            }
                        } else {
                            self.advance();
                            tokens.push(Token::Minus);
                        }
                    }
                    '*' => {
                        self.advance();
                        tokens.push(Token::Multiply);
                    }
                    '/' => {
                        self.advance();
                        tokens.push(Token::Divide);
                    }
                    '\'' => {
                        self.advance();
                        if let Some(token) = self.read_identifier() {
                            if let Token::Variable(name) = token {
                                tokens.push(Token::Assign(name));
                            }
                        }
                    }
                    '0'..='9' => {
                        if let Some(token) = self.read_number() {
                            tokens.push(token);
                        }
                    }
                    _ if !ch.is_whitespace() => {
                        if let Some(token) = self.read_identifier() {
                            tokens.push(token);
                        } else {
                            // Skip invalid characters
                            self.advance();
                        }
                    }
                    _ => {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }

        tokens
    }
}
