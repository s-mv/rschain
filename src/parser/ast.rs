use crate::lexer::lexer::Symbol;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assignment(Assignment),
    If(If),
    Call(Call),
    Infix(Infix),
    Identifier(String),
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub identifier: String,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct Infix {
    pub left: Box<Expression>,
    pub operator: InfixOp,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum InfixOp {
    Plus,          // +
    Minus,         // -
    DoubleEquals,  // ==
    NotEquals,     // !=
    Star,          // *
    Slash,         // /
    GreaterThan,   // >
    GreaterEquals, // >=
    LessThan,      // <
    LessEquals,    // <=
}

impl InfixOp {
    pub fn map_operator(symbol: Symbol) -> Option<InfixOp> {
        match symbol {
            Symbol::Plus => Some(InfixOp::Plus),
            Symbol::Minus => Some(InfixOp::Minus),
            Symbol::DoubleEquals => Some(InfixOp::DoubleEquals),
            Symbol::NotEquals => Some(InfixOp::NotEquals),
            Symbol::Star => Some(InfixOp::Star),
            Symbol::Slash => Some(InfixOp::Slash),
            Symbol::GreaterThan => Some(InfixOp::GreaterThan),
            Symbol::GreaterEquals => Some(InfixOp::GreaterEquals),
            Symbol::LessThan => Some(InfixOp::LessThan),
            Symbol::LessEquals => Some(InfixOp::LessEquals),
            _ => None,
        }
    }
}
