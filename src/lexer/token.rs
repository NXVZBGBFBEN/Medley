#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Times,
    Div,
    Frac,
    Plus,
    Minus,
    LParen,
    RParen,
    LBrace,
    RBrace,
}
