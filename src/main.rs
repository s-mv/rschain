mod lexer;

use crate::lexer::Lexer;

fn main() {
    let source = "if 1 >= 2 do
    a = 2
end else do
    a = 3
end
";
    let mut lexer = Lexer::new(source.chars().collect());
    loop {
        let c = lexer.consume();
        c.print();

        if c.token_type == lexer::TokenType::EOF {
            break;
        }
    }
}
