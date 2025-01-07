pub struct Tokenizer {
    input: Vec<String>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.split_whitespace().map(|s| s.to_string()).collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let word = &self.input[self.position];
        self.position += 1;

        match word.as_str() {
            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            "*" => Some(Token::Multiply),
            "/" => Some(Token::Divide),
            "dup" => Some(Token::Dup),
            _ => {
                if let Ok(num) = word.parse::<f64>() {
                    Some(Token::Number(num))
                } else {
                    eprintln!("Error: Unrecognized token '{}'", word);
                    None
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Dup,
    Coroutine(Vec<Token>),
}

