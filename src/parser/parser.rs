use super::ast::{Assignment, Block, Call, Expression, If};
use crate::lexer::lexer::{Keyword, Lexer, Symbol, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input.chars().collect());
        let current = lexer.consume();
        let next = lexer.consume();
        Parser {
            lexer,
            current,
            next,
        }
    }

    fn advance(&mut self) {
        self.current = self.next.clone();
        self.next = self.lexer.consume();
    }

    fn expect(&mut self, expected_type: TokenType) -> Result<(), String> {
        if self.current.token_type == expected_type {
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected_type.as_string(),
                self.current.token_type.as_string()
            ))
        }
    }

    fn expect_next(&mut self, expected_type: TokenType) -> Result<(), String> {
        if self.next.token_type == expected_type {
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected_type.as_string(),
                self.next.token_type.as_string()
            ))
        }
    }

    pub fn parse(&mut self) -> Block {
        let mut statements = Vec::new();

        while self.current.token_type != TokenType::EOF {
            if let Some(statement) = self.parse_expression() {
                statements.push(statement);
            }
        }

        let program = Block { statements };

        program
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_if()
            .or_else(|| self.parse_call())
            .or_else(|| self.parse_assignment())
            // .or_else(|| self.parse_infix())
            .or_else(|| self.parse_primary())
    }

    fn parse_if(&mut self) -> Option<Expression> {
        self.expect(TokenType::Keyword(Keyword::If)).ok()?;
        self.advance();

        let condition = self.parse_expression()?;

        self.expect(TokenType::Symbol(Symbol::Comma)).ok()?;
        self.advance();

        let then_block = self.parse_block();

        let else_block = if self.current.token_type == TokenType::Keyword(Keyword::Else) {
            self.advance();

            self.expect(TokenType::Symbol(Symbol::Comma)).ok()?;
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        Some(Expression::If(If {
            condition: Box::new(condition),
            then_block,
            else_block,
        }))
    }

    fn parse_assignment(&mut self) -> Option<Expression> {
        let TokenType::Identifier(identifier) = &self.current.token_type else {
            return None;
        };
        let identifier = identifier.clone();

        self.expect_next(TokenType::Symbol(Symbol::Equals)).ok()?;
        self.advance();
        self.advance();

        if let Some(value) = self.parse_expression() {
            return Some(Expression::Assignment(Assignment {
                identifier,
                value: Box::new(value),
            }));
        }

        None
    }

    fn parse_block(&mut self) -> Block {
        let mut statements = Vec::new();
        while self.current.token_type != TokenType::Symbol(Symbol::Dot)
            && self.current.token_type != TokenType::EOF
        {
            if let Some(statement) = self.parse_expression() {
                statements.push(statement);
            } else {
                self.advance();
                break;
            }
        }

        self.advance();

        Block { statements }
    }

    fn parse_call(&mut self) -> Option<Expression> {
        let TokenType::Identifier(callee) = &self.current.token_type else {
            return None;
        };

        let callee = callee.to_string();

        self.expect_next(TokenType::Symbol(Symbol::LParen)).ok()?;
        self.advance();
        self.advance();

        let mut arguments = Vec::new();

        while self.current.token_type != TokenType::Symbol(Symbol::RParen) {
            let argument = self.parse_expression()?;
            arguments.push(argument);

            if self.current.token_type == TokenType::Symbol(Symbol::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(TokenType::Symbol(Symbol::RParen)).ok()?;
        self.advance();

        Some(Expression::Call(Call { callee, arguments }))
    }

    // fn parse_infix(&mut self) -> Option<Expression> {
    //     self.parse_primary()
    // }

    fn parse_primary(&mut self) -> Option<Expression> {
        match &self.current.token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Some(Expression::Identifier(name))
            }
            TokenType::Integer(value) => {
                let value = *value;
                self.advance();
                Some(Expression::Integer(value))
            }
            TokenType::Float(value) => {
                let value = *value;
                self.advance();
                Some(Expression::Float(value))
            }
            _ => None,
        }
    }
}
