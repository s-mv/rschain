use super::ast::{Assignment, Block, Call, Expression, If};
use crate::lexer::lexer::{Keyword, Lexer, Symbol, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
    nested_call: bool,
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
            nested_call: false,
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

    // fn parse_statement(&mut self) -> Option<Expression> {
    //     let expr = self.parse_expression()?;
    //     Some(expr)
    // }

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

        self.nested_call = false;
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
        self.nested_call = true; // parens are compulsary inside assignment

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

        if self.next.token_type == TokenType::EOF {
            return None;
        }

        // dbg!(&self.next);

        if let TokenType::Symbol(symbol_type) = &self.next.token_type {
            match symbol_type {
                Symbol::Equals
                | Symbol::DoubleEquals
                | Symbol::NotEquals
                | Symbol::Star
                | Symbol::Slash
                | Symbol::GreaterThan
                | Symbol::GreaterEquals
                | Symbol::LessThan
                | Symbol::LessEquals
                | Symbol::Dot
                | Symbol::Comma => return None,
                _ => {}
            }
        }

        if let TokenType::Keyword(_) = self.next.token_type {
            return None;
        }

        let callee = callee.clone();
        self.advance();

        let mut arguments = Vec::new();
        let mut nested = self.nested_call;

        if nested {
            self.expect(TokenType::Symbol(Symbol::LParen)).ok()?;
            self.advance();
        } else if self.expect(TokenType::Symbol(Symbol::LParen)) == Ok(()) {
            self.advance();
            nested = true;
        }

        loop {
            self.nested_call = true;
            let argument = self.parse_expression()?;
            arguments.push(argument);

            if nested && self.current.token_type == TokenType::Symbol(Symbol::RParen) {
                self.advance();
                break;
            }

            if self.current.token_type != TokenType::Symbol(Symbol::Comma) {
                if nested {
                    return None;
                }
                break;
            } else {
                self.advance();
            }
        }

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
