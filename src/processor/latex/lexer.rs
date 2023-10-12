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

fn lex(input: Vec<char>) -> Result<Vec<Token>, Error> {
    let lexer = LatexLexer::new(input);
    let mut token_stream = Vec::<Token>::new();
    todo!()
}

/*
impl Lexer {
    //トークン化処理
    fn tokenize(&mut self) -> Result<Option<Token>, Error> {
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        let token = if self.curr().is_some() && Self::is_numeric(self.curr().unwrap()) {
            //数字を読み込んだ場合
            let mut number = vec![*self.curr().unwrap()];
            while self.peek().is_some() && Self::is_numeric(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
            Ok(String::from_iter(number)
                .parse::<f64>()
                .ok()
                .map(Token::Number))
        } else if self.curr() == Some(&'\\') {
            //コマンドを読み込んだ場合
            let mut command = vec![*self.curr().unwrap()];
            while self.peek().is_some() && self.peek().unwrap().is_ascii_alphabetic() {
                self.next();
                command.push(*self.curr().unwrap());
            }
            match String::from_iter(command).as_str() {
                "\\times" => Ok(Some(Token::Times)),
                "\\div" => Ok(Some(Token::Div)),
                "\\frac" => Ok(Some(Token::Frac)),
                d => Err(Error::InvalidCommandError(d.parse().unwrap())),
            }
        } else {
            //それ以外を読み込んだ場合
            match self.curr() {
                Some('+') => Ok(Some(Token::Plus)),
                Some('-') => Ok(Some(Token::Minus)),
                Some('(') => Ok(Some(Token::LParen)),
                Some(')') => Ok(Some(Token::RParen)),
                Some('{') => Ok(Some(Token::LBrace)),
                Some('}') => Ok(Some(Token::RBrace)),
                None => Ok(None),
                Some(d) => Err(Error::InvalidCharacterError(*d)),
            }
        };
        self.next();
        token
    }
    fn next(&mut self) {
        self.position += 1
    }
    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }
    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }
    fn is_numeric(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

//字句解析の実行(得たトークンを可変配列に入れて返す)
pub fn lex(input: Vec<char>) -> Result<Vec<Token>, Error> {
    let mut target = Lexer { input, position: 0 };
    let mut token = Vec::new();
    loop {
        match target.tokenize() {
            Ok(x) => {
                //終端検出
                let Some(y) = x else { break; };
                token.push(y);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(token)
}
 */