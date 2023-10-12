use crate::error::Error;
use crate::token::Token;
use crate::Lexer;
use std::iter::Peekable;
use std::vec::IntoIter;

struct LatexLexer {
    character_stream: Peekable<IntoIter<char>>,
}

impl Lexer for LatexLexer {
    fn new(input: Vec<char>) -> Self {
        LatexLexer {
            character_stream: input.into_iter().peekable(),
        }
    }
    fn tokenize(&mut self) -> Result<Token, Error> {
        todo!()
    }
}
