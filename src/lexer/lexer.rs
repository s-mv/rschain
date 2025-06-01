#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Plus,          // +
    Minus,         // -
    Equals,        // =
    DoubleEquals,  // ==
    NotEquals,     // !=
    Star,          // *
    Slash,         // /
    GreaterThan,   // >
    GreaterEquals, // >=
    LessThan,      // <
    LessEquals,    // <=
    LParen,        // (
    RParen,        // )
    Dot,           // .
    Comma,         // ,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    If,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Symbol(Symbol),
    Keyword(Keyword),
    EOF,
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub index: u64,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

pub struct Lexer {
    input: Vec<char>,
    position: Position,
}

impl Symbol {
    pub fn as_string(&self) -> String {
        match self {
            Symbol::Plus => "+".to_string(),
            Symbol::Minus => "-".to_string(),
            Symbol::Equals => "=".to_string(),
            Symbol::DoubleEquals => "==".to_string(),
            Symbol::NotEquals => "!=".to_string(),
            Symbol::Star => "*".to_string(),
            Symbol::Slash => "/".to_string(),
            Symbol::GreaterThan => ">".to_string(),
            Symbol::GreaterEquals => ">=".to_string(),
            Symbol::LessThan => "<".to_string(),
            Symbol::LessEquals => "<=".to_string(),
            Symbol::LParen => "(".to_string(),
            Symbol::RParen => ")".to_string(),
            Symbol::Dot => ".".to_string(),
            Symbol::Comma => ",".to_string(),
        }
    }
}

impl Keyword {
    pub fn as_string(&self) -> String {
        match self {
            Keyword::If => "if".to_string(),
            Keyword::Else => "else".to_string(),
        }
        .to_owned()
    }
}

impl TokenType {
    pub fn as_string(&self) -> String {
        match self {
            TokenType::Identifier(value) => value.to_string(),
            TokenType::Integer(value) => value.to_string(),
            TokenType::Float(value) => value.to_string(),
            TokenType::Symbol(value) => value.as_string(),
            TokenType::Keyword(value) => value.as_string(),
            TokenType::EOF => "EOF".to_string(),
        }
    }
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Lexer {
            input: input,
            position: Position {
                index: 0,
                line: 1,
                column: 1,
            },
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.position.index as usize).copied()
    }

    fn peek(&self) -> Option<char> {
        self.input.get((self.position.index + 1) as usize).copied()
    }

    fn lookahead(&self, step: u64) -> Option<char> {
        self.input
            .get((self.position.index + step) as usize)
            .copied()
    }

    fn advance(&mut self) {
        let current: char = self.current().unwrap_or('\0');

        if current == '\0' {
            return;
        }

        if current == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }

        self.position.index += 1;
    }

    fn update(&mut self, new_index: u64) {
        while self.position.index != new_index {
            self.advance();
        }
    }

    fn return_end<Function>(&mut self, condition: Function) -> u64
    where
        Function: Fn(char) -> bool,
    {
        let mut step = 0;
        while let Some(c) = self.lookahead(step) {
            if !condition(c) {
                break;
            }
            step += 1;
        }

        self.position.index + step
    }

    pub fn consume(&mut self) -> Token {
        while let Some('/') = self.current() {
            if let Some('/') = self.lookahead(1) {
                self.advance();
                self.advance();

                while let Some(c) = self.current() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }

        while let Some(c) = self.current() {
            if c == '\0' {
                let token = Token {
                    token_type: TokenType::EOF,
                    position: self.position.clone(),
                };
                return token;
            }
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let position = self.position.clone();

        let c = match self.current() {
            Some(c) => c,
            None => {
                return Token {
                    token_type: TokenType::EOF,
                    position,
                }
            }
        };

        let symbol = match c {
            '+' => Some(Symbol::Plus),
            '-' => Some(Symbol::Minus),
            '=' => Some(Symbol::Equals),
            '*' => Some(Symbol::Star),
            '/' => Some(Symbol::Slash),
            '>' => Some(Symbol::GreaterThan),
            '<' => Some(Symbol::LessThan),
            '.' => Some(Symbol::Dot),
            ',' => Some(Symbol::Comma),
            '(' => Some(Symbol::LParen),
            ')' => Some(Symbol::RParen),
            _ => None,
        };

        let two_char_symbol = match (c, self.peek()) {
            ('=', Some('=')) => Some(Symbol::DoubleEquals),
            ('!', Some('=')) => Some(Symbol::NotEquals),
            ('<', Some('=')) => Some(Symbol::LessEquals),
            ('>', Some('=')) => Some(Symbol::GreaterEquals),
            _ => None,
        };

        if let Some(symbol) = two_char_symbol {
            self.advance();
            self.advance();
            return Token {
                token_type: TokenType::Symbol(symbol),
                position,
            };
        }

        if let Some(symbol) = symbol {
            let token = Token {
                token_type: TokenType::Symbol(symbol),
                position,
            };
            self.advance();
            return token;
        }

        if c.is_digit(10) {
            let mut new_index = self.return_end(|c| c.is_digit(10) || c == '.');
            let num_chars = &self.input[self.position.index as usize..new_index as usize];
            let mut num_str: String = num_chars.iter().collect();

            if num_str.ends_with('.') {
                num_str.pop();
                new_index = new_index - 1;
            }

            let token = if num_str.contains('.') {
                let num: f64 = num_str.parse().expect("Failed to parse float.");
                Token {
                    token_type: TokenType::Float(num),
                    position,
                }
            } else {
                let num: i64 = num_str.parse().expect("Failed to parse integer.");
                Token {
                    token_type: TokenType::Integer(num),
                    position,
                }
            };
            self.update(new_index);
            return token;
        }

        if c.is_alphabetic() || c == '_' {
            let new_index = self.return_end(|c| c.is_alphanumeric() || c == '_');
            let ident_chars = &self.input[self.position.index as usize..new_index as usize];
            let ident: String = ident_chars.iter().collect();
            self.update(new_index);

            let token_type = match ident.as_str() {
                "if" => TokenType::Keyword(Keyword::If),
                "else" => TokenType::Keyword(Keyword::Else),
                _ => TokenType::Identifier(ident),
            };

            return Token {
                token_type,
                position,
            };
        }

        Token {
            token_type: TokenType::EOF,
            position,
        }
    }
}
