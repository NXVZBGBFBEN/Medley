mod error;
mod token;

mod processor {
    mod latex {
        mod lexer;
        mod parser;
    }
}

trait Lexer {
    fn new(input: Vec<char>) -> Self;
    fn tokenize(&mut self) -> Result<token::Token, error::Error>;
}
