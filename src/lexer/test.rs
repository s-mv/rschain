use std::fs;

use super::lexer::{Lexer, Symbol, TokenType};

#[test]
fn tokens() {
    let source = fs::read_to_string("examples/basic.rsc").expect("Failed to read test file");
    let mut lexer = Lexer::new(source.chars().collect());

    let expected = vec![
        TokenType::Identifier("print".to_string()),
        TokenType::Symbol(Symbol::LParen),
        TokenType::Identifier("a".to_string()),
        TokenType::Symbol(Symbol::Comma),
        TokenType::Identifier("b".to_string()),
        TokenType::Symbol(Symbol::LParen),
        TokenType::Identifier("c".to_string()),
        TokenType::Symbol(Symbol::RParen),
        TokenType::Symbol(Symbol::Comma),
        TokenType::Identifier("d".to_string()),
        TokenType::Symbol(Symbol::RParen),
        TokenType::EOF,
    ];

    for (i, expected_token) in expected.iter().enumerate() {
        let token = lexer.consume();
        assert_eq!(
            &token.token_type, expected_token,
            "Token mismatch at index {}: got {:?}, expected {:?}",
            i, token.token_type, expected_token
        );
    }
}
