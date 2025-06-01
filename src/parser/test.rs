use std::fs;

use crate::parser::ast::{Call, Expression};

use super::parser::Parser;

#[test]
fn ast() {
    let source = fs::read_to_string("examples/basic.rsc").expect("Failed to read test file");

    let mut parser = Parser::new(&source);

    let block = parser.parse();
    assert_eq!(block.statements.len(), 1);

    let statement = block.statements[0].clone();

    if let Expression::Call(Call { callee, arguments }) = statement {
        assert_eq!(callee, "print");
        assert_eq!(arguments.len(), 3);

        let arg2 = arguments[1].clone();

        if let Expression::Call(Call {callee, arguments}) = arg2 {
            assert_eq!(callee, "b");
           assert_eq!(arguments.len(), 1);
    }
    } else {
        panic!("Expected a Call expression");
    }
}
