#[derive(PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Equals,
    Star,
    Slash,
    GreaterThan,
    LessThan,
}

#[derive(PartialEq)]
pub enum Keyword {
    If,
    Do,
    End,
}

#[derive(PartialEq)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Symbol(Symbol),
    Keyword(Keyword),
    EOF,
}

pub struct Position {
    index: u64,
    line: u64,
    column: u64,
}

pub struct Lexer {
    input: Vec<char>,
    position: Position,
}

impl Symbol {
    pub fn as_char(&self) -> char {
        match self {
            Symbol::Plus => '+',
            Symbol::Minus => '-',
            Symbol::Equals => '=',
            Symbol::Star => '*',
            Symbol::Slash => '/',
            Symbol::GreaterThan => '>',
            Symbol::LessThan => '>',
        }
    }
}

impl Keyword {
    pub fn as_string(&self) -> String {
        match self {
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::End => "end",
        }
        .to_owned()
    }
}

impl Token {
    pub fn print(&self) {
        match self {
            Token::Identifier(value) => print!("Identifier {value}\n"),
            Token::Integer(value) => print!("Integer {value}\n"),
            Token::Float(value) => print!("Float {value}\n"),
            Token::Symbol(value) => print!("Symbol {}\n", value.as_char()),
            Token::EOF => print!("EOF\n"),
            Token::Keyword(value) => print!("Keyword {}\n", value.as_string()),
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
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
            self.position.line = 1;
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
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        let c = match self.current() {
            Some(c) => c,
            None => return Token::EOF,
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

        if let Some(symbol) = symbol {
            self.advance();
            return Token::Symbol(symbol);
        }

        if c.is_digit(10) {
            let new_index = self.return_end(|c| c.is_digit(10) || c == '.');
            let num_chars = &self.input[self.position.index as usize..new_index as usize];
            let num_str: String = num_chars.iter().collect();
            if num_str.contains('.') {
                let num: f64 = num_str.parse().expect("Failed to parse float.");
                self.update(new_index);

                return Token::Float(num);
            } else {
                let num: i64 = num_str.parse().expect("Failed to parse integer.");
                self.update(new_index);

                return Token::Integer(num);
            }
        }

        if c.is_alphabetic() || c == '_' {
            let new_index = self.return_end(|c| c.is_alphanumeric() || c == '_');
            let ident_chars = &self.input[self.position.index as usize..new_index as usize];
            let ident: String = ident_chars.iter().collect();
            self.update(new_index);

            let token = match ident.as_str() {
                "if" => Token::Keyword(Keyword::If),
                "do" => Token::Keyword(Keyword::Do),
                "end" => Token::Keyword(Keyword::End),
                _ => Token::Identifier(ident),
            };

            return token;
        }

        return Token::EOF;
    }
}
