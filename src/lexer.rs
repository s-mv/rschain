#[derive(PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Equals,
    DoubleEquals,
    NotEquals,
    Star,
    Slash,
    GreaterThan,
    GreaterEquals,
    LessThan,
    LessEquals,
    Newline,
}

#[derive(PartialEq)]
pub enum Keyword {
    If,
    Do,
    End,
    Else,
}

#[derive(PartialEq)]
pub enum TokenType {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Symbol(Symbol),
    Keyword(Keyword),
    EOF,
}

#[derive(Clone)]
pub struct Position {
    index: u64,
    line: u64,
    column: u64,
}

pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

pub struct Lexer {
    input: Vec<char>,
    position: Position,
}

impl Symbol {
    pub fn as_string(&self) -> &str {
        match self {
            Symbol::Plus => "+",
            Symbol::Minus => "-",
            Symbol::Equals => "=",
            Symbol::DoubleEquals => "==",
            Symbol::NotEquals => "!=",
            Symbol::Star => "*",
            Symbol::Slash => "/",
            Symbol::GreaterThan => ">",
            Symbol::GreaterEquals => ">=",
            Symbol::LessThan => "<",
            Symbol::LessEquals => "<=",
            Symbol::Newline => "\\n",
        }
    }
}

impl Keyword {
    pub fn as_string(&self) -> String {
        match self {
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::End => "end",
            Keyword::Else => "else",
        }
        .to_owned()
    }
}

impl Token {
    pub fn print(&self) {
        match &self.token_type {
            TokenType::Identifier(value) => print!("Identifier -> {value}\n"),
            TokenType::Integer(value) => print!("Integer    -> {value}\n"),
            TokenType::Float(value) => print!("Float      -> {value}\n"),
            TokenType::Symbol(value) => print!("Symbol     -> {}\n", value.as_string()),
            TokenType::Keyword(value) => print!("Keyword    -> {}\n", value.as_string()),
            TokenType::EOF => print!("EOF        -> \\_(:D)_/\n"),
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
            if c == '\n' {
                let token = Token {
                    token_type: TokenType::Symbol(Symbol::Newline),
                    position: self.position.clone(),
                };
                self.advance();
                return token;
            }
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let c = match self.current() {
            Some(c) => c,
            None => {
                return Token {
                    token_type: TokenType::EOF,
                    position: self.position.clone(),
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
            self.advance(); // first char
            self.advance(); // second char
            return Token {
                token_type: TokenType::Symbol(symbol),
                position: self.position.clone(),
            };
        }

        if let Some(symbol) = symbol {
            let token = Token {
                token_type: TokenType::Symbol(symbol),
                position: self.position.clone(),
            };
            self.advance();
            return token;
        }

        if c.is_digit(10) {
            let new_index = self.return_end(|c| c.is_digit(10) || c == '.');
            let num_chars = &self.input[self.position.index as usize..new_index as usize];
            let num_str: String = num_chars.iter().collect();
            let token = if num_str.contains('.') {
                let num: f64 = num_str.parse().expect("Failed to parse float.");
                Token {
                    token_type: TokenType::Float(num),
                    position: self.position.clone(),
                }
            } else {
                let num: i64 = num_str.parse().expect("Failed to parse integer.");
                Token {
                    token_type: TokenType::Integer(num),
                    position: self.position.clone(),
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
                "do" => TokenType::Keyword(Keyword::Do),
                "end" => TokenType::Keyword(Keyword::End),
                "else" => TokenType::Keyword(Keyword::Else),
                _ => TokenType::Identifier(ident),
            };

            return Token {
                token_type,
                position: self.position.clone(),
            };
        }

        Token {
            token_type: TokenType::EOF,
            position: self.position.clone(),
        }
    }
}
