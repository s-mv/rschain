mod lexer;

use crate::lexer::Lexer;

fn main() {
    let source = "if 1 > 2 do a = 2 end";
    let mut lexer = Lexer::new(source);
    loop {
        let c = lexer.consume();
        c.print();
        
        if c == lexer::Token::EOF {
            break;
        }
    }
}
