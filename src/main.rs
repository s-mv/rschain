mod lexer;
mod parser;

use parser::parser::Parser;
use std::fs;

fn main() -> std::io::Result<()> {
    let source = fs::read_to_string("examples/test.rsc")?;

    let mut parser = Parser::new(&source);
    let program = parser.parse();

    dbg!(program);

    Ok(())
}
