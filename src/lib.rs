mod engine {
    mod internal;
}
mod processor {
    use crate::{error, token};

    trait Lexer {
        fn new(input: Vec<char>) -> Self;
        fn tokenize(&mut self) -> Result<token::Token, error::Error>;
    }
    mod latex {
        mod lexer;
        mod parser;
    }
}
mod config_manager;
mod error;
mod syntax_tree;
mod token;
