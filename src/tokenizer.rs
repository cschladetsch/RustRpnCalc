#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Dup,
    Coroutine(Vec<Token>), // Represents a sequence of operations
}

pub struct Tokenizer {
    input: Vec<char>, // Process input as characters
    position: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer with input as characters
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Advances to the next token
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            '{' => {
                let mut tokens = Vec::new();
                while let Some(token) = self.next_token() {
                    if self.peek() == Some('}') {
                        self.position += 1; // Consume '}'
                        break;
                    }
                    tokens.push(token);
                }
                Some(Token::Coroutine(tokens))
            }
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            'd' if self.peek_word("dup") => {
                self.position += 2; // Consume 'up'
                Some(Token::Dup)
            }
            '0'..='9' | '.' => Some(self.parse_number(ch)),
            _ => {
                eprintln!("Error: Unrecognized token '{}'", ch);
                None
            }
        }
    }

    fn parse_number(&mut self, first_char: char) -> Token {
        let mut number = String::new();
        number.push(first_char);
        while let Some(ch) = self.peek() {
            if ch.is_numeric() || ch == '.' {
                number.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn peek_word(&self, word: &str) -> bool {
        self.input[self.position..]
            .iter()
            .collect::<String>()
            .starts_with(word)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
}

